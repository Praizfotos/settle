//! Repository traits — abstractions over SQLx queries.
//! Concrete implementations use sqlx::PgPool.

use crate::{
    database::models::{AgreementRow, DisputeRow, MilestoneRow, SettlementEventRow},
    errors::Result,
};
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// Agreement Repository
// ─────────────────────────────────────────────────────────────────────────────

pub struct AgreementRepository {
    // pub pool: sqlx::PgPool,
}

impl AgreementRepository {
    pub fn new() -> Self { Self {} }

    pub async fn find_by_on_chain_id(&self, on_chain_id: i64) -> Result<Option<AgreementRow>> {
        // TODO: SELECT * FROM agreements WHERE on_chain_id = $1
        todo!()
    }

    pub async fn insert(&self, row: &AgreementRow) -> Result<()> {
        // TODO: INSERT INTO agreements (...) VALUES (...)
        todo!()
    }

    pub async fn update_state(&self, id: Uuid, state: &str) -> Result<()> {
        // TODO: UPDATE agreements SET state=$1, updated_at=NOW() WHERE id=$2
        todo!()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Milestone Repository
// ─────────────────────────────────────────────────────────────────────────────

pub struct MilestoneRepository {}

impl MilestoneRepository {
    pub fn new() -> Self { Self {} }

    pub async fn list_for_agreement(&self, agreement_id: Uuid) -> Result<Vec<MilestoneRow>> {
        // TODO: SELECT * FROM milestones WHERE agreement_id = $1 ORDER BY on_chain_index
        todo!()
    }

    pub async fn update_state(&self, id: Uuid, state: &str) -> Result<()> {
        todo!()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Dispute Repository
// ─────────────────────────────────────────────────────────────────────────────

pub struct DisputeRepository {}

impl DisputeRepository {
    pub fn new() -> Self { Self {} }

    pub async fn find_by_agreement(&self, agreement_id: Uuid) -> Result<Option<DisputeRow>> {
        todo!()
    }

    pub async fn insert(&self, row: &DisputeRow) -> Result<()> {
        todo!()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Indexer Cursor Repository
// ─────────────────────────────────────────────────────────────────────────────

pub struct IndexerCursorRepository {}

impl IndexerCursorRepository {
    pub fn new() -> Self { Self {} }

    pub async fn get_cursor(&self) -> Result<i64> {
        // TODO: SELECT last_ingested_ledger FROM indexer_cursor WHERE id=1
        Ok(0)
    }

    pub async fn set_cursor(&self, ledger: i64) -> Result<()> {
        // TODO: UPDATE indexer_cursor SET last_ingested_ledger=$1, updated_at=NOW() WHERE id=1
        Ok(())
    }
}
