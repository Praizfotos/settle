use soroban_sdk::contracterror;

/// Centralized contract error system
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SettleError {
    // Authorization errors (1000-1099)
    Unauthorized = 1001,
    NotAgreementParty = 1002,
    NotArbitrator = 1003,
    
    // Agreement errors (2000-2099)
    AgreementNotFound = 2001,
    AgreementAlreadyExists = 2002,
    AgreementAlreadyFunded = 2003,
    AgreementNotFunded = 2004,
    AgreementExpired = 2005,
    AgreementCancelled = 2006,
    AgreementNotActive = 2007,
    InvalidAgreementStatus = 2008,
    
    // Funding errors (3000-3099)
    InsufficientFunds = 3001,
    InsufficientEscrowBalance = 3002,
    EscrowAlreadyFunded = 3003,
    EscrowNotFunded = 3004,
    TransferFailed = 3005,
    InvalidAmount = 3006,
    
    // Milestone errors (4000-4099)
    MilestoneNotFound = 4001,
    MilestoneAlreadyExists = 4002,
    MilestoneNotSubmitted = 4003,
    MilestoneAlreadyApproved = 4004,
    MilestoneExpired = 4005,
    InvalidMilestoneStatus = 4006,
    
    // Dispute errors (5000-5099)
    DisputeNotFound = 5001,
    DisputeAlreadyExists = 5002,
    DisputeNotOpen = 5003,
    InvalidDisputeStatus = 5004,
    
    // Validation errors (6000-6099)
    InvalidParticipant = 6001,
    InvalidDeadline = 6002,
    InvalidEvidence = 6003,
    InvalidTimestamp = 6004,
    InvalidPercentage = 6005,
    
    // State errors (7000-7099)  
    InvalidStateTransition = 7001,
    
    // System errors (9000-9099)
    SystemError = 9001,
}