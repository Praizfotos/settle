use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneState {
    Pending,
    Submitted,
    Approved,
    Rejected,
    Released,
    Disputed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: Uuid,
    pub agreement_id: Uuid,
    pub on_chain_index: i32,
    pub title: String,
    pub amount: i64,
    pub state: MilestoneState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_at_ledger: Option<i64>,
}
