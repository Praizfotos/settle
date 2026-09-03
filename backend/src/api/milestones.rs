use axum::{extract::Path, http::StatusCode, response::Json, routing::{get, post}, Router};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/",                    post(add_milestone))
        .route("/:agreement_id",       get(list_milestones))
        .route("/:agreement_id/:idx/submit",  post(submit_milestone))
        .route("/:agreement_id/:idx/approve", post(approve_milestone))
        .route("/:agreement_id/:idx/reject",  post(reject_milestone))
}

async fn add_milestone() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "error": "not implemented" })))
}

async fn list_milestones(Path(agreement_id): Path<String>) -> Json<Value> {
    Json(json!({ "agreement_id": agreement_id, "milestones": [] }))
}

async fn submit_milestone(Path((agreement_id, idx)): Path<(String, u32)>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "agreement_id": agreement_id, "index": idx })))
}

async fn approve_milestone(Path((agreement_id, idx)): Path<(String, u32)>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "agreement_id": agreement_id, "index": idx })))
}

async fn reject_milestone(Path((agreement_id, idx)): Path<(String, u32)>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "agreement_id": agreement_id, "index": idx })))
}
