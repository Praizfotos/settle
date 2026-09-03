use serde::Deserialize;
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub stellar: StellarConfig,
    pub server: ServerConfig,
    pub indexer: IndexerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub max_lifetime: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StellarConfig {
    pub network_passphrase: String,
    pub horizon_url: String,
    pub contract_address: String,
    pub secret_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndexerConfig {
    pub poll_interval_ms: u64,
    pub batch_size: u32,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Config {
    /// Load configuration from environment variables with sensible defaults
    pub fn from_env() -> Result<Self, ConfigError> {
        let database = DatabaseConfig {
            url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://settle:settle@localhost:5432/settle".to_string()),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("DATABASE_MAX_CONNECTIONS must be a number"))?,
            min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("DATABASE_MIN_CONNECTIONS must be a number"))?,
            max_lifetime: env::var("DATABASE_MAX_LIFETIME")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("DATABASE_MAX_LIFETIME must be a number"))?,
        };

        let stellar = StellarConfig {
            network_passphrase: env::var("STELLAR_NETWORK_PASSPHRASE")
                .unwrap_or_else(|_| "Test SDF Network ; September 2015".to_string()),
            horizon_url: env::var("STELLAR_HORIZON_URL")
                .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".to_string()),
            contract_address: env::var("STELLAR_CONTRACT_ADDRESS")
                .map_err(|_| ConfigError::MissingRequired("STELLAR_CONTRACT_ADDRESS"))?,
            secret_key: env::var("STELLAR_SECRET_KEY").ok(),
        };

        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("SERVER_PORT must be a number"))?,
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "*".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        };

        let indexer = IndexerConfig {
            poll_interval_ms: env::var("INDEXER_POLL_INTERVAL_MS")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("INDEXER_POLL_INTERVAL_MS must be a number"))?,
            batch_size: env::var("INDEXER_BATCH_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("INDEXER_BATCH_SIZE must be a number"))?,
            max_retries: env::var("INDEXER_MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("INDEXER_MAX_RETRIES must be a number"))?,
            retry_delay_ms: env::var("INDEXER_RETRY_DELAY_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidConfig("INDEXER_RETRY_DELAY_MS must be a number"))?,
        };

        Ok(Config {
            database,
            stellar,
            server,
            indexer,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingRequired(&'static str),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(&'static str),
}