use soroban_sdk::{contracttype, Address, Bytes, String, Vec};

/// Core agreement state and lifecycle management
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum AgreementStatus {
    Draft,
    Funded,
    Active,
    Completed,
    Disputed,
    Resolved,
    Expired,
    Cancelled,
}

/// Agreement data structure
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Agreement {
    pub id: String,
    pub creator: Address,
    pub counterparty: Address,
    pub token: Address,
    pub total_amount: i128,
    pub funded_amount: i128,
    pub released_amount: i128,
    pub refunded_amount: i128,
    pub status: AgreementStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub milestones: Vec<String>,
}

/// Milestone state management
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum MilestoneStatus {
    Pending,
    Submitted,
    Approved,
    Rejected,
    Released,
}

/// Milestone data structure
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Milestone {
    pub id: String,
    pub agreement_id: String,
    pub name: String,
    pub description: String,
    pub amount: i128,
    pub status: MilestoneStatus,
    pub due_date: u64,
    pub submitted_at: Option<u64>,
    pub approved_at: Option<u64>,
    pub evidence: Option<String>,
}

/// Escrow state transitions
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum EscrowStatus {
    Empty,
    Funded,
    Locked,
    Released,
    Refunded,
}

/// Escrow management
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Escrow {
    pub agreement_id: String,
    pub token: Address,
    pub amount: i128,
    pub locked_amount: i128,
    pub released_amount: i128,
    pub status: EscrowStatus,
    pub created_at: u64,
    pub last_action_at: u64,
}

/// Dispute resolution process
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum DisputeStatus {
    Open,
    EvidenceSubmission,
    UnderReview,
    Resolved,
    Closed,
}

/// Dispute data structure
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Dispute {
    pub id: String,
    pub agreement_id: String,
    pub opened_by: Address,
    pub reason: String,
    pub evidence: Vec<String>,
    pub status: DisputeStatus,
    pub resolution: Option<String>,
    pub opened_at: u64,
    pub resolved_at: Option<u64>,
    pub arbitrator: Option<Address>,
}

/// Participant information
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Participant {
    pub address: Address,
    pub reputation_score: u32,
    pub total_agreements: u32,
    pub successful_agreements: u32,
    pub disputed_agreements: u32,
    pub total_volume: i128,
    pub joined_at: u64,
    pub last_activity: u64,
}

/// Event types for indexing and audit trail
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum EventType {
    AgreementCreated,
    AgreementFunded,
    AgreementActivated,
    AgreementCompleted,
    AgreementExpired,
    AgreementCancelled,
    
    MilestoneCreated,
    MilestoneSubmitted,
    MilestoneApproved,
    MilestoneRejected,
    MilestoneReleased,
    
    EscrowFunded,
    EscrowLocked,
    EscrowReleased,
    EscrowRefunded,
    
    DisputeOpened,
    EvidenceSubmitted,
    DisputeResolved,
    DisputeClosed,
    
    ReputationUpdated,
}

/// Structured event for indexing
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct SettleEvent {
    pub event_type: EventType,
    pub agreement_id: Option<String>,
    pub milestone_id: Option<String>,
    pub dispute_id: Option<String>,
    pub participant: Address,
    pub data: Bytes,
    pub timestamp: u64,
    pub block_height: u32,
}