//! Helpers for building Soroban contract invocation transactions.
//!
//! NOTE: These are primarily used for backend-initiated operations.
//! Most user-authorized writes go through the frontend SDK + wallet.

use crate::errors::{AppError, Result};
use stellar_xdr::curr::{
    HostFunction, InvokeContractArgs, InvokeHostFunctionOp, Operation, OperationBody,
    ScAddress, ScString, ScSymbol, ScVal, ScVec, VecM,
};

/// Helper to create ScString from a string (for Soroban String type).
fn sc_string(data: &str) -> Result<ScVal> {
    let s: ScString = data.as_bytes().to_vec().try_into()
        .map_err(|_| AppError::InvalidInput(format!("String too long: {}", data.len())))?;
    Ok(ScVal::String(s))
}

/// Build a Soroban `invoke` operation for a contract method.
fn build_invoke_op(
    contract_id: &str,
    method: &str,
    args: Vec<ScVal>,
) -> Result<Operation> {
    let contract_address = decode_contract_address(contract_id)?;

    let function_name = ScSymbol::try_from(method)
        .map_err(|_| AppError::InvalidInput(format!("Method name too long: {method}")))?;

    let args_vecm: VecM<ScVal> = args.try_into()
        .map_err(|_| AppError::InvalidInput("Too many arguments".to_string()))?;

    let invoke_args = InvokeContractArgs {
        contract_address,
        function_name,
        args: args_vecm,
    };

    let host_fn = HostFunction::InvokeContract(invoke_args);

    let auth: VecM<stellar_xdr::curr::SorobanAuthorizationEntry> = VecM::default();

    Ok(Operation {
        source_account: None,
        body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
            host_function: host_fn,
            auth,
        }),
    })
}

/// Decode a contract address (hex-encoded 32-byte hash) to ScAddress.
fn decode_contract_address(contract_id: &str) -> Result<ScAddress> {
    let bytes = hex::decode(contract_id)
        .map_err(|_| AppError::InvalidInput(format!("Invalid contract ID hex: {contract_id}")))?;

    if bytes.len() == 32 {
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&bytes);
        return Ok(ScAddress::Contract(stellar_xdr::curr::Hash(hash_bytes)));
    }

    Err(AppError::InvalidInput(format!(
        "Invalid contract ID: expected 32 bytes, got {}",
        bytes.len()
    )))
}

/// Decode an ed25519 public key strkey to raw 32 bytes.
fn decode_ed25519_public_key(strkey: &str) -> Result<[u8; 32]> {
    use stellar_strkey::ed25519::PublicKey;

    let pubkey = PublicKey::from_string(strkey)
        .map_err(|e| AppError::InvalidInput(format!("Invalid public key: {e}")))?;

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&pubkey.0);
    Ok(bytes)
}

/// Decode a strkey account address to ScAddress::Account.
fn decode_account_address(strkey: &str) -> Result<ScAddress> {
    let pubkey_bytes = decode_ed25519_public_key(strkey)?;
    Ok(ScAddress::Account(stellar_xdr::curr::AccountId(
        stellar_xdr::curr::PublicKey::PublicKeyTypeEd25519(stellar_xdr::curr::Uint256(
            pubkey_bytes,
        )),
    )))
}

/// Build an i128 ScVal from a signed 128-bit integer.
fn sc_i128(amount: i128) -> ScVal {
    ScVal::I128(stellar_xdr::curr::Int128Parts {
        hi: (amount >> 64) as i64,
        lo: amount as u64,
    })
}

/// Build a `create_agreement` contract invocation.
///
/// Contract signature:
///   create_agreement(env, id: String, creator: Address, counterparty: Address,
///                    token: Address, total_amount: i128, expires_at: u64,
///                    milestones: Vec<String>)
pub fn build_create_agreement(
    source_account: &str,
    contract_id: &str,
    agreement_id: &str,
    counterparty: &str,
    token: &str,
    amount: i128,
    expires_at: u64,
    milestone_names: &[String],
) -> Result<String> {
    // Encode milestones as Soroban Vec<String>
    let milestone_scvals: Vec<ScVal> = milestone_names
        .iter()
        .map(|name| sc_string(name))
        .collect::<Result<Vec<_>>>()?;
    let milestones_vecm: VecM<ScVal> = milestone_scvals.try_into()
        .map_err(|_| AppError::InvalidInput("Too many milestones".to_string()))?;

    let args = vec![
        sc_string(agreement_id)?,                       // String (not Bytes)
        ScVal::Address(decode_account_address(source_account)?), // Address
        ScVal::Address(decode_account_address(counterparty)?),   // Address (not Bytes)
        ScVal::Address(decode_account_address(token)?),           // Address (not Bytes)
        sc_i128(amount),                                      // i128
        ScVal::U64(expires_at),                               // u64
        ScVal::Vec(Some(ScVec(milestones_vecm))),              // Vec<String> (not Void)
    ];

    let _op = build_invoke_op(contract_id, "create_agreement", args)?;

    Ok(format!("Op built for create_agreement on contract {contract_id}"))
}

/// Build a `fund_agreement` invocation.
///
/// Contract signature:
///   fund_agreement(env, id: String, funder: Address, amount: i128)
pub fn build_fund_agreement(
    source_account: &str,
    contract_id: &str,
    agreement_id: &str,
    amount: i128,
) -> Result<String> {
    let args = vec![
        sc_string(agreement_id)?,                           // String (not Bytes)
        ScVal::Address(decode_account_address(source_account)?), // Address
        sc_i128(amount),                                    // i128
    ];

    let _op = build_invoke_op(contract_id, "fund_agreement", args)?;

    Ok(format!("Op built for fund_agreement on contract {contract_id}"))
}

/// Build an `approve_milestone` invocation.
///
/// Contract signature:
///   approve_milestone(env, id: String, approver: Address)
pub fn build_approve_milestone(
    source_account: &str,
    contract_id: &str,
    milestone_id: &str,
) -> Result<String> {
    let args = vec![
        sc_string(milestone_id)?,                           // String (not Bytes)
        ScVal::Address(decode_account_address(source_account)?), // Address
    ];

    let _op = build_invoke_op(contract_id, "approve_milestone", args)?;

    Ok(format!("Op built for approve_milestone on contract {contract_id}"))
}

/// Build an `open_dispute` invocation.
///
/// Contract signature:
///   open_dispute(env, id: String, agreement_id: String, opener: Address,
///                reason: String, initial_evidence: String)
pub fn build_open_dispute(
    source_account: &str,
    contract_id: &str,
    dispute_id: &str,
    agreement_id: &str,
    reason: &str,
    initial_evidence: &str,
) -> Result<String> {
    let args = vec![
        sc_string(dispute_id)?,                             // String (not Bytes)
        sc_string(agreement_id)?,                           // String (not Bytes)
        ScVal::Address(decode_account_address(source_account)?), // Address
        sc_string(reason)?,                                 // String
        sc_string(initial_evidence)?,                       // String (not Void)
    ];

    let _op = build_invoke_op(contract_id, "open_dispute", args)?;

    Ok(format!("Op built for open_dispute on contract {contract_id}"))
}
