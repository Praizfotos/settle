//! Centralised error types for the Settle backend.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // ── Database ──────────────────────────────────────────────────────
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    // ── Stellar / Soroban ─────────────────────────────────────────────
    #[error("stellar rpc error: {0}")]
    StellarRpc(String),

    #[error("xdr decode error: {0}")]
    XdrDecode(String),

    #[error("contract invocation failed: {0}")]
    ContractInvocation(String),

    // ── Domain ────────────────────────────────────────────────────────
    #[error("agreement not found: {0}")]
    AgreementNotFound(u64),

    #[error("milestone not found: agreement={0} index={1}")]
    MilestoneNotFound(u64, u32),

    #[error("dispute not found for agreement: {0}")]
    DisputeNotFound(u64),

    #[error("invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("insufficient funds")]
    InsufficientFunds,

    #[error("unauthorized")]
    Unauthorized,

    // ── Indexer ───────────────────────────────────────────────────────
    #[error("indexer cursor error: {0}")]
    IndexerCursor(String),

    #[error("event processing error: {0}")]
    EventProcessing(String),

    // ── Generic ───────────────────────────────────────────────────────
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Convenience alias used throughout the codebase.
pub type Result<T> = std::result::Result<T, AppError>;
