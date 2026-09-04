//! GET/POST /v1/disputes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::app_state::AppState;

#[derive(Deserialize)]
struct OpenDisputeBody {
    agreement_id: String,
    reason: String,
    initial_evidence: String,
    disputed_amount: i64,
}

#[derive(Deserialize)]
struct ResolveBody {
    resolution: String,
    winner: String,
    compensation_amount: Option<i64>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(open_dispute).get(list_disputes))
        .route("/:id", get(get_dispute))
        .route("/:id/review", post(begin_review))
        .route("/:id/resolve", post(resolve_dispute))
}

/// POST /v1/disputes
async fn open_dispute(
    State(_state): State<AppState>,
    Json(body): Json<OpenDisputeBody>,
) -> (StatusCode, Json<Value>) {
    // TODO: Build Soroban transaction XDR, return for wallet signing
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "On-chain dispute opening not yet implemented. Soroban transaction construction is in progress.",
            "agreement_id": body.agreement_id,
            "reason": body.reason,
        })),
    )
}

/// GET /v1/disputes
async fn list_disputes(
    State(state): State<AppState>,
) -> Json<Value> {
    match state.dispute_service.list_all(50, 0).await {
        Ok(disputes) => {
            let responses: Vec<Value> = disputes
                .iter()
                .map(|d| {
                    json!({
                        "id": d.id.to_string(),
                        "on_chain_id": d.on_chain_dispute_id,
                        "agreement_id": d.agreement_id.to_string(),
                        "opener": d.opener_address,
                        "reason": d.reason,
                        "status": format!("{:?}", d.state),
                        "resolution": format!("{:?}", d.resolution),
                        "opened_at": d.opened_at.to_rfc3339(),
                        "resolved_at": d.resolved_at.map(|t| t.to_rfc3339()),
                    })
                })
                .collect();
            Json(json!({ "disputes": responses }))
        }
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// GET /v1/disputes/:id
async fn get_dispute(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    match state.dispute_service.get_by_chain_id(&id).await {
        Ok(d) => (
            StatusCode::OK,
            Json(json!({
                "id": d.id.to_string(),
                "on_chain_id": d.on_chain_dispute_id,
                "agreement_id": d.agreement_id.to_string(),
                "opener": d.opener_address,
                "reason": d.reason,
                "status": format!("{:?}", d.state),
                "resolution": format!("{:?}", d.resolution),
                "opened_at": d.opened_at.to_rfc3339(),
                "resolved_at": d.resolved_at.map(|t| t.to_rfc3339()),
            })),
        ),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

/// POST /v1/disputes/:id/review
async fn begin_review(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    // TODO: Build Soroban transaction XDR, return for wallet signing
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "On-chain review not yet implemented. Soroban transaction construction is in progress.",
            "dispute_id": id,
        })),
    )
}

/// POST /v1/disputes/:id/resolve
async fn resolve_dispute(
    State(_state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<ResolveBody>,
) -> (StatusCode, Json<Value>) {
    // TODO: Build Soroban transaction XDR, return for wallet signing
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "On-chain dispute resolution not yet implemented. Soroban transaction construction is in progress.",
            "dispute_id": id,
            "resolution": body.resolution,
            "winner": body.winner,
        })),
    )
}
