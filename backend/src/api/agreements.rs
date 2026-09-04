//! GET/POST /v1/agreements

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct ListParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    pub address: Option<String>,
}

fn default_limit() -> i64 {
    20
}

#[derive(Deserialize)]
pub struct CreateAgreementBody {
    pub counterparty: String,
    pub token: String,
    pub total_amount: i64,
    pub title: Option<String>,
    pub expires_at_ledger: Option<i64>,
}

#[derive(Serialize)]
pub struct AgreementResponse {
    pub id: String,
    pub on_chain_id: String,
    pub client: String,
    pub provider: String,
    pub token: String,
    pub total_amount: i64,
    pub funded_amount: i64,
    pub released_amount: i64,
    pub refunded_amount: i64,
    pub title: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub tx_hash: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_agreement).get(list_agreements))
        .route("/:id", get(get_agreement))
        .route("/:id/fund", post(fund_agreement))
        .route("/:id/activate", post(activate_agreement))
        .route("/:id/complete", post(complete_agreement))
}

/// POST /v1/agreements
async fn create_agreement(
    State(state): State<AppState>,
    Json(body): Json<CreateAgreementBody>,
) -> (StatusCode, Json<Value>) {
    // Agreement creation requires a wallet-signed transaction.
    // The backend receives the tx_xdr from the frontend and submits it.
    // For now, we accept the basic info and return a draft agreement.
    let client_address = body
        .counterparty
        .clone(); // In real flow, this comes from auth/wallet

    match state
        .agreement_service
        .handle_created_event(
            &uuid::Uuid::new_v4().to_string(),
            &client_address,
            &body.counterparty,
            &body.token,
            body.total_amount,
            "DRAFT",
        )
        .await
    {
        Ok(()) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "draft",
                "message": "Agreement created. Sign the Stellar transaction to finalize.",
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

/// GET /v1/agreements
async fn list_agreements(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<Value> {
    let result = if let Some(address) = &params.address {
        state
            .agreement_service
            .list_for_address(address, params.limit, params.offset)
            .await
    } else {
        state
            .agreement_service
            .list_all(params.limit, params.offset)
            .await
    };

    match result {
        Ok(agreements) => {
            let responses: Vec<Value> = agreements
                .iter()
                .map(|a| {
                    json!({
                        "id": a.id.to_string(),
                        "on_chain_id": a.on_chain_id,
                        "client": a.creator,
                        "provider": a.counterparty,
                        "token": a.token_address,
                        "total_amount": a.total_amount,
                        "funded_amount": a.escrowed_amount,
                        "released_amount": a.released_amount,
                        "refunded_amount": a.refunded_amount,
                        "title": a.title,
                        "status": format!("{:?}", a.state).to_uppercase(),
                        "created_at": a.created_at.to_rfc3339(),
                        "updated_at": a.updated_at.to_rfc3339(),
                        "tx_hash": a.tx_hash,
                    })
                })
                .collect();

            Json(json!({ "agreements": responses }))
        }
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// GET /v1/agreements/:id
async fn get_agreement(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    match state.agreement_service.get_by_chain_id(&id).await {
        Ok(a) => (
            StatusCode::OK,
            Json(json!({
                "id": a.id.to_string(),
                "on_chain_id": a.on_chain_id,
                "client": a.creator,
                "provider": a.counterparty,
                "token": a.token_address,
                "total_amount": a.total_amount,
                "funded_amount": a.escrowed_amount,
                "released_amount": a.released_amount,
                "refunded_amount": a.refunded_amount,
                "title": a.title,
                "status": format!("{:?}", a.state).to_uppercase(),
                "created_at": a.created_at.to_rfc3339(),
                "updated_at": a.updated_at.to_rfc3339(),
                "tx_hash": a.tx_hash,
            })),
        ),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

/// POST /v1/agreements/:id/fund
async fn fund_agreement(
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    // TODO: Build Soroban transaction XDR, return for wallet signing
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "On-chain funding not yet implemented. Soroban transaction construction is in progress.",
            "agreement_id": id,
        })),
    )
}

/// POST /v1/agreements/:id/activate
async fn activate_agreement(
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    // TODO: Build Soroban transaction XDR, return for wallet signing
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "On-chain activation not yet implemented. Soroban transaction construction is in progress.",
            "agreement_id": id,
        })),
    )
}

/// POST /v1/agreements/:id/complete
async fn complete_agreement(
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    // TODO: Build Soroban transaction XDR, return for wallet signing
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "On-chain completion not yet implemented. Soroban transaction construction is in progress.",
            "agreement_id": id,
        })),
    )
}
