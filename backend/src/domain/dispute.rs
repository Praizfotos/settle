use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeState {
    Open,
    UnderReview,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeResolution {
    Pending,
    ReleasedToProvider,
    RefundedToClient,
    SplitSettlement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub id: Uuid,
    pub agreement_id: Uuid,
    pub on_chain_dispute_id: i64,
    pub opener_address: String,
    pub disputed_amount: i64,
    pub reason: String,
    pub state: DisputeState,
    pub resolution: DisputeResolution,
    pub opened_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}
