use std::sync::Arc;
use sqlx::PgPool;

use crate::stellar::client::StellarClient;
use crate::services::*;

/// Application state shared across all handlers and services.
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub stellar_client: Arc<StellarClient>,
    pub agreement_service: Arc<AgreementService>,
    pub settlement_service: Arc<SettlementService>,
    pub dispute_service: Arc<DisputeService>,
    pub reputation_service: Arc<ReputationService>,
}
