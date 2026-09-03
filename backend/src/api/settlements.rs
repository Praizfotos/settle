use axum::{extract::Path, response::Json, routing::get, Router};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/",    get(list_settlements))
        .route("/:id", get(get_settlement))
}

async fn list_settlements() -> Json<Value> {
    Json(json!({ "settlements": [] }))
}

async fn get_settlement(Path(id): Path<String>) -> Json<Value> {
    Json(json!({ "id": id }))
}
