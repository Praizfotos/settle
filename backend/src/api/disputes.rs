use axum::{extract::Path, http::StatusCode, response::Json, routing::{get, post}, Router};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/",    post(open_dispute))
        .route("/:id", get(get_dispute))
        .route("/:id/review",  post(begin_review))
        .route("/:id/resolve", post(resolve_dispute))
}

async fn open_dispute() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "error": "not implemented" })))
}

async fn get_dispute(Path(id): Path<String>) -> Json<Value> {
    Json(json!({ "id": id }))
}

async fn begin_review(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "id": id })))
}

async fn resolve_dispute(Path(id): Path<String>) -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "id": id })))
}
