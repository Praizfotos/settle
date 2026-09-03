//! Polls Stellar RPC for new contract events and feeds them to the decoder.
//!
//! The listener maintains a cursor (last ingested ledger sequence) in the
//! database so it can resume after restarts without missing events or
//! reprocessing history.

use std::sync::Arc;
use std::time::Duration;
use sqlx::PgPool;
use tokio::time::sleep;
use tracing::{debug, info, warn, error};

use crate::errors::{AppError, Result};
use crate::stellar::client::StellarClient;
use crate::indexer::decoder::{decode, RawEvent};
use crate::indexer::processor::process;

/// Configuration for the indexer listener.
pub struct ListenerConfig {
    pub stellar_client: Arc<StellarClient>,
    pub contract_address: String,
    pub poll_interval_ms: u64,
    pub batch_size: u32,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

/// Persistent cursor so the indexer can resume after a restart.
#[derive(Debug, Clone)]
pub struct IndexerCursor {
    pub last_ledger: i64,
}

/// Indexer listener that processes contract events
pub struct IndexerListener {
    config: ListenerConfig,
}

impl IndexerListener {
    pub fn new(config: ListenerConfig) -> Self {
        Self { config }
    }

    /// Run the indexer event loop.
    ///
    /// Polls Stellar RPC for new events starting from the persisted cursor,
    /// decodes each event, and hands it to the processor.
    pub async fn run(&mut self, db_pool: PgPool) -> Result<()> {
        info!(
            contract_address = %self.config.contract_address,
            poll_interval_ms = self.config.poll_interval_ms,
            "indexer listener starting"
        );

        let mut retry_count = 0;

        loop {
            match self.process_events(&db_pool).await {
                Ok(events_processed) => {
                    retry_count = 0; // Reset retry count on success
                    
                    if events_processed == 0 {
                        // No new events, wait before polling again
                        sleep(Duration::from_millis(self.config.poll_interval_ms)).await;
                    } else {
                        info!("Processed {} events", events_processed);
                    }
                }
                Err(e) => {
                    error!("Error processing events: {}", e);
                    retry_count += 1;
                    
                    if retry_count >= self.config.max_retries {
                        error!("Max retries ({}) exceeded, stopping indexer", self.config.max_retries);
                        return Err(e);
                    }
                    
                    warn!("Retrying in {}ms (attempt {} of {})", 
                          self.config.retry_delay_ms, retry_count, self.config.max_retries);
                    sleep(Duration::from_millis(self.config.retry_delay_ms)).await;
                }
            }
        }
    }

    async fn process_events(&self, db_pool: &PgPool) -> Result<u32> {
        // Load current cursor from database
        let cursor = self.load_cursor(db_pool).await?;
        
        // Get latest ledger from Stellar
        let latest_ledger = self.config.stellar_client.get_latest_ledger().await?;
        
        if cursor.last_ledger >= latest_ledger as i64 {
            // Already caught up
            return Ok(0);
        }

        let from_ledger = cursor.last_ledger + 1;
        let to_ledger = std::cmp::min(
            from_ledger + self.config.batch_size as i64 - 1,
            latest_ledger as i64
        );

        debug!("Fetching events from ledger {} to {}", from_ledger, to_ledger);

        // Fetch events from Stellar
        let raw_events = self.config.stellar_client.get_events(
            &self.config.contract_address,
            from_ledger as u32,
            to_ledger as u32,
        ).await?;

        let mut events_processed = 0;

        // Process each event
        for raw_event in raw_events {
            match decode(raw_event) {
                Ok(settle_event) => {
                    if let Err(e) = process(db_pool, settle_event).await {
                        error!("Failed to process event: {}", e);
                        // Continue processing other events
                        continue;
                    }
                    events_processed += 1;
                }
                Err(e) => {
                    warn!("Failed to decode event: {}", e);
                    // Continue with next event
                    continue;
                }
            }
        }

        // Update cursor
        self.update_cursor(db_pool, to_ledger).await?;

        Ok(events_processed)
    }

    async fn load_cursor(&self, db_pool: &PgPool) -> Result<IndexerCursor> {
        let row = sqlx::query_as!(
            IndexerCursor,
            "SELECT last_ledger FROM indexer_cursor ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.unwrap_or(IndexerCursor { last_ledger: 0 }))
    }

    async fn update_cursor(&self, db_pool: &PgPool, ledger: i64) -> Result<()> {
        sqlx::query!(
            "UPDATE indexer_cursor SET last_ledger = $1, last_processed_at = NOW() WHERE id = (SELECT id FROM indexer_cursor ORDER BY id DESC LIMIT 1)",
            ledger
        )
        .execute(db_pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
