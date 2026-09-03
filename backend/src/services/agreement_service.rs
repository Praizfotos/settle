//! Application service — orchestrates Agreement domain + Stellar adapter + DB.
//!
//! This is the single entry point for all agreement business operations.
//! Callers (HTTP handlers, indexer) MUST use this service rather than
//! touching the database or Stellar adapter directly.

use crate::{
    domain::agreement::{Agreement, AgreementState, CreateAgreementRequest},
    errors::Result,
};
use tracing::{info, warn};

pub struct AgreementService {
    // TODO: inject DB pool and StellarClient
}

impl AgreementService {
    pub fn new() -> Self {
        Self {}
    }

    /// Build and submit a create_agreement Soroban invocation.
    pub async fn create(&self, req: CreateAgreementRequest) -> Result<Agreement> {
        info!(title = %req.title, "creating agreement");
        // TODO:
        // 1. Validate business rules (amount > 0, parties differ, etc.)
        // 2. Build Soroban transaction via StellarClient
        // 3. Submit and await confirmation
        // 4. Persist the projection to PostgreSQL
        // 5. Return the new Agreement
        todo!("agreement creation not yet implemented")
    }

    /// Apply an AgreementFunded event from the indexer to the read model.
    pub async fn handle_funded_event(&self, on_chain_id: i64, amount: i64) -> Result<()> {
        info!(on_chain_id, amount, "handling AgreementFunded event");
        // TODO: update agreement state to Funded in DB
        Ok(())
    }

    /// Apply an AgreementCompleted event to the read model.
    pub async fn handle_completed_event(&self, on_chain_id: i64) -> Result<()> {
        info!(on_chain_id, "handling AgreementCompleted event");
        // TODO: update agreement state to Completed in DB
        Ok(())
    }
}
