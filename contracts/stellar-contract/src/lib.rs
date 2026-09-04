#![no_std]

extern crate soroban_sdk;

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

// Module declarations
mod types;
mod errors;
mod validation;
mod authorization;
mod upgrade;
mod agreement;
mod escrow;
mod milestone;
mod dispute;
mod storage;
mod events;

#[cfg(test)]
mod agreement_tests;

#[cfg(test)]
mod escrow_tests;

#[cfg(test)]
mod milestone_tests;

#[cfg(test)]
mod dispute_tests;

// Re-exports for external usage
pub use types::*;
pub use errors::*;

use agreement::AgreementContract;
use escrow::EscrowContract;
use milestone::MilestoneContract;
use dispute::DisputeContract;
use upgrade::UpgradeManager;

/// Main Settle contract with modular architecture and separated concerns
#[contract]
pub struct SettleContract;

#[contractimpl]
impl SettleContract {
    /// Initialize the contract
    pub fn initialize(env: Env, admin: Address) -> Result<(), SettleError> {
        // Check if contract operations are not paused
        UpgradeManager::require_not_paused(&env)?;
        
        // Initialize deployment metadata
        UpgradeManager::initialize_deployment(&env, &admin)?;
        
        Ok(())
    }
    
    // Agreement management functions
    pub fn create_agreement(
        env: Env,
        id: String,
        creator: Address,
        counterparty: Address,
        token: Address,
        total_amount: i128,
        expires_at: u64,
        milestones: Vec<String>,
    ) -> Result<Agreement, SettleError> {
        UpgradeManager::require_not_paused(&env)?;
        AgreementContract::create_agreement(
            env, id, creator, counterparty, token, total_amount, expires_at, milestones
        )
    }
    
    pub fn fund_agreement(
        env: Env,
        id: String,
        funder: Address,
        amount: i128,
    ) -> Result<Agreement, SettleError> {
        AgreementContract::fund_agreement(env, id, funder, amount)
    }
    
    pub fn activate_agreement(
        env: Env,
        id: String,
        activator: Address,
    ) -> Result<Agreement, SettleError> {
        AgreementContract::activate_agreement(env, id, activator)
    }
    
    pub fn complete_agreement(
        env: Env,
        id: String,
        completer: Address,
    ) -> Result<Agreement, SettleError> {
        AgreementContract::complete_agreement(env, id, completer)
    }
    
    pub fn cancel_agreement(
        env: Env,
        id: String,
        canceller: Address,
    ) -> Result<Agreement, SettleError> {
        AgreementContract::cancel_agreement(env, id, canceller)
    }
    
    pub fn get_agreement(env: Env, id: String) -> Option<Agreement> {
        AgreementContract::get_agreement(env, id)
    }
    
    // Milestone management functions
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
        MilestoneContract::create_milestone(
            env, id, agreement_id, creator, name, description, amount, due_date
        )
    }
    
    pub fn submit_milestone(
        env: Env,
        id: String,
        submitter: Address,
        evidence: String,
    ) -> Result<Milestone, SettleError> {
        MilestoneContract::submit_milestone(env, id, submitter, evidence)
    }
    
    pub fn approve_milestone(
        env: Env,
        id: String,
        approver: Address,
    ) -> Result<Milestone, SettleError> {
        MilestoneContract::approve_milestone(env, id, approver)
    }
    
    pub fn reject_milestone(
        env: Env,
        id: String,
        rejecter: Address,
        reason: String,
    ) -> Result<Milestone, SettleError> {
        MilestoneContract::reject_milestone(env, id, rejecter, reason)
    }
    
    pub fn release_milestone_payment(
        env: Env,
        id: String,
        releaser: Address,
        recipient: Address,
    ) -> Result<Milestone, SettleError> {
        MilestoneContract::release_milestone_payment(env, id, releaser, recipient)
    }
    
    pub fn get_milestone(env: Env, id: String) -> Option<Milestone> {
        MilestoneContract::get_milestone(env, id)
    }
    
    // Escrow management functions
    pub fn create_escrow(
        env: Env,
        agreement_id: String,
        token: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        EscrowContract::create_escrow(env, agreement_id, token, amount)
    }
    
    pub fn fund_escrow(
        env: Env,
        agreement_id: String,
        funder: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        EscrowContract::fund_escrow(env, agreement_id, funder, amount)
    }
    
    pub fn lock_escrow(
        env: Env,
        agreement_id: String,
        locker: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        EscrowContract::lock_escrow(env, agreement_id, locker, amount)
    }
    
    pub fn release_escrow(
        env: Env,
        agreement_id: String,
        releaser: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        EscrowContract::release_escrow(env, agreement_id, releaser, recipient, amount)
    }
    
    pub fn refund_escrow(
        env: Env,
        agreement_id: String,
        refunder: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        EscrowContract::refund_escrow(env, agreement_id, refunder, recipient, amount)
    }
    
    pub fn get_escrow(env: Env, agreement_id: String) -> Option<Escrow> {
        EscrowContract::get_escrow(env, agreement_id)
    }
    
    // Dispute management functions
    pub fn open_dispute(
        env: Env,
        id: String,
        agreement_id: String,
        opener: Address,
        reason: String,
        initial_evidence: String,
    ) -> Result<Dispute, SettleError> {
        DisputeContract::open_dispute(env, id, agreement_id, opener, reason, initial_evidence)
    }
    
    pub fn submit_evidence(
        env: Env,
        id: String,
        submitter: Address,
        evidence: String,
    ) -> Result<Dispute, SettleError> {
        DisputeContract::submit_evidence(env, id, submitter, evidence)
    }
    
    pub fn resolve_dispute(
        env: Env,
        id: String,
        arbitrator: Address,
        resolution: String,
        winner: Address,
        compensation_amount: i128,
    ) -> Result<Dispute, SettleError> {
        DisputeContract::resolve_dispute(env, id, arbitrator, resolution, winner, compensation_amount)
    }
    
    pub fn close_dispute(
        env: Env,
        id: String,
        closer: Address,
    ) -> Result<Dispute, SettleError> {
        DisputeContract::close_dispute(env, id, closer)
    }
    
    pub fn get_dispute(env: Env, id: String) -> Option<Dispute> {
        DisputeContract::get_dispute(env, id)
    }
    
    // Utility and query functions
    pub fn get_agreements_by_participant(
        env: Env,
        participant: Address,
        limit: u32,
        offset: u32,
    ) -> Vec<Agreement> {
        AgreementContract::get_agreements_by_participant(env, participant, limit, offset)
    }
    
    pub fn get_milestones_by_agreement(
        env: Env,
        agreement_id: String,
        limit: u32,
        offset: u32,
    ) -> Vec<Milestone> {
        MilestoneContract::get_milestones_by_agreement(env, agreement_id, limit, offset)
    }
    
    pub fn has_open_dispute(env: Env, agreement_id: String) -> bool {
        DisputeContract::has_open_dispute(env, agreement_id)
    }
    
    pub fn get_contract_version(env: Env) -> u32 {
        UpgradeManager::get_version(&env)
    }
    
    // Administrative functions
    
    /// Pause contract operations (admin only)
    pub fn pause_contract(env: Env, admin: Address) -> Result<(), SettleError> {
        UpgradeManager::pause_contract(&env, &admin)
    }
    
    /// Resume contract operations (admin only)
    pub fn resume_contract(env: Env, admin: Address) -> Result<(), SettleError> {
        UpgradeManager::resume_contract(&env, &admin)
    }
    
    /// Check if contract is paused
    pub fn is_contract_paused(env: Env) -> bool {
        UpgradeManager::is_paused(&env)
    }
    
    /// Set contract version (admin only)
    pub fn set_contract_version(env: Env, admin: Address, version: u32) -> Result<(), SettleError> {
        UpgradeManager::set_version(&env, &admin, version)
    }
}