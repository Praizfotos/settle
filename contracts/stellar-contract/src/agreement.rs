use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

use crate::types::{Agreement, AgreementStatus, EventType, SettleEvent};
use crate::errors::SettleError;
use crate::validation::Validator;
use crate::events::EventBuilder;
use crate::storage::AgreementStorage;

/// Agreement contract with domain-driven architecture
#[contract]
pub struct AgreementContract;

#[contractimpl]
impl AgreementContract {
    /// Create a new agreement between parties
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
        // Authorization check
        creator.require_auth();
        
        // Validate inputs
        Validator::validate_amount(total_amount)?;
        Validator::validate_address(&env, &creator)?;
        Validator::validate_address(&env, &counterparty)?;
        Validator::validate_address(&env, &token)?;
        Validator::validate_different_parties(&creator, &counterparty)?;
        Validator::validate_deadline(&env, expires_at)?;
        
        // Check agreement doesn't already exist
        if AgreementStorage::exists(&env, &id) {
            return Err(SettleError::AgreementAlreadyExists);
        }
        
        // Create agreement
        let agreement = Agreement {
            id: id.clone(),
            creator: creator.clone(),
            counterparty: counterparty.clone(),
            token,
            total_amount,
            funded_amount: 0,
            released_amount: 0,
            refunded_amount: 0,
            status: AgreementStatus::Draft,
            created_at: env.ledger().timestamp(),
            expires_at,
            milestones,
        };
        
        // Store agreement
        AgreementStorage::set(&env, &id, &agreement);
        
        // Emit event
        EventBuilder::agreement_created(&env, &agreement);
        
        Ok(agreement)
    }
    
    /// Fund an agreement with tokens
    pub fn fund_agreement(
        env: Env,
        id: String,
        funder: Address,
        amount: i128,
    ) -> Result<Agreement, SettleError> {
        // Authorization check
        funder.require_auth();
        
        // Validate amount
        Validator::validate_amount(amount)?;
        
        // Get agreement
        let mut agreement = AgreementStorage::get(&env, &id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Validate can fund
        Validator::validate_can_fund(&agreement)?;
        
        // Check funder is a party to the agreement
        if funder != agreement.creator && funder != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check not already fully funded
        let remaining = agreement.total_amount - agreement.funded_amount;
        if amount > remaining {
            return Err(SettleError::InvalidAmount);
        }
        
        // Update funded amount
        agreement.funded_amount += amount;
        
        // Update status if fully funded
        if agreement.funded_amount >= agreement.total_amount {
            agreement.status = AgreementStatus::Funded;
        }
        
        // Store updated agreement
        AgreementStorage::set(&env, &id, &agreement);
        
        // Emit events
        if agreement.status == AgreementStatus::Funded {
            EventBuilder::agreement_funded(&env, &agreement);
        }
        
        Ok(agreement)
    }
    
    /// Activate a funded agreement
    pub fn activate_agreement(
        env: Env,
        id: String,
        activator: Address,
    ) -> Result<Agreement, SettleError> {
        // Authorization check
        activator.require_auth();
        
        // Get agreement
        let mut agreement = AgreementStorage::get(&env, &id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check activator is a party
        if activator != agreement.creator && activator != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Validate state transition
        Validator::validate_agreement_transition(&agreement.status, &AgreementStatus::Active)?;
        
        // Check fully funded
        if agreement.funded_amount < agreement.total_amount {
            return Err(SettleError::AgreementNotFunded);
        }
        
        // Check not expired
        if env.ledger().timestamp() >= agreement.expires_at {
            return Err(SettleError::AgreementExpired);
        }
        
        // Update status
        agreement.status = AgreementStatus::Active;
        
        // Store updated agreement
        AgreementStorage::set(&env, &id, &agreement);
        
        // Emit event
        EventBuilder::agreement_activated(&env, &agreement);
        
        Ok(agreement)
    }
    
    /// Complete an agreement (all milestones met)
    pub fn complete_agreement(
        env: Env,
        id: String,
        completer: Address,
    ) -> Result<Agreement, SettleError> {
        // Authorization check
        completer.require_auth();
        
        // Get agreement
        let mut agreement = AgreementStorage::get(&env, &id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check completer is a party
        if completer != agreement.creator && completer != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Validate state transition
        Validator::validate_agreement_transition(&agreement.status, &AgreementStatus::Completed)?;
        
        // Update status
        agreement.status = AgreementStatus::Completed;
        
        // Store updated agreement
        AgreementStorage::set(&env, &id, &agreement);
        
        // Emit event
        EventBuilder::agreement_completed(&env, &agreement);
        
        Ok(agreement)
    }
    
    /// Cancel an agreement (before completion)
    pub fn cancel_agreement(
        env: Env,
        id: String,
        canceller: Address,
    ) -> Result<Agreement, SettleError> {
        // Authorization check
        canceller.require_auth();
        
        // Get agreement
        let mut agreement = AgreementStorage::get(&env, &id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check canceller is a party
        if canceller != agreement.creator && canceller != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Validate state transition
        Validator::validate_agreement_transition(&agreement.status, &AgreementStatus::Cancelled)?;
        
        // Update status
        agreement.status = AgreementStatus::Cancelled;
        
        // Store updated agreement
        AgreementStorage::set(&env, &id, &agreement);
        
        // Emit event
        EventBuilder::agreement_cancelled(&env, &agreement);
        
        Ok(agreement)
    }
    
    /// Get agreement by ID
    pub fn get_agreement(env: Env, id: String) -> Option<Agreement> {
        AgreementStorage::get(&env, &id)
    }
    
    /// Check if agreement exists
    pub fn agreement_exists(env: Env, id: String) -> bool {
        AgreementStorage::exists(&env, &id)
    }
    
    /// Get agreements by participant
    pub fn get_agreements_by_participant(
        env: Env,
        participant: Address,
        limit: u32,
        offset: u32,
    ) -> Vec<Agreement> {
        AgreementStorage::get_by_participant(&env, &participant, limit, offset)
    }
}