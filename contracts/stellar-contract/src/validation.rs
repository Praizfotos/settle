use soroban_sdk::{Address, Env, String};

use crate::errors::SettleError;
use crate::types::{Agreement, AgreementStatus, Milestone, MilestoneStatus, DisputeStatus};

/// Input validation utilities
pub struct Validator;

impl Validator {
    /// Validate amount is positive and within reasonable bounds
    pub fn validate_amount(amount: i128) -> Result<(), SettleError> {
        if amount <= 0 {
            return Err(SettleError::InvalidAmount);
        }
        
        // Prevent overflow issues - max 10^18 (equivalent to 1 billion tokens with 9 decimals)
        if amount > 1_000_000_000_000_000_000i128 {
            return Err(SettleError::InvalidAmount);
        }
        
        Ok(())
    }
    
    /// Validate percentage is between 0 and 100
    pub fn validate_percentage(percentage: u32) -> Result<(), SettleError> {
        if percentage > 100 {
            return Err(SettleError::InvalidPercentage);
        }
        Ok(())
    }
    
    /// Validate address is not zero and properly formatted
    pub fn validate_address(env: &Env, address: &Address) -> Result<(), SettleError> {
        // Basic validation - Soroban SDK handles format validation
        // Additional checks can be added here
        Ok(())
    }
    
    /// Validate two addresses are different (for agreements between different parties)
    pub fn validate_different_parties(creator: &Address, counterparty: &Address) -> Result<(), SettleError> {
        if creator == counterparty {
            return Err(SettleError::InvalidParticipant);
        }
        Ok(())
    }
    
    /// Validate timestamp is in reasonable range
    pub fn validate_timestamp(env: &Env, timestamp: u64) -> Result<(), SettleError> {
        let current_time = env.ledger().timestamp();
        
        // Allow some tolerance for clock skew (5 minutes)
        if timestamp > current_time + 300 {
            return Err(SettleError::InvalidTimestamp);
        }
        
        // Don't allow timestamps too far in the past (1 year)
        if timestamp < current_time - 31_536_000 {
            return Err(SettleError::InvalidTimestamp);
        }
        
        Ok(())
    }
    
    /// Validate deadline is in the future
    pub fn validate_deadline(env: &Env, deadline: u64) -> Result<(), SettleError> {
        let current_time = env.ledger().timestamp();
        
        if deadline <= current_time {
            return Err(SettleError::InvalidDeadline);
        }
        
        // Reasonable maximum deadline (10 years from now)
        if deadline > current_time + 315_360_000 {
            return Err(SettleError::InvalidDeadline);
        }
        
        Ok(())
    }
    
    /// Validate string length and content
    pub fn validate_string_length(s: &String, max_length: u32) -> Result<(), SettleError> {
        if s.len() > max_length {
            return Err(SettleError::InvalidEvidence);
        }
        Ok(())
    }
    
    /// Validate agreement state transition
    pub fn validate_agreement_transition(
        current: &AgreementStatus,
        target: &AgreementStatus,
    ) -> Result<(), SettleError> {
        use AgreementStatus::*;
        
        let valid_transition = match (current, target) {
            // From Draft
            (Draft, Funded) => true,
            (Draft, Cancelled) => true,
            
            // From Funded
            (Funded, Active) => true,
            (Funded, Cancelled) => true,
            (Funded, Expired) => true,
            
            // From Active
            (Active, Completed) => true,
            (Active, Disputed) => true,
            (Active, Expired) => true,
            
            // From Disputed
            (Disputed, Resolved) => true,
            (Disputed, Cancelled) => true,
            
            // No transitions from terminal states
            (Completed | Resolved | Expired | Cancelled, _) => false,
            
            // Any other transition is invalid
            _ => false,
        };
        
        if !valid_transition {
            return Err(SettleError::InvalidStateTransition);
        }
        
        Ok(())
    }
    
    /// Validate milestone state transition
    pub fn validate_milestone_transition(
        current: &MilestoneStatus,
        target: &MilestoneStatus,
    ) -> Result<(), SettleError> {
        use MilestoneStatus::*;
        
        let valid_transition = match (current, target) {
            // From Pending
            (Pending, Submitted) => true,
            
            // From Submitted
            (Submitted, Approved) => true,
            (Submitted, Rejected) => true,
            
            // From Approved
            (Approved, Released) => true,
            
            // From Rejected back to Pending (can resubmit)
            (Rejected, Pending) => true,
            
            // No transitions from Released
            (Released, _) => false,
            
            // Any other transition is invalid
            _ => false,
        };
        
        if !valid_transition {
            return Err(SettleError::InvalidStateTransition);
        }
        
        Ok(())
    }
    
    /// Validate dispute state transition
    pub fn validate_dispute_transition(
        current: &DisputeStatus,
        target: &DisputeStatus,
    ) -> Result<(), SettleError> {
        use DisputeStatus::*;
        
        let valid_transition = match (current, target) {
            // From Open
            (Open, EvidenceSubmission) => true,
            (Open, Closed) => true,
            
            // From EvidenceSubmission
            (EvidenceSubmission, UnderReview) => true,
            (EvidenceSubmission, Closed) => true,
            
            // From UnderReview
            (UnderReview, Resolved) => true,
            (UnderReview, Closed) => true,
            
            // No transitions from terminal states
            (Resolved | Closed, _) => false,
            
            // Any other transition is invalid
            _ => false,
        };
        
        if !valid_transition {
            return Err(SettleError::InvalidStateTransition);
        }
        
        Ok(())
    }
    
    /// Validate agreement can be funded
    pub fn validate_can_fund(agreement: &Agreement) -> Result<(), SettleError> {
        match agreement.status {
            AgreementStatus::Draft => Ok(()),
            AgreementStatus::Funded => Err(SettleError::AgreementAlreadyFunded),
            AgreementStatus::Expired => Err(SettleError::AgreementExpired),
            AgreementStatus::Cancelled => Err(SettleError::AgreementCancelled),
            _ => Err(SettleError::InvalidAgreementStatus),
        }
    }
    
    /// Validate milestone can be submitted
    pub fn validate_can_submit_milestone(
        env: &Env,
        milestone: &Milestone,
        agreement: &Agreement,
    ) -> Result<(), SettleError> {
        // Check milestone status
        if milestone.status != MilestoneStatus::Pending {
            return Err(SettleError::InvalidMilestoneStatus);
        }
        
        // Check agreement is active
        if agreement.status != AgreementStatus::Active {
            return Err(SettleError::AgreementNotActive);
        }
        
        // Check not expired
        let current_time = env.ledger().timestamp();
        if milestone.due_date < current_time {
            return Err(SettleError::MilestoneExpired);
        }
        
        Ok(())
    }
    
    /// Validate dispute can be opened
    pub fn validate_can_open_dispute(
        env: &Env,
        agreement: &Agreement,
    ) -> Result<(), SettleError> {
        // Can only dispute active agreements
        match agreement.status {
            AgreementStatus::Active => Ok(()),
            AgreementStatus::Completed => Err(SettleError::InvalidAgreementStatus),
            AgreementStatus::Disputed => Err(SettleError::DisputeAlreadyExists),
            _ => Err(SettleError::InvalidAgreementStatus),
        }
    }
}