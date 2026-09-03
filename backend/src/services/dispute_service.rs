use crate::{domain::dispute::Dispute, errors::Result};
use tracing::info;

pub struct DisputeService {}

impl DisputeService {
    pub fn new() -> Self { Self {} }

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
}
