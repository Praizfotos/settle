use soroban_sdk::{Address, Env, String};

use crate::types::{Escrow, EscrowStatus, AgreementStatus};
use crate::errors::SettleError;
use crate::validation::Validator;
use crate::events::EventBuilder;
use crate::storage::{EscrowStorage, AgreementStorage};

/// Escrow contract for secure fund management with automated releases
pub struct EscrowContract;
impl EscrowContract {
    /// Create escrow for an agreement
    pub fn create_escrow(
        env: Env,
        agreement_id: String,
        token: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        // Validate inputs
        Validator::validate_amount(amount)?;
        Validator::validate_address(&env, &token)?;
        
        // Check agreement exists
        let agreement = AgreementStorage::get(&env, &agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
        
        // Check escrow doesn't already exist
        if EscrowStorage::exists(&env, &agreement_id) {
            return Err(SettleError::EscrowAlreadyFunded);
        }
        
        // Create escrow
        let escrow = Escrow {
            agreement_id: agreement_id.clone(),
            token,
            amount,
            locked_amount: 0,
            released_amount: 0,
            status: EscrowStatus::Empty,
            created_at: env.ledger().timestamp(),
            last_action_at: env.ledger().timestamp(),
        };
        
        // Store escrow
        EscrowStorage::set(&env, &agreement_id, &escrow);
        
        Ok(escrow)
    }
    
    /// Fund escrow with tokens
    pub fn fund_escrow(
        env: Env,
        agreement_id: String,
        funder: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        // Authorization check
        funder.require_auth();
        
        // Validate amount
        Validator::validate_amount(amount)?;
        
        // Get agreement and escrow
        let agreement = AgreementStorage::get(&env, &agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
            
        let mut escrow = EscrowStorage::get(&env, &agreement_id)
            .ok_or(SettleError::EscrowNotFunded)?;
        
        // Check funder is a party to the agreement
        if funder != agreement.creator && funder != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check escrow can be funded
        if escrow.status != EscrowStatus::Empty {
            return Err(SettleError::EscrowAlreadyFunded);
        }
        
        // Check amount matches expected
        if amount != escrow.amount {
            return Err(SettleError::InvalidAmount);
        }
        
        // Update escrow status
        escrow.status = EscrowStatus::Funded;
        escrow.last_action_at = env.ledger().timestamp();
        
        // Store updated escrow
        EscrowStorage::set(&env, &agreement_id, &escrow);
        
        // Emit event
        EventBuilder::escrow_funded(&env, &agreement_id, amount, &funder);
        
        Ok(escrow)
    }
    
    /// Lock funds in escrow (when agreement becomes active)
    pub fn lock_escrow(
        env: Env,
        agreement_id: String,
        locker: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        // Authorization check
        locker.require_auth();
        
        // Get agreement and escrow
        let agreement = AgreementStorage::get(&env, &agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
            
        let mut escrow = EscrowStorage::get(&env, &agreement_id)
            .ok_or(SettleError::EscrowNotFunded)?;
        
        // Check locker is a party
        if locker != agreement.creator && locker != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check escrow is funded
        if escrow.status != EscrowStatus::Funded {
            return Err(SettleError::EscrowNotFunded);
        }
        
        // Check agreement is active
        if agreement.status != AgreementStatus::Active {
            return Err(SettleError::AgreementNotActive);
        }
        
        // Check available balance
        let available = escrow.amount - escrow.locked_amount;
        if amount > available {
            return Err(SettleError::InsufficientEscrowBalance);
        }
        
        // Lock funds
        escrow.locked_amount += amount;
        escrow.status = EscrowStatus::Locked;
        escrow.last_action_at = env.ledger().timestamp();
        
        // Store updated escrow
        EscrowStorage::set(&env, &agreement_id, &escrow);
        
        // Emit event
        EventBuilder::escrow_locked(&env, &agreement_id, amount, &locker);
        
        Ok(escrow)
    }
    
    /// Release funds from escrow (milestone completion)
    pub fn release_escrow(
        env: Env,
        agreement_id: String,
        releaser: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        // Authorization check
        releaser.require_auth();
        
        // Validate inputs
        Validator::validate_amount(amount)?;
        Validator::validate_address(&env, &recipient)?;
        
        // Get agreement and escrow
        let agreement = AgreementStorage::get(&env, &agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
            
        let mut escrow = EscrowStorage::get(&env, &agreement_id)
            .ok_or(SettleError::EscrowNotFunded)?;
        
        // Check releaser is a party
        if releaser != agreement.creator && releaser != agreement.counterparty {
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check escrow has locked funds
        if escrow.locked_amount < amount {
            return Err(SettleError::InsufficientEscrowBalance);
        }
        
        // Release funds
        escrow.locked_amount -= amount;
        escrow.released_amount += amount;
        escrow.last_action_at = env.ledger().timestamp();
        
        // Update status
        if escrow.locked_amount == 0 && escrow.released_amount == escrow.amount {
            escrow.status = EscrowStatus::Released;
        }
        
        // Store updated escrow
        EscrowStorage::set(&env, &agreement_id, &escrow);
        
        // Emit event
        EventBuilder::escrow_released(&env, &agreement_id, amount, &recipient);
        
        Ok(escrow)
    }
    
    /// Refund funds from escrow (dispute resolution or cancellation)
    pub fn refund_escrow(
        env: Env,
        agreement_id: String,
        refunder: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<Escrow, SettleError> {
        // Authorization check
        refunder.require_auth();
        
        // Validate inputs
        Validator::validate_amount(amount)?;
        Validator::validate_address(&env, &recipient)?;
        
        // Get agreement and escrow
        let agreement = AgreementStorage::get(&env, &agreement_id)
            .ok_or(SettleError::AgreementNotFound)?;
            
        let mut escrow = EscrowStorage::get(&env, &agreement_id)
            .ok_or(SettleError::EscrowNotFunded)?;
        
        // Check refunder is authorized (agreement party or arbitrator)
        let is_party = refunder == agreement.creator || refunder == agreement.counterparty;
        if !is_party {
            // TODO: Check if refunder is arbitrator from dispute resolution
            return Err(SettleError::NotAgreementParty);
        }
        
        // Check agreement allows refunds (cancelled or resolved dispute)
        match agreement.status {
            AgreementStatus::Cancelled | AgreementStatus::Resolved => {},
            _ => return Err(SettleError::InvalidAgreementStatus),
        }
        
        // Check available balance (locked + unlocked funds)
        let available = escrow.amount - escrow.released_amount;
        if amount > available {
            return Err(SettleError::InsufficientEscrowBalance);
        }
        
        // Refund funds
        if amount <= escrow.locked_amount {
            escrow.locked_amount -= amount;
        } else {
            escrow.locked_amount = 0;
        }
        escrow.last_action_at = env.ledger().timestamp();
        escrow.status = EscrowStatus::Refunded;
        
        // Store updated escrow
        EscrowStorage::set(&env, &agreement_id, &escrow);
        
        // Emit event
        EventBuilder::escrow_refunded(&env, &agreement_id, amount, &recipient);
        
        Ok(escrow)
    }
    
    /// Get escrow by agreement ID
    pub fn get_escrow(env: Env, agreement_id: String) -> Option<Escrow> {
        EscrowStorage::get(&env, &agreement_id)
    }
    
    /// Get escrow balance information
    pub fn get_escrow_balance(env: Env, agreement_id: String) -> Result<(i128, i128, i128), SettleError> {
        let escrow = EscrowStorage::get(&env, &agreement_id)
            .ok_or(SettleError::EscrowNotFunded)?;
        
        let available = escrow.amount - escrow.locked_amount - escrow.released_amount;
        Ok((escrow.locked_amount, escrow.released_amount, available))
    }
}