use sqlx::{PgPool, Pool, Postgres, migrate::MigrateDatabase};
use std::time::Duration;
use tracing::{info, error};

use crate::config::DatabaseConfig;
use crate::errors::AppError;

/// Database connection manager
pub struct DatabaseManager {
    pool: PgPool,
}

impl DatabaseManager {
    /// Create new database manager with connection pool
    pub async fn new(config: &DatabaseConfig) -> Result<Self, AppError> {
        info!("Connecting to database: {}", mask_password(&config.url));
        
        // Ensure database exists
        if !Postgres::database_exists(&config.url).await.unwrap_or(false) {
            info!("Creating database...");
            Postgres::create_database(&config.url)
                .await
                .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;
        }

        // Create connection pool
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .max_lifetime(Some(Duration::from_secs(config.max_lifetime)))
            .connect(&config.url)
            .await
            .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

        info!("Database connection pool created with {} max connections", config.max_connections);

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<(), AppError> {
        info!("Running database migrations...");
        
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| AppError::Migration(e.to_string()))?;
        
        info!("Database migrations completed");
        Ok(())
    }

    /// Get database connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Health check for database connection
    pub async fn health_check(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }

    /// Close all database connections
    pub async fn close(self) {
        info!("Closing database connections...");
        self.pool.close().await;
    }
}

/// Mask password in database URL for logging
fn mask_password(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let mut masked = url.to_string();
            masked.replace_range(colon_pos + 1..at_pos, "***");
            return masked;
        }
    }
    url.to_string()
}