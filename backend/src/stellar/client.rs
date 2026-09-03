//! Thin wrapper around the Stellar RPC / Horizon HTTP APIs.
//! Services call this adapter; they never touch reqwest or XDR directly.

use crate::errors::{AppError, Result};
use tracing::debug;

pub struct StellarClient {
    pub rpc_url: String,
    http: reqwest::Client,
}

impl StellarClient {
    pub fn new(rpc_url: String) -> Self {
        Self {
            rpc_url,
            http: reqwest::Client::new(),
        }
    }

    /// Submit a signed XDR transaction envelope and return the tx hash.
    pub async fn submit_transaction(&self, tx_xdr: &str) -> Result<String> {
        debug!("submitting transaction");
        // TODO: POST to RPC sendTransaction endpoint
        // Return tx_hash on success
        todo!("transaction submission not implemented")
    }

    /// Simulate a transaction and return the footprint + auth entries.
    pub async fn simulate_transaction(&self, tx_xdr: &str) -> Result<serde_json::Value> {
        debug!("simulating transaction");
        // TODO: POST to RPC simulateTransaction endpoint
        todo!("transaction simulation not implemented")
    }

    /// Poll for events from a given ledger cursor across the specified contract IDs.
    pub async fn get_events(
        &self,
        start_ledger: u32,
        contract_ids: &[&str],
        limit: u32,
    ) -> Result<Vec<serde_json::Value>> {
        debug!(start_ledger, "polling for contract events");
        // TODO: POST to RPC getEvents endpoint
        Ok(vec![])
    }

    /// Fetch the current ledger sequence number.
    pub async fn get_latest_ledger(&self) -> Result<u32> {
        // TODO: POST to RPC getLatestLedger endpoint
        Ok(0)
    }
}
