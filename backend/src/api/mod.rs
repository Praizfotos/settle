//! HTTP API layer — thin Axum routers that delegate to services.

use axum::{extract::State, routing::get, Router, Json};
use serde_json::{json, Value};

pub mod agreements;
pub mod disputes;
pub mod milestones;
pub mod reputation;
pub mod settlements;

use crate::AppState;

/// Build the root API router with all sub-routes mounted.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/agreements",  agreements::router())
        .nest("/milestones",  milestones::router())
        .nest("/disputes",    disputes::router())
        .nest("/settlements", settlements::router())
        .nest("/reputation",  reputation::router())
}

async fn health(State(state): State<AppState>) -> Json<Value> {
    let db_healthy = state.db_pool.is_closed() == false;
    
    Json(json!({
        "status": if db_healthy { "healthy" } else { "unhealthy" },
        "database": db_healthy,
        "service": "settle-backend",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
