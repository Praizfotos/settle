//! Helpers for building Soroban contract invocation transactions.
//!
//! Each function returns a base64-encoded XDR transaction envelope
//! ready for simulation + signing + submission.

use crate::errors::Result;

/// Build a `create_agreement` contract invocation.
pub fn build_create_agreement(
    source_account: &str,
    contract_id: &str,
    client: &str,
    provider: &str,
    token: &str,
    amount: i128,
    title: &str,
) -> Result<String> {
    // TODO: use stellar_xdr to construct InvokeContractArgs
    todo!("build_create_agreement not implemented")
}

/// Build a `fund_agreement` invocation.
pub fn build_fund_agreement(
    source_account: &str,
    contract_id: &str,
    agreement_id: u64,
) -> Result<String> {
    todo!("build_fund_agreement not implemented")
}

/// Build an `approve_milestone` invocation.
pub fn build_approve_milestone(
    source_account: &str,
    contract_id: &str,
    agreement_id: u64,
    milestone_index: u32,
) -> Result<String> {
    todo!("build_approve_milestone not implemented")
}

/// Build an `open_dispute` invocation.
pub fn build_open_dispute(
    source_account: &str,
    contract_id: &str,
    agreement_id: u64,
    disputed_amount: i128,
    reason: &str,
) -> Result<String> {
    todo!("build_open_dispute not implemented")
}
