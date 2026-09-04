use core::cmp;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

use crate::types::{Agreement, Milestone, Dispute, Escrow, Participant};

/// Storage key types for different data structures
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Agreement(String),
    Milestone(String),
    Dispute(String),
    Escrow(String),
    Participant(Address),
    
    // Index keys
    AgreementsByParticipant(Address),
    MilestonesByAgreement(String),
    DisputesByParticipant(Address),
    AgreementDispute(String), // agreement_id -> dispute_id mapping
    
    // Counters and metadata
    TotalAgreements,
    TotalMilestones,
    TotalDisputes,
    ContractVersion,
    
    // Authorization and upgrade management
    AdminList,
    ContractPaused,
    DeployedAt,
    
    // Feature flags for gradual rollout
    FeatureFlags,
}

/// Agreement storage operations
pub struct AgreementStorage;

impl AgreementStorage {
    pub fn get(env: &Env, id: &String) -> Option<Agreement> {
        env.storage().persistent().get(&DataKey::Agreement(id.clone()))
    }
    
    pub fn set(env: &Env, id: &String, agreement: &Agreement) {
        env.storage().persistent().set(&DataKey::Agreement(id.clone()), agreement);
        
        // Update indexes
        Self::add_to_participant_index(env, &agreement.creator, id);
        Self::add_to_participant_index(env, &agreement.counterparty, id);
        
        // Update counter
        Self::increment_total(env);
    }
    
    pub fn exists(env: &Env, id: &String) -> bool {
        env.storage().persistent().has(&DataKey::Agreement(id.clone()))
    }
    
    pub fn get_by_participant(env: &Env, participant: &Address, limit: u32, offset: u32) -> Vec<Agreement> {
        let key = DataKey::AgreementsByParticipant(participant.clone());
        let agreement_ids: Vec<String> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
        
        let mut agreements = Vec::new(env);
        let start = offset as usize;
        let end = cmp::min(start + limit as usize, agreement_ids.len() as usize);
        
        for i in start..end {
            if let Some(agreement) = Self::get(env, &agreement_ids.get(i as u32).unwrap()) {
                agreements.push_back(agreement);
            }
        }
        
        agreements
    }
    
    fn add_to_participant_index(env: &Env, participant: &Address, agreement_id: &String) {
        let key = DataKey::AgreementsByParticipant(participant.clone());
        let mut agreements: Vec<String> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
        agreements.push_back(agreement_id.clone());
        env.storage().persistent().set(&key, &agreements);
    }
    
    fn increment_total(env: &Env) {
        let key = DataKey::TotalAgreements;
        let current: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + 1));
    }
}

/// Milestone storage operations
pub struct MilestoneStorage;

impl MilestoneStorage {
    pub fn get(env: &Env, id: &String) -> Option<Milestone> {
        env.storage().persistent().get(&DataKey::Milestone(id.clone()))
    }
    
    pub fn set(env: &Env, id: &String, milestone: &Milestone) {
        env.storage().persistent().set(&DataKey::Milestone(id.clone()), milestone);
        
        // Update agreement index
        Self::add_to_agreement_index(env, &milestone.agreement_id, id);
        
        // Update counter
        Self::increment_total(env);
    }
    
    pub fn exists(env: &Env, id: &String) -> bool {
        env.storage().persistent().has(&DataKey::Milestone(id.clone()))
    }
    
    pub fn get_by_agreement(env: &Env, agreement_id: &String, limit: u32, offset: u32) -> Vec<Milestone> {
        let key = DataKey::MilestonesByAgreement(agreement_id.clone());
        let milestone_ids: Vec<String> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
        
        let mut milestones = Vec::new(env);
        let start = offset as usize;
        let end = cmp::min(start + limit as usize, milestone_ids.len() as usize);
        
        for i in start..end {
            if let Some(milestone) = Self::get(env, &milestone_ids.get(i as u32).unwrap()) {
                milestones.push_back(milestone);
            }
        }
        
        milestones
    }
    
    fn add_to_agreement_index(env: &Env, agreement_id: &String, milestone_id: &String) {
        let key = DataKey::MilestonesByAgreement(agreement_id.clone());
        let mut milestones: Vec<String> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
        milestones.push_back(milestone_id.clone());
        env.storage().persistent().set(&key, &milestones);
    }
    
    fn increment_total(env: &Env) {
        let key = DataKey::TotalMilestones;
        let current: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + 1));
    }
}

/// Dispute storage operations
pub struct DisputeStorage;

impl DisputeStorage {
    pub fn get(env: &Env, id: &String) -> Option<Dispute> {
        env.storage().persistent().get(&DataKey::Dispute(id.clone()))
    }
    
    pub fn set(env: &Env, id: &String, dispute: &Dispute) {
        env.storage().persistent().set(&DataKey::Dispute(id.clone()), dispute);
        
        // Update participant indexes
        Self::add_to_participant_index(env, &dispute.opened_by, id);
        
        // Update counter
        Self::increment_total(env);
    }
    
    pub fn exists_for_agreement(env: &Env, agreement_id: &String) -> bool {
        let key = DataKey::AgreementDispute(agreement_id.clone());
        env.storage().persistent().has(&key)
    }
    
    pub fn set_agreement_dispute(env: &Env, agreement_id: &String, dispute_id: &String) {
        let key = DataKey::AgreementDispute(agreement_id.clone());
        env.storage().persistent().set(&key, dispute_id);
    }
    
    pub fn get_agreement_dispute(env: &Env, agreement_id: &String) -> Option<String> {
        let key = DataKey::AgreementDispute(agreement_id.clone());
        env.storage().persistent().get(&key)
    }
    
    pub fn get_by_participant(env: &Env, participant: &Address, limit: u32, offset: u32) -> Vec<Dispute> {
        let key = DataKey::DisputesByParticipant(participant.clone());
        let dispute_ids: Vec<String> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
        
        let mut disputes = Vec::new(env);
        let start = offset as usize;
        let end = cmp::min(start + limit as usize, dispute_ids.len() as usize);
        
        for i in start..end {
            if let Some(dispute) = Self::get(env, &dispute_ids.get(i as u32).unwrap()) {
                disputes.push_back(dispute);
            }
        }
        
        disputes
    }
    
    fn add_to_participant_index(env: &Env, participant: &Address, dispute_id: &String) {
        let key = DataKey::DisputesByParticipant(participant.clone());
        let mut disputes: Vec<String> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));
        disputes.push_back(dispute_id.clone());
        env.storage().persistent().set(&key, &disputes);
    }
    
    fn increment_total(env: &Env) {
        let key = DataKey::TotalDisputes;
        let current: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + 1));
    }
}

/// Escrow storage operations
pub struct EscrowStorage;

impl EscrowStorage {
    pub fn get(env: &Env, agreement_id: &String) -> Option<Escrow> {
        env.storage().persistent().get(&DataKey::Escrow(agreement_id.clone()))
    }
    
    pub fn set(env: &Env, agreement_id: &String, escrow: &Escrow) {
        env.storage().persistent().set(&DataKey::Escrow(agreement_id.clone()), escrow);
    }
    
    pub fn exists(env: &Env, agreement_id: &String) -> bool {
        env.storage().persistent().has(&DataKey::Escrow(agreement_id.clone()))
    }
}

/// Participant storage operations
pub struct ParticipantStorage;

impl ParticipantStorage {
    pub fn get(env: &Env, address: &Address) -> Option<Participant> {
        env.storage().persistent().get(&DataKey::Participant(address.clone()))
    }
    
    pub fn set(env: &Env, address: &Address, participant: &Participant) {
        env.storage().persistent().set(&DataKey::Participant(address.clone()), participant);
    }
    
    pub fn exists(env: &Env, address: &Address) -> bool {
        env.storage().persistent().has(&DataKey::Participant(address.clone()))
    }
    
    pub fn create_if_not_exists(env: &Env, address: &Address) {
        if !Self::exists(env, address) {
            let participant = Participant {
                address: address.clone(),
                reputation_score: 100, // Start with neutral score
                total_agreements: 0,
                successful_agreements: 0,
                disputed_agreements: 0,
                total_volume: 0,
                joined_at: env.ledger().timestamp(),
                last_activity: env.ledger().timestamp(),
            };
            Self::set(env, address, &participant);
        }
    }
}