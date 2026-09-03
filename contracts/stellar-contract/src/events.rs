use soroban_sdk::{Bytes, Env, Address, String};

use crate::types::{Agreement, Milestone, Dispute, EventType, SettleEvent};

/// Event builder for structured event emission
pub struct EventBuilder;

impl EventBuilder {
    /// Create and emit a structured event
    fn emit_event(
        env: &Env,
        event_type: EventType,
        agreement_id: Option<String>,
        milestone_id: Option<String>,
        dispute_id: Option<String>,
        participant: &Address,
        data: Bytes,
    ) {
        let event = SettleEvent {
            event_type: event_type.clone(),
            agreement_id,
            milestone_id,
            dispute_id,
            participant: participant.clone(),
            data,
            timestamp: env.ledger().timestamp(),
            block_height: env.ledger().sequence(),
        };
        
        env.events().publish((
            "settle_event",
            event_type,
            participant,
        ), event);
    }
    
    // Agreement events
    pub fn agreement_created(env: &Env, agreement: &Agreement) {
        let data = Bytes::new(env); // Could serialize additional data
        Self::emit_event(
            env,
            EventType::AgreementCreated,
            Some(agreement.id.clone()),
            None,
            None,
            &agreement.creator,
            data,
        );
    }
    
    pub fn agreement_funded(env: &Env, agreement: &Agreement) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::AgreementFunded,
            Some(agreement.id.clone()),
            None,
            None,
            &agreement.creator,
            data,
        );
    }
    
    pub fn agreement_activated(env: &Env, agreement: &Agreement) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::AgreementActivated,
            Some(agreement.id.clone()),
            None,
            None,
            &agreement.creator,
            data,
        );
    }
    
    pub fn agreement_completed(env: &Env, agreement: &Agreement) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::AgreementCompleted,
            Some(agreement.id.clone()),
            None,
            None,
            &agreement.creator,
            data,
        );
    }
    
    pub fn agreement_cancelled(env: &Env, agreement: &Agreement) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::AgreementCancelled,
            Some(agreement.id.clone()),
            None,
            None,
            &agreement.creator,
            data,
        );
    }
    
    pub fn agreement_expired(env: &Env, agreement: &Agreement) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::AgreementExpired,
            Some(agreement.id.clone()),
            None,
            None,
            &agreement.creator,
            data,
        );
    }
    
    // Milestone events
    pub fn milestone_created(env: &Env, milestone: &Milestone) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::MilestoneCreated,
            Some(milestone.agreement_id.clone()),
            Some(milestone.id.clone()),
            None,
            &Address::from_string(&String::from_str(env, "creator")), // TODO: Get from context
            data,
        );
    }
    
    pub fn milestone_submitted(env: &Env, milestone: &Milestone) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::MilestoneSubmitted,
            Some(milestone.agreement_id.clone()),
            Some(milestone.id.clone()),
            None,
            &Address::from_string(&String::from_str(env, "submitter")), // TODO: Get from context
            data,
        );
    }
    
    pub fn milestone_approved(env: &Env, milestone: &Milestone) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::MilestoneApproved,
            Some(milestone.agreement_id.clone()),
            Some(milestone.id.clone()),
            None,
            &Address::from_string(&String::from_str(env, "approver")), // TODO: Get from context
            data,
        );
    }
    
    pub fn milestone_rejected(env: &Env, milestone: &Milestone) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::MilestoneRejected,
            Some(milestone.agreement_id.clone()),
            Some(milestone.id.clone()),
            None,
            &Address::from_string(&String::from_str(env, "rejecter")), // TODO: Get from context
            data,
        );
    }
    
    pub fn milestone_released(env: &Env, milestone: &Milestone, recipient: &Address) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::MilestoneReleased,
            Some(milestone.agreement_id.clone()),
            Some(milestone.id.clone()),
            None,
            recipient,
            data,
        );
    }
    
    // Escrow events
    pub fn escrow_funded(env: &Env, agreement_id: &String, amount: i128) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::EscrowFunded,
            Some(agreement_id.clone()),
            None,
            None,
            &Address::from_string(&String::from_str(env, "funder")), // TODO: Get from context
            data,
        );
    }
    
    pub fn escrow_locked(env: &Env, agreement_id: &String, amount: i128) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::EscrowLocked,
            Some(agreement_id.clone()),
            None,
            None,
            &Address::from_string(&String::from_str(env, "locker")), // TODO: Get from context
            data,
        );
    }
    
    pub fn escrow_released(env: &Env, agreement_id: &String, amount: i128, recipient: &Address) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::EscrowReleased,
            Some(agreement_id.clone()),
            None,
            None,
            recipient,
            data,
        );
    }
    
    pub fn escrow_refunded(env: &Env, agreement_id: &String, amount: i128, recipient: &Address) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::EscrowRefunded,
            Some(agreement_id.clone()),
            None,
            None,
            recipient,
            data,
        );
    }
    
    // Dispute events
    pub fn dispute_opened(env: &Env, dispute: &Dispute) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::DisputeOpened,
            Some(dispute.agreement_id.clone()),
            None,
            Some(dispute.id.clone()),
            &dispute.opened_by,
            data,
        );
    }
    
    pub fn evidence_submitted(env: &Env, dispute: &Dispute, submitter: &Address) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::EvidenceSubmitted,
            Some(dispute.agreement_id.clone()),
            None,
            Some(dispute.id.clone()),
            submitter,
            data,
        );
    }
    
    pub fn dispute_resolved(env: &Env, dispute: &Dispute, winner: &Address, amount: i128) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::DisputeResolved,
            Some(dispute.agreement_id.clone()),
            None,
            Some(dispute.id.clone()),
            winner,
            data,
        );
    }
    
    pub fn dispute_closed(env: &Env, dispute: &Dispute) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::DisputeClosed,
            Some(dispute.agreement_id.clone()),
            None,
            Some(dispute.id.clone()),
            &dispute.opened_by,
            data,
        );
    }
    
    // Reputation events
    pub fn reputation_updated(env: &Env, participant: &Address, old_score: u32, new_score: u32) {
        let data = Bytes::new(env);
        Self::emit_event(
            env,
            EventType::ReputationUpdated,
            None,
            None,
            None,
            participant,
            data,
        );
    }
}