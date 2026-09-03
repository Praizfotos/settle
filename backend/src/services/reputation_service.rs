use crate::{domain::reputation::ReputationScore, errors::Result};
use tracing::info;

pub struct ReputationService {
    _db_pool: sqlx::PgPool,
}

impl ReputationService {
    pub fn new(db_pool: sqlx::PgPool) -> Self { Self { _db_pool: db_pool } }

    /// Compute a reputation score for an address by aggregating their
    /// settlement history from the read DB.
    pub async fn compute_for_address(&self, address: &str) -> Result<ReputationScore> {
        info!(address, "computing reputation");
        // TODO: query completed agreements, dispute records, volume from DB
        // then call ReputationScore::compute(...)
        todo!()
    }
}
