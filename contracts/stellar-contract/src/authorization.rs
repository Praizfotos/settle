use soroban_sdk::{Address, Env, String};

use crate::errors::SettleError;
use crate::types::{Agreement, Milestone, Dispute, AgreementStatus, DisputeStatus};
use crate::storage::{AgreementStorage, MilestoneStorage, DisputeStorage};

/// Authorization utilities for secure settlement operations
pub struct Authorization;

impl Authorization {
    /// Verify caller is authorized to perform agreement actions
    pub fn require_agreement_party(
        env: &Env,
        agreement_id: &str,
        caller: &Address,
    ) -> Result<Agreement, SettleError> {
        let agreement_string = String::from_str(env, agreement_id);
        let agreement = AgreementStorage::get(env, &agreement_string)
            .ok_or(SettleError::AgreementNotFound)?;
            
        if caller != &agreement.creator && caller != &agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        Ok(agreement)
    }
    
    /// Verify caller can fund an agreement (creator or counterparty)
    pub fn require_can_fund_agreement(
        env: &Env,
        agreement_id: &str,
        caller: &Address,
    ) -> Result<Agreement, SettleError> {
        let agreement = Self::require_agreement_party(env, agreement_id, caller)?;
        
        if agreement.status != AgreementStatus::Draft {
            return Err(SettleError::InvalidAgreementStatus);
        }
        
        Ok(agreement)
    }
    
    /// Verify caller can activate agreement (typically counterparty after funding)
    pub fn require_can_activate_agreement(
        env: &Env,
        agreement_id: &str,
        caller: &Address,
    ) -> Result<Agreement, SettleError> {
        let agreement = Self::require_agreement_party(env, agreement_id, caller)?;
        
        if agreement.status != AgreementStatus::Funded {
            return Err(SettleError::InvalidAgreementStatus);
        }
        
        // Typically counterparty activates after creator funds
        if caller == &agreement.creator {
            return Err(SettleError::Unauthorized);
        }
        
        Ok(agreement)
    }
    
    /// Verify caller can submit milestone evidence
    pub fn require_can_submit_milestone(
        env: &Env,
        milestone_id: &str,
        caller: &Address,
    ) -> Result<(Agreement, Milestone), SettleError> {
        let milestone_string = String::from_str(env, milestone_id);
        let milestone = MilestoneStorage::get(env, &milestone_string)
            .ok_or(SettleError::MilestoneNotFound)?;
            
        let agreement = AgreementStorage::get(env, &milestone.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
            
        // Check agreement is active
        if agreement.status != AgreementStatus::Active {
            return Err(SettleError::AgreementNotActive);
        }
        
        // Check caller is agreement party (usually creator submits milestones)
        if caller != &agreement.creator && caller != &agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        Ok((agreement, milestone))
    }
    
    /// Verify caller can approve/reject milestone
    pub fn require_can_review_milestone(
        env: &Env,
        milestone_id: &str,
        caller: &Address,
    ) -> Result<(Agreement, Milestone), SettleError> {
        let milestone_string = String::from_str(env, milestone_id);
        let milestone = MilestoneStorage::get(env, &milestone_string)
            .ok_or(SettleError::MilestoneNotFound)?;
            
        let agreement = AgreementStorage::get(env, &milestone.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
            
        // Check agreement is active
        if agreement.status != AgreementStatus::Active {
            return Err(SettleError::AgreementNotActive);
        }
        
        // Typically counterparty reviews milestone submissions
        if caller == &agreement.creator {
            return Err(SettleError::Unauthorized);
        }
        
        if caller != &agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        Ok((agreement, milestone))
    }
    
    /// Verify caller can open dispute
    pub fn require_can_open_dispute(
        env: &Env,
        agreement_id: &str,
        caller: &Address,
    ) -> Result<Agreement, SettleError> {
        let agreement = Self::require_agreement_party(env, agreement_id, caller)?;
        
        // Can only dispute active or completed agreements
        match agreement.status {
            AgreementStatus::Active | AgreementStatus::Completed => {},
            _ => return Err(SettleError::InvalidAgreementStatus),
        }
        
        // Check if dispute already exists
        let agreement_string = String::from_str(env, agreement_id);
        if DisputeStorage::exists_for_agreement(env, &agreement_string) {
            return Err(SettleError::DisputeAlreadyExists);
        }
        
        Ok(agreement)
    }
    
    /// Verify caller can submit evidence to dispute
    pub fn require_can_submit_evidence(
        env: &Env,
        dispute_id: &str,
        caller: &Address,
    ) -> Result<(Agreement, Dispute), SettleError> {
        let dispute_string = String::from_str(env, dispute_id);
        let dispute = DisputeStorage::get(env, &dispute_string)
            .ok_or(SettleError::DisputeNotFound)?;
            
        let agreement = AgreementStorage::get(env, &dispute.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
            
        // Check dispute is in evidence submission phase
        match dispute.status {
            DisputeStatus::Open | DisputeStatus::EvidenceSubmission => {},
            _ => return Err(SettleError::InvalidDisputeStatus),
        }
        
        // Check caller is agreement party
        if caller != &agreement.creator && caller != &agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        Ok((agreement, dispute))
    }
    
    /// Verify caller is authorized arbitrator (for MVP, this is simplified)
    pub fn require_arbitrator(
        env: &Env,
        dispute_id: &str,
        _caller: &Address,
    ) -> Result<Dispute, SettleError> {
        let dispute_string = String::from_str(env, dispute_id);
        let dispute = DisputeStorage::get(env, &dispute_string)
            .ok_or(SettleError::DisputeNotFound)?;
            
        // For MVP: any address can act as arbitrator
        // In production: check against registered arbitrator list
        if dispute.status != DisputeStatus::UnderReview {
            return Err(SettleError::InvalidDisputeStatus);
        }
        
        Ok(dispute)
    }
    
    /// Verify caller has admin privileges for contract management
    pub fn require_admin(_env: &Env, caller: &Address) -> Result<(), SettleError> {
        // For now, check if caller deployed the contract
        // In production: maintain admin list in storage
        caller.require_auth();
        Ok(())
    }
    
    /// Check if caller is admin without requiring auth (for view functions)
    pub fn is_admin(_env: &Env, _caller: &Address) -> bool {
        // Simplified check for MVP
        true // In production: check against stored admin list
    }
}