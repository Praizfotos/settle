//! Settle Backend — entry point.
//!
//! Boots the Axum HTTP server, the Stellar event indexer, and the
//! background reconciliation loop. All subsystems share a single
//! PostgreSQL connection pool.

use anyhow::Result;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tower_http::cors::CorsLayer;
use axum::Router;

mod config;
mod api;
mod domain;
mod errors;
mod database;
mod indexer;
mod services;
mod stellar;

use config::Config;
use database::DatabaseManager;
use stellar::client::StellarClient;
use indexer::listener::{IndexerListener, ListenerConfig};
use services::*;

/// Application state shared across all handlers and services
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub stellar_client: Arc<StellarClient>,
    pub agreement_service: Arc<AgreementService>,
    pub settlement_service: Arc<SettlementService>,
    pub dispute_service: Arc<DisputeService>,
    pub reputation_service: Arc<ReputationService>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize structured logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Settle backend starting");

    // Load configuration from environment
    let config = Config::from_env()
        .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;
    
    info!("Configuration loaded");

    // Initialize database connection and run migrations
    let db_manager = DatabaseManager::new(&config.database).await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;
    
    db_manager.migrate().await
        .map_err(|e| anyhow::anyhow!("Failed to run database migrations: {}", e))?;

    let db_pool = db_manager.pool().clone();

    // Initialize Stellar client
    let stellar_client = Arc::new(StellarClient::new(
        config.stellar.horizon_url.clone(),
        config.stellar.network_passphrase.clone(),
    ));

    // Initialize services
    let agreement_service = Arc::new(AgreementService::new(db_pool.clone()));
    let settlement_service = Arc::new(SettlementService::new(db_pool.clone()));
    let dispute_service = Arc::new(DisputeService::new(db_pool.clone()));
    let reputation_service = Arc::new(ReputationService::new(db_pool.clone()));

    // Create application state
    let app_state = AppState {
        db_pool: db_pool.clone(),
        stellar_client: stellar_client.clone(),
        agreement_service: agreement_service.clone(),
        settlement_service: settlement_service.clone(),
        dispute_service: dispute_service.clone(),
        reputation_service: reputation_service.clone(),
    };

    // Start indexer task
    let indexer_config = ListenerConfig {
        stellar_client: stellar_client.clone(),
        contract_address: config.stellar.contract_address.clone(),
        poll_interval_ms: config.indexer.poll_interval_ms,
        batch_size: config.indexer.batch_size,
        max_retries: config.indexer.max_retries,
        retry_delay_ms: config.indexer.retry_delay_ms,
    };

    let indexer_pool = db_pool.clone();
    let indexer_handle = tokio::spawn(async move {
        let mut listener = IndexerListener::new(indexer_config);
        if let Err(e) = listener.run(indexer_pool).await {
            error!("Indexer error: {}", e);
        }
    });

    // Build API router
    let cors = CorsLayer::new()
        .allow_origin(config.server.cors_origins.iter()
            .map(|origin| origin.parse().unwrap())
            .collect::<Vec<_>>())
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers(vec![
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    let app = Router::new()
        .nest("/api/v1", api::router())
        .layer(cors)
        .with_state(app_state);

    // Start API server
    let listen_addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting API server on {}", listen_addr);

    let listener = tokio::net::TcpListener::bind(&listen_addr).await
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", listen_addr, e))?;

    let server_handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            error!("Server error: {}", e);
        }
    });

    info!("Settle backend ready");

    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
        }
        _ = indexer_handle => {
            warn!("Indexer task finished unexpectedly");
        }
        _ = server_handle => {
            warn!("Server task finished unexpectedly");
        }
    }

    info!("Settle backend shutdown complete");
    Ok(())
}
