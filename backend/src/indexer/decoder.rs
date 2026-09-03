//! Decodes raw Stellar contract events (XDR-encoded topics + data) into
//! typed `SettleEvent` variants that the processor can act on.
//!
//! Events follow the structure defined in the stellar-contract events.rs module.

use crate::errors::{AppError, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// Raw event data from Stellar RPC
#[derive(Debug, Clone)]
pub struct RawEvent {
    pub ledger: u32,
    pub transaction_hash: String,
    pub contract_id: String,
    pub topics: Vec<String>,
    pub data: String,
}

/// All contract events emitted by Settle contracts.
/// This matches the EventType enum from stellar-contract/src/types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SettleEvent {
    // Agreement events
    AgreementCreated {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        creator: String,
        counterparty: String,
        token: String,
        total_amount: i128,
        expires_at: u64,
        timestamp: u64,
    },
    AgreementFunded {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        funder: String,
        amount: i128,
        total_funded: i128,
        timestamp: u64,
    },
    AgreementActivated {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        activator: String,
        timestamp: u64,
    },
    AgreementCompleted {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        completer: String,
        timestamp: u64,
    },
    AgreementExpired {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        timestamp: u64,
    },
    AgreementCancelled {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        canceller: String,
        timestamp: u64,
    },

    // Milestone events
    MilestoneCreated {
        ledger: u32,
        tx_hash: String,
        milestone_id: String,
        agreement_id: String,
        creator: String,
        name: String,
        amount: i128,
        due_date: u64,
        timestamp: u64,
    },
    MilestoneSubmitted {
        ledger: u32,
        tx_hash: String,
        milestone_id: String,
        agreement_id: String,
        submitter: String,
        evidence: String,
        timestamp: u64,
    },
    MilestoneApproved {
        ledger: u32,
        tx_hash: String,
        milestone_id: String,
        agreement_id: String,
        approver: String,
        timestamp: u64,
    },
    MilestoneRejected {
        ledger: u32,
        tx_hash: String,
        milestone_id: String,
        agreement_id: String,
        rejecter: String,
        reason: String,
        timestamp: u64,
    },
    MilestoneReleased {
        ledger: u32,
        tx_hash: String,
        milestone_id: String,
        agreement_id: String,
        releaser: String,
        recipient: String,
        amount: i128,
        timestamp: u64,
    },

    // Escrow events
    EscrowFunded {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        funder: String,
        amount: i128,
        total_amount: i128,
        timestamp: u64,
    },
    EscrowLocked {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        locker: String,
        amount: i128,
        timestamp: u64,
    },
    EscrowReleased {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        releaser: String,
        recipient: String,
        amount: i128,
        timestamp: u64,
    },
    EscrowRefunded {
        ledger: u32,
        tx_hash: String,
        agreement_id: String,
        refunder: String,
        recipient: String,
        amount: i128,
        timestamp: u64,
    },

    // Dispute events
    DisputeOpened {
        ledger: u32,
        tx_hash: String,
        dispute_id: String,
        agreement_id: String,
        opener: String,
        reason: String,
        timestamp: u64,
    },
    EvidenceSubmitted {
        ledger: u32,
        tx_hash: String,
        dispute_id: String,
        agreement_id: String,
        submitter: String,
        evidence: String,
        timestamp: u64,
    },
    DisputeResolved {
        ledger: u32,
        tx_hash: String,
        dispute_id: String,
        agreement_id: String,
        arbitrator: String,
        resolution: String,
        winner: String,
        compensation_amount: i128,
        timestamp: u64,
    },
    DisputeClosed {
        ledger: u32,
        tx_hash: String,
        dispute_id: String,
        agreement_id: String,
        closer: String,
        timestamp: u64,
    },

    // Reputation events  
    ReputationUpdated {
        ledger: u32,
        tx_hash: String,
        participant: String,
        new_score: u32,
        reason: String,
        timestamp: u64,
    },
}

/// Decode a raw event from Stellar into a structured SettleEvent
pub fn decode(raw_event: RawEvent) -> Result<SettleEvent> {
    debug!("Decoding event with topics: {:?}", raw_event.topics);

    // Events are published with topics: ["settle_event", event_type, participant]  
    if raw_event.topics.len() < 3 {
        return Err(AppError::InvalidInput(
            "Event must have at least 3 topics".to_string()
        ));
    }

    let event_namespace = &raw_event.topics[0];
    let event_type = &raw_event.topics[1];
    let participant = &raw_event.topics[2];

    if event_namespace != "settle_event" {
        return Err(AppError::InvalidInput(
            format!("Unknown event namespace: {}", event_namespace)
        ));
    }

    // For MVP, we'll implement basic event decoding
    // In production, this would parse the XDR-encoded data field
    match event_type.as_str() {
        "AgreementCreated" => decode_agreement_created(raw_event, participant),
        "AgreementFunded" => decode_agreement_funded(raw_event, participant),
        "AgreementActivated" => decode_agreement_activated(raw_event, participant),
        "MilestoneSubmitted" => decode_milestone_submitted(raw_event, participant),
        "MilestoneApproved" => decode_milestone_approved(raw_event, participant),
        "DisputeOpened" => decode_dispute_opened(raw_event, participant),
        _ => {
            warn!("Unknown event type: {}", event_type);
            Err(AppError::InvalidInput(format!("Unknown event type: {}", event_type)))
        }
    }
}

// Helper functions for decoding specific event types
// These would parse the actual XDR data field in production

fn decode_agreement_created(raw_event: RawEvent, participant: &str) -> Result<SettleEvent> {
    Ok(SettleEvent::AgreementCreated {
        ledger: raw_event.ledger,
        tx_hash: raw_event.transaction_hash,
        agreement_id: "placeholder_agreement_id".to_string(), // Parse from data
        creator: participant.to_string(),
        counterparty: "placeholder_counterparty".to_string(), // Parse from data
        token: "USDC".to_string(), // Parse from data
        total_amount: 1000_0000000, // Parse from data (7 decimals)
        expires_at: 0, // Parse from data
        timestamp: 0, // Parse from data
    })
}

fn decode_agreement_funded(raw_event: RawEvent, participant: &str) -> Result<SettleEvent> {
    Ok(SettleEvent::AgreementFunded {
        ledger: raw_event.ledger,
        tx_hash: raw_event.transaction_hash,
        agreement_id: "placeholder_agreement_id".to_string(),
        funder: participant.to_string(),
        amount: 1000_0000000,
        total_funded: 1000_0000000,
        timestamp: 0,
    })
}

fn decode_agreement_activated(raw_event: RawEvent, participant: &str) -> Result<SettleEvent> {
    Ok(SettleEvent::AgreementActivated {
        ledger: raw_event.ledger,
        tx_hash: raw_event.transaction_hash,
        agreement_id: "placeholder_agreement_id".to_string(),
        activator: participant.to_string(),
        timestamp: 0,
    })
}

fn decode_milestone_submitted(raw_event: RawEvent, participant: &str) -> Result<SettleEvent> {
    Ok(SettleEvent::MilestoneSubmitted {
        ledger: raw_event.ledger,
        tx_hash: raw_event.transaction_hash,
        milestone_id: "placeholder_milestone_id".to_string(),
        agreement_id: "placeholder_agreement_id".to_string(),
        submitter: participant.to_string(),
        evidence: "placeholder_evidence".to_string(),
        timestamp: 0,
    })
}

fn decode_milestone_approved(raw_event: RawEvent, participant: &str) -> Result<SettleEvent> {
    Ok(SettleEvent::MilestoneApproved {
        ledger: raw_event.ledger,
        tx_hash: raw_event.transaction_hash,
        milestone_id: "placeholder_milestone_id".to_string(),
        agreement_id: "placeholder_agreement_id".to_string(),
        approver: participant.to_string(),
        timestamp: 0,
    })
}

fn decode_dispute_opened(raw_event: RawEvent, participant: &str) -> Result<SettleEvent> {
    Ok(SettleEvent::DisputeOpened {
        ledger: raw_event.ledger,
        tx_hash: raw_event.transaction_hash,
        dispute_id: "placeholder_dispute_id".to_string(),
        agreement_id: "placeholder_agreement_id".to_string(),
        opener: participant.to_string(),
        reason: "placeholder_reason".to_string(),
        timestamp: 0,
    })
}
