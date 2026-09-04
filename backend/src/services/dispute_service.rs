use crate::{domain::dispute::Dispute, errors::Result};
use tracing::info;

pub struct DisputeService {
    _db_pool: sqlx::PgPool,
}

impl DisputeService {
    pub fn new(db_pool: sqlx::PgPool) -> Self { Self { _db_pool: db_pool } }

    pub async fn open_dispute(
        &self,
        agreement_id: &str,
        opener: &str,
        amount: i64,
        reason: &str,
    ) -> Result<Dispute> {
        info!(agreement_id, opener, "opening dispute");
        // TODO: call dispute Soroban contract, persist, emit event
        todo!()
    }

    pub async fn resolve(
        &self,
        dispute_id: &str,
        release_to_provider: bool,
    ) -> Result<Dispute> {
        info!(dispute_id, release_to_provider, "resolving dispute");
        // TODO: submit on-chain resolution, update DB
        todo!()
    }

    pub async fn list_all(&self, _limit: i64, _offset: i64) -> Result<Vec<Dispute>> {
        Ok(vec![])
    }

    pub async fn get_by_chain_id(&self, _chain_id: &str) -> Result<Option<Dispute>> {
        Ok(None)
    }
}
