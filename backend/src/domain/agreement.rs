//! Agreement domain model (off-chain projection of on-chain state).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgreementState {
    Draft,
    Funded,
    Active,
    Disputed,
    Completed,
    Expired,
    Released,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agreement {
    /// Internal DB primary key (UUID).
    pub id: Uuid,
    /// On-chain agreement ID (u64 counter from the contract).
    pub on_chain_id: i64,
    pub client_address: String,
    pub provider_address: String,
    /// Stellar asset contract address.
    pub token_address: String,
    pub total_amount: i64,
    pub escrowed_amount: i64,
    pub released_amount: i64,
    pub refunded_amount: i64,
    pub title: String,
    pub state: AgreementState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Ledger number at expiry (None = no expiry).
    pub expires_at_ledger: Option<i64>,
    /// Transaction hash from the create_agreement invocation.
    pub tx_hash: Option<String>,
}

impl Agreement {
    /// Invariant: escrowed + released + refunded must equal total.
    pub fn amounts_balance(&self) -> bool {
        self.escrowed_amount + self.released_amount + self.refunded_amount == self.total_amount
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgreementRequest {
    pub client_address: String,
    pub provider_address: String,
    pub token_address: String,
    pub total_amount: i64,
    pub title: String,
    pub expires_at_ledger: Option<i64>,
}
