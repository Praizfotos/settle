//! GET/POST /v1/agreements

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};

// TODO: wire AppState with DB pool + service layer
pub fn router() -> Router {
    Router::new()
        .route("/",    post(create_agreement))
        .route("/",    get(list_agreements))
        .route("/:id", get(get_agreement))
        .route("/:id/fund",     post(fund_agreement))
        .route("/:id/activate", post(activate_agreement))
        .route("/:id/complete", post(complete_agreement))
}

/// POST /v1/agreements
/// Create a new draft agreement and optionally submit the funding tx.
async fn create_agreement() -> (StatusCode, Json<Value>) {
    // TODO: deserialize CreateAgreementRequest, call AgreementService::create
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "error": "not implemented" })))
}

/// GET /v1/agreements
async fn list_agreements() -> Json<Value> {
    // TODO: call AgreementService::list with pagination
    Json(json!({ "agreements": [] }))
}

/// GET /v1/agreements/:id
async fn get_agreement(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    // TODO: call AgreementService::get_by_id
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "id": id })))
}

/// POST /v1/agreements/:id/fund
async fn fund_agreement(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "id": id })))
}

/// POST /v1/agreements/:id/activate
async fn activate_agreement(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "id": id })))
}

/// POST /v1/agreements/:id/complete
async fn complete_agreement(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "id": id })))
}
