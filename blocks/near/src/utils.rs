use serde_json::json;
use substreams::scalar::BigInt as ScalarBigInt;
use substreams_near::pb::sf::near::r#type::v1::{
    access_key_permission, action, execution_outcome, failure_execution_status, state_change_cause, state_change_value, AccessKey, Action, BigInt, CryptoHash, ExecutionOutcome, PublicKey,
    StateChangeCause, StateChangeValue,
};

pub fn hash_bytes(hash: &Option<CryptoHash>) -> Vec<u8> {
    hash.as_ref().map(|h| h.bytes.clone()).unwrap_or_default()
}

// NEAR BigInts are big-endian unsigned (yoctoNEAR). Decimal string -> NUMERIC(78,0).
pub fn bigint_to_string(value: &Option<BigInt>) -> String {
    match value {
        Some(b) if !b.bytes.is_empty() => ScalarBigInt::from_unsigned_bytes_be(&b.bytes).to_string(),
        _ => "0".to_string(),
    }
}

pub fn optional_bigint_to_string(value: &Option<BigInt>) -> Option<String> {
    value.as_ref().map(|_| bigint_to_string(value))
}

pub fn public_key_bytes(key: &Option<PublicKey>) -> Vec<u8> {
    key.as_ref().map(|k| k.bytes.clone()).unwrap_or_default()
}

pub fn public_key_type(key: &Option<PublicKey>) -> String {
    match key.as_ref().map(|k| k.r#type) {
        Some(0) => "ed25519".to_string(),
        Some(1) => "secp256k1".to_string(),
        Some(other) => other.to_string(),
        None => String::new(),
    }
}

// Names follow firehose-parquet.
pub fn action_type_name(a: &Action) -> &'static str {
    match &a.action {
        Some(action::Action::CreateAccount(_)) => "CreateAccount",
        Some(action::Action::DeployContract(_)) => "DeployContract",
        Some(action::Action::FunctionCall(_)) => "FunctionCall",
        Some(action::Action::Transfer(_)) => "Transfer",
        Some(action::Action::Stake(_)) => "Stake",
        Some(action::Action::AddKey(_)) => "AddKey",
        Some(action::Action::DeleteKey(_)) => "DeleteKey",
        Some(action::Action::DeleteAccount(_)) => "DeleteAccount",
        Some(action::Action::Delegate(_)) => "Delegate",
        None => "Unknown",
    }
}

pub fn execution_status(outcome: Option<&ExecutionOutcome>) -> &'static str {
    match outcome.and_then(|o| o.status.as_ref()) {
        Some(execution_outcome::Status::SuccessValue(_)) => "SuccessValue",
        Some(execution_outcome::Status::SuccessReceiptId(_)) => "SuccessReceiptId",
        Some(execution_outcome::Status::Failure(_)) => "Failure",
        _ => "Unknown",
    }
}

// Debug rendering of the failure variant, only for failed outcomes.
pub fn execution_failure(outcome: Option<&ExecutionOutcome>) -> Option<String> {
    match outcome.and_then(|o| o.status.as_ref()) {
        Some(execution_outcome::Status::Failure(f)) => Some(match &f.failure {
            Some(failure_execution_status::Failure::ActionError(e)) => format!("{:?}", e),
            Some(failure_execution_status::Failure::InvalidTxError(e)) => format!("InvalidTxError({})", e),
            None => "Failure".to_string(),
        }),
        _ => None,
    }
}

pub fn state_change_type(value: &StateChangeValue) -> &'static str {
    match &value.value {
        Some(state_change_value::Value::AccountUpdate(_)) => "AccountUpdate",
        Some(state_change_value::Value::AccountDeletion(_)) => "AccountDeletion",
        Some(state_change_value::Value::AccessKeyUpdate(_)) => "AccessKeyUpdate",
        Some(state_change_value::Value::AccessKeyDeletion(_)) => "AccessKeyDeletion",
        Some(state_change_value::Value::DataUpdate(_)) => "DataUpdate",
        Some(state_change_value::Value::DataDeletion(_)) => "DataDeletion",
        Some(state_change_value::Value::ContractCodeUpdate(_)) => "ContractCodeUpdate",
        Some(state_change_value::Value::ContractDeletion(_)) => "ContractCodeDeletion",
        None => "Unknown",
    }
}

pub fn state_change_cause(cause: &StateChangeCause) -> &'static str {
    match &cause.cause {
        Some(state_change_cause::Cause::NotWritableToDisk(_)) => "NotWritableToDisk",
        Some(state_change_cause::Cause::InitialState(_)) => "InitialState",
        Some(state_change_cause::Cause::TransactionProcessing(_)) => "TransactionProcessing",
        Some(state_change_cause::Cause::ActionReceiptProcessingStarted(_)) => "ActionReceiptProcessingStarted",
        Some(state_change_cause::Cause::ActionReceiptGasReward(_)) => "ActionReceiptGasReward",
        Some(state_change_cause::Cause::ReceiptProcessing(_)) => "ReceiptProcessing",
        Some(state_change_cause::Cause::PostponedReceipt(_)) => "PostponedReceipt",
        Some(state_change_cause::Cause::UpdatedDelayedReceipts(_)) => "UpdatedDelayedReceipts",
        Some(state_change_cause::Cause::ValidatorAccountsUpdate(_)) => "ValidatorAccountsUpdate",
        Some(state_change_cause::Cause::Migration(_)) => "Migration",
        None => "Unknown",
    }
}

// (tx hash, receipt hash) referenced by the cause, when any.
pub fn state_change_cause_hashes(cause: &StateChangeCause) -> (Vec<u8>, Vec<u8>) {
    match &cause.cause {
        Some(state_change_cause::Cause::TransactionProcessing(c)) => (hash_bytes(&c.tx_hash), vec![]),
        Some(state_change_cause::Cause::ActionReceiptProcessingStarted(c)) => (vec![], hash_bytes(&c.receipt_hash)),
        Some(state_change_cause::Cause::ActionReceiptGasReward(c)) => (vec![], hash_bytes(&c.tx_hash)),
        Some(state_change_cause::Cause::ReceiptProcessing(c)) => (vec![], hash_bytes(&c.tx_hash)),
        Some(state_change_cause::Cause::PostponedReceipt(c)) => (vec![], hash_bytes(&c.tx_hash)),
        _ => (vec![], vec![]),
    }
}

pub fn access_key_json(key: &Option<AccessKey>) -> Option<String> {
    key.as_ref().map(|k| {
        match k.permission.as_ref().and_then(|p| p.permission.as_ref()) {
            Some(access_key_permission::Permission::FunctionCall(f)) => json!({
                "type": "FunctionCall",
                "receiver_id": f.receiver_id,
                "method_names": f.method_names,
                "allowance": optional_bigint_to_string(&f.allowance),
            }),
            Some(access_key_permission::Permission::FullAccess(_)) => json!({"type": "FullAccess"}),
            None => json!({"type": "Unknown"}),
        }
        .to_string()
    })
}

pub fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let triple = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
        out.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        out.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        out.push(if chunk.len() > 1 { CHARS[((triple >> 6) & 0x3F) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { CHARS[(triple & 0x3F) as usize] as char } else { '=' });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_standard_alphabet() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(&[0xff, 0xee]), "/+4=");
    }
}
