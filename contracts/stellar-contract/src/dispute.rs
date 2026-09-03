use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

use crate::types::{Dispute, DisputeStatus, Agreement, AgreementStatus};
use crate::errors::SettleError;
use crate::validation::Validator;
use crate::events::EventBuilder;
use crate::storage::{DisputeStorage, AgreementStorage};

/// Dispute contract for structured resolution process
#[contract]
pub struct DisputeContract;

#[contractimpl]
impl DisputeContract {
    /// Open a dispute for an agreement
    pub fn open_dispute(
        env: Env,
        id: String,
        agreement_id: String,
        opener: Address,
        reason: String,
        initial_evidence: String,
    ) -> Result<Dispute, SettleError> {
        // Authorization check
        opener.require_auth();
        
        // Validate inputs
        Validator::validate_string_length(&reason, 500)?;
        Validator::validate_string_length(&initial_evidence, 2000)?;
        
        // Check agreement exists and can be disputed
        let agreement = AgreementStorage::get(&env, &agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        Validator::validate_can_open_dispute(&env, &agreement)?;
        
        // Check opener is a party
        if opener != agreement.creator && opener != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check dispute doesn't already exist
        if DisputeStorage::exists_for_agreement(&env, &agreement_id) {
            return Err(SettleError::DisputeAlreadyExists);
        }
        
        // Create dispute
        let mut evidence_vec = Vec::new(&env);
        evidence_vec.push_back(initial_evidence);
        
        let dispute = Dispute {
            id: id.clone(),
            agreement_id: agreement_id.clone(),
            opened_by: opener.clone(),
            reason,
            evidence: evidence_vec,
            status: DisputeStatus::Open,
            resolution: None,
            opened_at: env.ledger().timestamp(),
            resolved_at: None,
            arbitrator: None,
        };
        
        // Store dispute
        DisputeStorage::set(&env, &id, &dispute);
        DisputeStorage::set_agreement_dispute(&env, &agreement_id, &id);
        
        // Emit event
        EventBuilder::dispute_opened(&env, &dispute);
        
        Ok(dispute)
    }
    
    /// Submit evidence to a dispute
    pub fn submit_evidence(
        env: Env,
        id: String,
        submitter: Address,
        evidence: String,
    ) -> Result<Dispute, SettleError> {
        // Authorization check
        submitter.require_auth();
        
        // Validate evidence
        Validator::validate_string_length(&evidence, 2000)?;
        
        // Get dispute and agreement
        let mut dispute = DisputeStorage::get(&env, &id)
            .ok_or(SettleError::DisputeNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &dispute.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check submitter is a party
        if submitter != agreement.creator && submitter != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check dispute is open for evidence
        if dispute.status != DisputeStatus::Open && dispute.status != DisputeStatus::EvidenceSubmission {
            return Err(SettleError::DisputeNotOpen);
        }
        
        // Update dispute status if needed
        if dispute.status == DisputeStatus::Open {
            dispute.status = DisputeStatus::EvidenceSubmission;
        }
        
        // Add evidence
        dispute.evidence.push_back(evidence);
        
        // Store updated dispute
        DisputeStorage::set(&env, &id, &dispute);
        
        // Emit event
        EventBuilder::evidence_submitted(&env, &dispute, &submitter);
        
        Ok(dispute)
    }
    
    /// Move dispute to review phase
    pub fn move_to_review(
        env: Env,
        id: String,
        arbitrator: Address,
    ) -> Result<Dispute, SettleError> {
        // Authorization check - TODO: Validate arbitrator authority
        arbitrator.require_auth();
        
        // Get dispute
        let mut dispute = DisputeStorage::get(&env, &id)
            .ok_or(SettleError::DisputeNotFound)?;
        
        // Validate state transition
        Validator::validate_dispute_transition(&dispute.status, &DisputeStatus::UnderReview)?;
        
        // Update dispute
        dispute.status = DisputeStatus::UnderReview;
        dispute.arbitrator = Some(arbitrator);
        
        // Store updated dispute
        DisputeStorage::set(&env, &id, &dispute);
        
        Ok(dispute)
    }
    
    /// Resolve dispute with decision
    pub fn resolve_dispute(
        env: Env,
        id: String,
        arbitrator: Address,
        resolution: String,
        winner: Address,
        compensation_amount: i128,
    ) -> Result<Dispute, SettleError> {
        // Authorization check
        arbitrator.require_auth();
        
        // Validate inputs
        Validator::validate_string_length(&resolution, 1000)?;
        Validator::validate_amount(compensation_amount)?;
        
        // Get dispute and agreement
        let mut dispute = DisputeStorage::get(&env, &id)
            .ok_or(SettleError::DisputeNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &dispute.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check arbitrator is authorized
        if let Some(assigned_arbitrator) = &dispute.arbitrator {
            if arbitrator != *assigned_arbitrator {
                return Err(SettleError::NotArbitrator);
            }
        } else {
            return Err(SettleError::NotArbitrator);
        }
        
        // Validate state transition
        Validator::validate_dispute_transition(&dispute.status, &DisputeStatus::Resolved)?;
        
        // Check winner is a valid party
        if winner != agreement.creator && winner != agreement.counterparty {
            return Err(SettleError::InvalidParticipant);
        }
        
        // Update dispute
        dispute.status = DisputeStatus::Resolved;
        dispute.resolution = Some(resolution);
        dispute.resolved_at = Some(env.ledger().timestamp());
        
        // Store updated dispute
        DisputeStorage::set(&env, &id, &dispute);
        
        // Emit event
        EventBuilder::dispute_resolved(&env, &dispute, &winner, compensation_amount);
        
        Ok(dispute)
    }
    
    /// Close dispute without resolution (mutual agreement)
    pub fn close_dispute(
        env: Env,
        id: String,
        closer: Address,
    ) -> Result<Dispute, SettleError> {
        // Authorization check
        closer.require_auth();
        
        // Get dispute and agreement
        let mut dispute = DisputeStorage::get(&env, &id)
            .ok_or(SettleError::DisputeNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &dispute.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check closer is a party
        if closer != agreement.creator && closer != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Can close from various states
        match dispute.status {
            DisputeStatus::Open | DisputeStatus::EvidenceSubmission | DisputeStatus::UnderReview => {},
            _ => return Err(SettleError::InvalidDisputeStatus),
        }
        
        // Update dispute
        dispute.status = DisputeStatus::Closed;
        dispute.resolved_at = Some(env.ledger().timestamp());
        
        // Store updated dispute
        DisputeStorage::set(&env, &id, &dispute);
        
        // Emit event
        EventBuilder::dispute_closed(&env, &dispute);
        
        Ok(dispute)
    }
    
    /// Get dispute by ID
    pub fn get_dispute(env: Env, id: String) -> Option<Dispute> {
        DisputeStorage::get(&env, &id)
    }
    
    /// Get dispute by agreement ID
    pub fn get_dispute_by_agreement(env: Env, agreement_id: String) -> Option<Dispute> {
        let dispute_id = DisputeStorage::get_agreement_dispute(&env, &agreement_id)?;
        DisputeStorage::get(&env, &dispute_id)
    }
    
    /// Check if agreement has an open dispute
    pub fn has_open_dispute(env: Env, agreement_id: String) -> bool {
        if let Some(dispute_id) = DisputeStorage::get_agreement_dispute(&env, &agreement_id) {
            if let Some(dispute) = DisputeStorage::get(&env, &dispute_id) {
                return matches!(dispute.status, 
                    DisputeStatus::Open | 
                    DisputeStatus::EvidenceSubmission | 
                    DisputeStatus::UnderReview
                );
            }
        }
        false
    }
    
    /// Get disputes by participant
    pub fn get_disputes_by_participant(
        env: Env,
        participant: Address,
        limit: u32,
        offset: u32,
    ) -> Vec<Dispute> {
        DisputeStorage::get_by_participant(&env, &participant, limit, offset)
    }
    
    /// Get dispute statistics
    pub fn get_dispute_stats(env: Env) -> (u32, u32, u32, u32, u32) {
        let mut open = 0u32;
        let mut evidence = 0u32;
        let mut review = 0u32;
        let mut resolved = 0u32;
        let mut closed = 0u32;
        
        // TODO: Implement efficient stats collection from storage
        // For now, return zeros - would need to iterate through all disputes
        
        (open, evidence, review, resolved, closed)
    }
}