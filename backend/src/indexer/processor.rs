//! Processes decoded settlement events by updating database state and
//! triggering any necessary side effects.
//!
//! The processor is the "write side" of CQRS — it maintains the indexed
//! representation of on-chain state that powers the API queries.

use sqlx::PgPool;
use tracing::{info, warn, error};
use serde_json::json;

use crate::errors::{AppError, Result};
use crate::indexer::decoder::SettleEvent;

/// Process a decoded settlement event
pub async fn process(db_pool: &PgPool, event: SettleEvent) -> Result<()> {
    info!("Processing event: {:?}", event);

    // Store the event in settlement_events table for audit trail
    store_event(db_pool, &event).await?;

    // Update relevant domain tables based on event type
    match &event {
        SettleEvent::AgreementCreated { 
            agreement_id, 
            creator,
            counterparty,
            token,
            total_amount,
            expires_at,
            timestamp,
            ..
        } => {
            create_or_update_agreement(
                db_pool,
                agreement_id,
                creator,
                counterparty,
                token,
                *total_amount,
                *expires_at,
                *timestamp,
                "DRAFT",
            ).await?;
        },
        
        SettleEvent::AgreementFunded {
            agreement_id,
            amount,
            total_funded,
            ..
        } => {
            update_agreement_funding(db_pool, agreement_id, *total_funded, "FUNDED").await?;
        },
        
        SettleEvent::AgreementActivated {
            agreement_id,
            ..
        } => {
            update_agreement_status(db_pool, agreement_id, "ACTIVE").await?;
        },
        
        SettleEvent::AgreementCompleted {
            agreement_id,
            ..
        } => {
            update_agreement_status(db_pool, agreement_id, "COMPLETED").await?;
        },
        
        SettleEvent::AgreementExpired {
            agreement_id,
            ..
        } => {
            update_agreement_status(db_pool, agreement_id, "EXPIRED").await?;
        },
        
        SettleEvent::AgreementCancelled {
            agreement_id,
            ..
        } => {
            update_agreement_status(db_pool, agreement_id, "CANCELLED").await?;
        },
        
        SettleEvent::MilestoneCreated {
            milestone_id,
            agreement_id,
            creator,
            name,
            amount,
            due_date,
            timestamp,
            ..
        } => {
            create_or_update_milestone(
                db_pool,
                milestone_id,
                agreement_id,
                name,
                "",
                *amount,
                *due_date,
                *timestamp,
                "PENDING",
            ).await?;
        },
        
        SettleEvent::MilestoneSubmitted {
            milestone_id,
            evidence,
            timestamp,
            ..
        } => {
            update_milestone_submission(db_pool, milestone_id, evidence, *timestamp, "SUBMITTED").await?;
        },
        
        SettleEvent::MilestoneApproved {
            milestone_id,
            timestamp,
            ..
        } => {
            update_milestone_approval(db_pool, milestone_id, *timestamp, "APPROVED").await?;
        },
        
        SettleEvent::MilestoneRejected {
            milestone_id,
            ..
        } => {
            update_milestone_status(db_pool, milestone_id, "REJECTED").await?;
        },
        
        SettleEvent::MilestoneReleased {
            milestone_id,
            ..
        } => {
            update_milestone_status(db_pool, milestone_id, "RELEASED").await?;
        },
        
        SettleEvent::DisputeOpened {
            dispute_id,
            agreement_id,
            opener,
            reason,
            timestamp,
            ..
        } => {
            create_or_update_dispute(
                db_pool,
                dispute_id,
                agreement_id,
                opener,
                reason,
                *timestamp,
                "OPEN",
            ).await?;
            
            // Also update the agreement status
            update_agreement_status(db_pool, agreement_id, "DISPUTED").await?;
        },
        
        SettleEvent::EvidenceSubmitted {
            dispute_id,
            evidence,
            ..
        } => {
            add_dispute_evidence(db_pool, dispute_id, evidence).await?;
            update_dispute_status(db_pool, dispute_id, "EVIDENCE_SUBMISSION").await?;
        },
        
        SettleEvent::DisputeResolved {
            dispute_id,
            agreement_id,
            arbitrator,
            resolution,
            timestamp,
            ..
        } => {
            resolve_dispute(db_pool, dispute_id, arbitrator, resolution, *timestamp).await?;
            update_agreement_status(db_pool, agreement_id, "RESOLVED").await?;
        },
        
        SettleEvent::DisputeClosed {
            dispute_id,
            ..
        } => {
            update_dispute_status(db_pool, dispute_id, "CLOSED").await?;
        },
        
        _ => {
            warn!("Unhandled event type: {:?}", event);
        }
    }

    Ok(())
}

/// Store event in settlement_events table for audit trail
async fn store_event(db_pool: &PgPool, event: &SettleEvent) -> Result<()> {
    let (event_type, agreement_id, milestone_id, dispute_id, participant, block_height, tx_hash) = match event {
        SettleEvent::AgreementCreated { agreement_id, creator, ledger, tx_hash, .. } => {
            ("AgreementCreated", Some(agreement_id.clone()), None, None, creator.clone(), *ledger as i64, tx_hash.clone())
        },
        SettleEvent::AgreementFunded { agreement_id, funder, ledger, tx_hash, .. } => {
            ("AgreementFunded", Some(agreement_id.clone()), None, None, funder.clone(), *ledger as i64, tx_hash.clone())
        },
        SettleEvent::MilestoneSubmitted { agreement_id, milestone_id, submitter, ledger, tx_hash, .. } => {
            ("MilestoneSubmitted", Some(agreement_id.clone()), Some(milestone_id.clone()), None, submitter.clone(), *ledger as i64, tx_hash.clone())
        },
        SettleEvent::DisputeOpened { agreement_id, dispute_id, opener, ledger, tx_hash, .. } => {
            ("DisputeOpened", Some(agreement_id.clone()), None, Some(dispute_id.clone()), opener.clone(), *ledger as i64, tx_hash.clone())
        },
        _ => {
            return Ok(()); // Skip storing other event types for now
        }
    };

    sqlx::query(
        r#"
        INSERT INTO settlement_events (event_type, agreement_id, milestone_id, dispute_id, participant, data, timestamp, block_height, transaction_hash)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), $7, $8)
        "#
    )
    .bind(event_type)
    .bind(agreement_id)
    .bind(milestone_id)
    .bind(dispute_id)
    .bind(participant)
    .bind(json!({})) // For now, store empty JSON. In production, store full event data
    .bind(block_height)
    .bind(tx_hash)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

// Database update functions

async fn create_or_update_agreement(
    db_pool: &PgPool,
    agreement_id: &str,
    creator: &str,
    counterparty: &str,
    token: &str,
    total_amount: i128,
    expires_at: u64,
    timestamp: u64,
    status: &str,
) -> Result<()> {
    let expires_at_dt = chrono::DateTime::from_timestamp(expires_at as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .naive_utc();
    let created_at_dt = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .naive_utc();

    sqlx::query(
        r#"
        INSERT INTO agreements (on_chain_id, creator, counterparty, token, total_amount, status, created_at, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (on_chain_id) 
        DO UPDATE SET 
            status = EXCLUDED.status,
            updated_at = NOW()
        "#
    )
    .bind(agreement_id)
    .bind(creator)
    .bind(counterparty)
    .bind(token)
    .bind(total_amount as i64)
    .bind(status)
    .bind(created_at_dt)
    .bind(expires_at_dt)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn update_agreement_funding(db_pool: &PgPool, agreement_id: &str, funded_amount: i128, status: &str) -> Result<()> {
    sqlx::query(
        "UPDATE agreements SET funded_amount = $1, status = $2, updated_at = NOW() WHERE on_chain_id = $3"
    )
    .bind(funded_amount as i64)
    .bind(status)
    .bind(agreement_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn update_agreement_status(db_pool: &PgPool, agreement_id: &str, status: &str) -> Result<()> {
    sqlx::query(
        "UPDATE agreements SET status = $1, updated_at = NOW() WHERE on_chain_id = $2"
    )
    .bind(status)
    .bind(agreement_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn create_or_update_milestone(
    db_pool: &PgPool,
    milestone_id: &str,
    agreement_id: &str,
    name: &str,
    description: &str,
    amount: i128,
    due_date: u64,
    _timestamp: u64,
    status: &str,
) -> Result<()> {
    let due_date_dt = chrono::DateTime::from_timestamp(due_date as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .naive_utc();

    sqlx::query(
        r#"
        INSERT INTO milestones (on_chain_id, agreement_id, name, description, amount, status, due_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (on_chain_id)
        DO UPDATE SET
            status = EXCLUDED.status,
            updated_at = NOW()
        "#
    )
    .bind(milestone_id)
    .bind(agreement_id)
    .bind(name)
    .bind(description)
    .bind(amount as i64)
    .bind(status)
    .bind(due_date_dt)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn update_milestone_submission(db_pool: &PgPool, milestone_id: &str, evidence: &str, timestamp: u64, status: &str) -> Result<()> {
    let submitted_at_dt = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .naive_utc();

    sqlx::query(
        "UPDATE milestones SET evidence = $1, submitted_at = $2, status = $3, updated_at = NOW() WHERE on_chain_id = $4"
    )
    .bind(evidence)
    .bind(submitted_at_dt)
    .bind(status)
    .bind(milestone_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn update_milestone_approval(db_pool: &PgPool, milestone_id: &str, timestamp: u64, status: &str) -> Result<()> {
    let approved_at_dt = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .naive_utc();

    sqlx::query(
        "UPDATE milestones SET approved_at = $1, status = $2, updated_at = NOW() WHERE on_chain_id = $3"
    )
    .bind(approved_at_dt)
    .bind(status)
    .bind(milestone_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn update_milestone_status(db_pool: &PgPool, milestone_id: &str, status: &str) -> Result<()> {
    sqlx::query(
        "UPDATE milestones SET status = $1, updated_at = NOW() WHERE on_chain_id = $2"
    )
    .bind(status)
    .bind(milestone_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn create_or_update_dispute(
    db_pool: &PgPool,
    dispute_id: &str,
    agreement_id: &str,
    opener: &str,
    reason: &str,
    timestamp: u64,
    status: &str,
) -> Result<()> {
    let opened_at_dt = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .naive_utc();

    sqlx::query(
        r#"
        INSERT INTO disputes (on_chain_id, agreement_id, opened_by, reason, status, opened_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (on_chain_id)
        DO UPDATE SET
            status = EXCLUDED.status,
            updated_at = NOW()
        "#
    )
    .bind(dispute_id)
    .bind(agreement_id)
    .bind(opener)
    .bind(reason)
    .bind(status)
    .bind(opened_at_dt)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn add_dispute_evidence(db_pool: &PgPool, dispute_id: &str, evidence: &str) -> Result<()> {
    sqlx::query(
        "UPDATE disputes SET evidence = array_append(evidence, $1), updated_at = NOW() WHERE on_chain_id = $2"
    )
    .bind(evidence)
    .bind(dispute_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn update_dispute_status(db_pool: &PgPool, dispute_id: &str, status: &str) -> Result<()> {
    sqlx::query(
        "UPDATE disputes SET status = $1, updated_at = NOW() WHERE on_chain_id = $2"
    )
    .bind(status)
    .bind(dispute_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

async fn resolve_dispute(db_pool: &PgPool, dispute_id: &str, arbitrator: &str, resolution: &str, timestamp: u64) -> Result<()> {
    let resolved_at_dt = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .naive_utc();

    sqlx::query(
        "UPDATE disputes SET arbitrator = $1, resolution = $2, resolved_at = $3, status = 'RESOLVED', updated_at = NOW() WHERE on_chain_id = $4"
    )
    .bind(arbitrator)
    .bind(resolution)
    .bind(resolved_at_dt)
    .bind(dispute_id)
    .execute(db_pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}