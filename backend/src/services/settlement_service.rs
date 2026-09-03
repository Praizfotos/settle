use crate::errors::Result;
use tracing::info;

pub struct SettlementService {}

impl SettlementService {
    pub fn new() -> Self { Self {} }

    /// Trigger a milestone release: calls escrow.release on-chain,
    /// then records the settlement event in the DB.
    pub async fn release_milestone(
        &self,
        agreement_id: &str,
        milestone_index: u32,
    ) -> Result<()> {
        info!(agreement_id, milestone_index, "releasing milestone");
        // TODO: call EscrowContract::release via StellarClient
        todo!()
    }

    /// Trigger a full refund after dispute or expiry.
    pub async fn refund(&self, agreement_id: &str, amount: i64) -> Result<()> {
        info!(agreement_id, amount, "processing refund");
        todo!()
    }
}
