use axum::{extract::Path, response::Json, routing::get, Router};
use serde_json::{json, Value};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:address", get(get_reputation))
}

async fn get_reputation(Path(address): Path<String>) -> Json<Value> {
    // TODO: call ReputationService::compute_for_address
    Json(json!({
        "address": address,
        "score": null,
        "label": "unrated",
        "message": "reputation indexing not yet active"
    }))
}
