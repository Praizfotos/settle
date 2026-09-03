use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

use crate::types::{Milestone, MilestoneStatus, Agreement, AgreementStatus};
use crate::errors::SettleError;
use crate::validation::Validator;
use crate::events::EventBuilder;
use crate::storage::{MilestoneStorage, AgreementStorage};

/// Milestone contract for progress tracking and validation
#[contract]
pub struct MilestoneContract;

#[contractimpl]
impl MilestoneContract {
    /// Create a new milestone for an agreement
    pub fn create_milestone(
        env: Env,
        id: String,
        agreement_id: String,
        creator: Address,
        name: String,
        description: String,
        amount: i128,
        due_date: u64,
    ) -> Result<Milestone, SettleError> {
        // Authorization check
        creator.require_auth();
        
        // Validate inputs
        Validator::validate_amount(amount)?;
        Validator::validate_deadline(&env, due_date)?;
        Validator::validate_string_length(&name, 100)?;
        Validator::validate_string_length(&description, 1000)?;
        
        // Check agreement exists
        let agreement = AgreementStorage::get(&env, &agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check creator is a party
        if creator != agreement.creator && creator != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check milestone doesn't already exist
        if MilestoneStorage::exists(&env, &id) {
            return Err(SettleError::MilestoneAlreadyExists);
        }
        
        // Create milestone
        let milestone = Milestone {
            id: id.clone(),
            agreement_id: agreement_id.clone(),
            name,
            description,
            amount,
            status: MilestoneStatus::Pending,
            due_date,
            submitted_at: None,
            approved_at: None,
            evidence: None,
        };
        
        // Store milestone
        MilestoneStorage::set(&env, &id, &milestone);
        
        // Emit event
        EventBuilder::milestone_created(&env, &milestone);
        
        Ok(milestone)
    }
    
    /// Submit milestone with evidence
    pub fn submit_milestone(
        env: Env,
        id: String,
        submitter: Address,
        evidence: String,
    ) -> Result<Milestone, SettleError> {
        // Authorization check
        submitter.require_auth();
        
        // Validate evidence
        Validator::validate_string_length(&evidence, 5000)?;
        
        // Get milestone and agreement
        let mut milestone = MilestoneStorage::get(&env, &id)
            .ok_or(SettleError::MilestoneNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &milestone.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check submitter is the counterparty (work performer)
        if submitter != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Validate can submit
        Validator::validate_can_submit_milestone(&env, &milestone, &agreement)?;
        
        // Update milestone
        milestone.status = MilestoneStatus::Submitted;
        milestone.submitted_at = Some(env.ledger().timestamp());
        milestone.evidence = Some(evidence);
        
        // Store updated milestone
        MilestoneStorage::set(&env, &id, &milestone);
        
        // Emit event
        EventBuilder::milestone_submitted(&env, &milestone);
        
        Ok(milestone)
    }
    
    /// Approve milestone (by agreement creator)
    pub fn approve_milestone(
        env: Env,
        id: String,
        approver: Address,
    ) -> Result<Milestone, SettleError> {
        // Authorization check
        approver.require_auth();
        
        // Get milestone and agreement
        let mut milestone = MilestoneStorage::get(&env, &id)
            .ok_or(SettleError::MilestoneNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &milestone.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check approver is the creator (work requester)
        if approver != agreement.creator {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Validate state transition
        Validator::validate_milestone_transition(&milestone.status, &MilestoneStatus::Approved)?;
        
        // Check milestone was submitted
        if milestone.status != MilestoneStatus::Submitted {
            return Err(SettleError::MilestoneNotSubmitted);
        }
        
        // Update milestone
        milestone.status = MilestoneStatus::Approved;
        milestone.approved_at = Some(env.ledger().timestamp());
        
        // Store updated milestone
        MilestoneStorage::set(&env, &id, &milestone);
        
        // Emit event
        EventBuilder::milestone_approved(&env, &milestone);
        
        Ok(milestone)
    }
    
    /// Reject milestone with reason
    pub fn reject_milestone(
        env: Env,
        id: String,
        rejecter: Address,
        reason: String,
    ) -> Result<Milestone, SettleError> {
        // Authorization check
        rejecter.require_auth();
        
        // Validate reason
        Validator::validate_string_length(&reason, 1000)?;
        
        // Get milestone and agreement
        let mut milestone = MilestoneStorage::get(&env, &id)
            .ok_or(SettleError::MilestoneNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &milestone.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check rejecter is the creator
        if rejecter != agreement.creator {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Validate state transition
        Validator::validate_milestone_transition(&milestone.status, &MilestoneStatus::Rejected)?;
        
        // Check milestone was submitted
        if milestone.status != MilestoneStatus::Submitted {
            return Err(SettleError::MilestoneNotSubmitted);
        }
        
        // Update milestone
        milestone.status = MilestoneStatus::Rejected;
        // Store rejection reason in evidence field
        milestone.evidence = Some(reason);
        
        // Store updated milestone
        MilestoneStorage::set(&env, &id, &milestone);
        
        // Emit event
        EventBuilder::milestone_rejected(&env, &milestone);
        
        Ok(milestone)
    }
    
    /// Release payment for approved milestone
    pub fn release_milestone_payment(
        env: Env,
        id: String,
        releaser: Address,
        recipient: Address,
    ) -> Result<Milestone, SettleError> {
        // Authorization check
        releaser.require_auth();
        
        // Get milestone and agreement
        let mut milestone = MilestoneStorage::get(&env, &id)
            .ok_or(SettleError::MilestoneNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &milestone.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check releaser is a party
        if releaser != agreement.creator && releaser != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Validate state transition
        Validator::validate_milestone_transition(&milestone.status, &MilestoneStatus::Released)?;
        
        // Check milestone was approved
        if milestone.status != MilestoneStatus::Approved {
            return Err(SettleError::MilestoneAlreadyApproved);
        }
        
        // Update milestone
        milestone.status = MilestoneStatus::Released;
        
        // Store updated milestone
        MilestoneStorage::set(&env, &id, &milestone);
        
        // Emit event
        EventBuilder::milestone_released(&env, &milestone, &recipient);
        
        Ok(milestone)
    }
    
    /// Reset milestone to pending (after rejection)
    pub fn reset_milestone(
        env: Env,
        id: String,
        resetter: Address,
    ) -> Result<Milestone, SettleError> {
        // Authorization check
        resetter.require_auth();
        
        // Get milestone and agreement
        let mut milestone = MilestoneStorage::get(&env, &id)
            .ok_or(SettleError::MilestoneNotFound)?;
            
        let agreement = AgreementStorage::get(&env, &milestone.agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check resetter is the counterparty
        if resetter != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Can only reset rejected milestones
        if milestone.status != MilestoneStatus::Rejected {
            return Err(SettleError::InvalidMilestoneStatus);
        }
        
        // Reset milestone
        milestone.status = MilestoneStatus::Pending;
        milestone.submitted_at = None;
        milestone.approved_at = None;
        milestone.evidence = None;
        
        // Store updated milestone
        MilestoneStorage::set(&env, &id, &milestone);
        
        Ok(milestone)
    }
    
    /// Get milestone by ID
    pub fn get_milestone(env: Env, id: String) -> Option<Milestone> {
        MilestoneStorage::get(&env, &id)
    }
    
    /// Get milestones for an agreement
    pub fn get_milestones_by_agreement(
        env: Env,
        agreement_id: String,
        limit: u32,
        offset: u32,
    ) -> Vec<Milestone> {
        MilestoneStorage::get_by_agreement(&env, &agreement_id, limit, offset)
    }
    
    /// Get milestone completion statistics for an agreement
    pub fn get_milestone_stats(
        env: Env,
        agreement_id: String,
    ) -> Result<(u32, u32, u32, u32, u32), SettleError> {
        let milestones = MilestoneStorage::get_by_agreement(&env, &agreement_id, u32::MAX, 0);
        
        let mut pending = 0u32;
        let mut submitted = 0u32;
        let mut approved = 0u32;
        let mut rejected = 0u32;
        let mut released = 0u32;
        
        for milestone in milestones {
            match milestone.status {
                MilestoneStatus::Pending => pending += 1,
                MilestoneStatus::Submitted => submitted += 1,
                MilestoneStatus::Approved => approved += 1,
                MilestoneStatus::Rejected => rejected += 1,
                MilestoneStatus::Released => released += 1,
            }
        }
        
        Ok((pending, submitted, approved, rejected, released))
    }
}