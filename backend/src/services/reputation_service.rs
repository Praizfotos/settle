use crate::{domain::reputation::ReputationScore, errors::Result};
use tracing::info;

pub struct ReputationService {}

impl ReputationService {
    pub fn new() -> Self { Self {} }

    /// Compute a reputation score for an address by aggregating their
    /// settlement history from the read DB.
    pub async fn compute_for_address(&self, address: &str) -> Result<ReputationScore> {
        info!(address, "computing reputation");
        // TODO: query completed agreements, dispute records, volume from DB
        // then call ReputationScore::compute(...)
        todo!()
    }
}
