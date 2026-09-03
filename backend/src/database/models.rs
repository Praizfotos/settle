//! SQLx row structs — one-to-one mapping with database tables.
//! These are deliberately separate from domain types to keep the
//! mapping explicit and allow schema evolution independently.

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct AgreementRow {
    pub id:               Uuid,
    pub on_chain_id:      i64,
    pub client_address:   String,
    pub provider_address: String,
    pub token_address:    String,
    pub total_amount:     i64,
    pub escrowed_amount:  i64,
    pub released_amount:  i64,
    pub refunded_amount:  i64,
    pub title:            String,
    pub state:            String,
    pub created_at:       DateTime<Utc>,
    pub updated_at:       DateTime<Utc>,
    pub expires_at_ledger: Option<i64>,
    pub tx_hash:          Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MilestoneRow {
    pub id:               Uuid,
    pub agreement_id:     Uuid,
    pub on_chain_index:   i32,
    pub title:            String,
    pub amount:           i64,
    pub state:            String,
    pub created_at:       DateTime<Utc>,
    pub updated_at:       DateTime<Utc>,
    pub due_at_ledger:    Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DisputeRow {
    pub id:                    Uuid,
    pub agreement_id:          Uuid,
    pub on_chain_dispute_id:   i64,
    pub opener_address:        String,
    pub disputed_amount:       i64,
    pub reason:                String,
    pub state:                 String,
    pub resolution:            String,
    pub opened_at:             DateTime<Utc>,
    pub resolved_at:           Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SettlementEventRow {
    pub id:              Uuid,
    pub agreement_id:    Uuid,
    pub event_type:      String, // "release" | "refund" | "escrow_lock"
    pub amount:          i64,
    pub participant:     String,
    pub ledger:          i64,
    pub tx_hash:         String,
    pub created_at:      DateTime<Utc>,
}

/// Cursor tracking for the event indexer.
#[derive(Debug, sqlx::FromRow)]
pub struct IndexerCursorRow {
    pub id:                   i32,
    pub last_ingested_ledger: i64,
    pub updated_at:           DateTime<Utc>,
}
