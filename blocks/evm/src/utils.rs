use substreams::scalar::BigInt as ScalarBigInt;
use substreams_ethereum::pb::eth::v2::{balance_change, block, gas_change, transaction_trace, BigInt, CallType, TransactionTraceStatus};

// Firehose `BigInt` is an unsigned big-endian byte string. Rendered as a decimal
// string so the SQL sink can store it as NUMERIC(78,0) (`uint256` annotation).
pub fn bigint_to_string(value: &Option<BigInt>) -> String {
    match value {
        Some(b) if !b.bytes.is_empty() => ScalarBigInt::from_unsigned_bytes_be(&b.bytes).to_string(),
        _ => "0".to_string(),
    }
}

// Same as `bigint_to_string` but keeps `None` for absent values (nullable columns).
pub fn optional_bigint_to_string(value: &Option<BigInt>) -> Option<String> {
    value.as_ref().map(|_| bigint_to_string(value))
}

pub fn bytes_to_uint256_string(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "0".to_string();
    }
    ScalarBigInt::from_unsigned_bytes_be(bytes).to_string()
}

// Enum names follow firehose-parquet: the Firehose enum name without its prefix.
fn strip_prefix(name: &str, prefix: &str) -> String {
    name.strip_prefix(prefix).unwrap_or(name).to_string()
}

pub fn detail_level_text(value: i32) -> String {
    block::DetailLevel::try_from(value)
        .map(|v| strip_prefix(v.as_str_name(), "DETAILLEVEL_"))
        .unwrap_or_else(|_| "UNKNOWN".to_string())
}

pub fn transaction_type_text(value: i32) -> String {
    transaction_trace::Type::try_from(value)
        .map(|v| strip_prefix(v.as_str_name(), "TRX_TYPE_"))
        .unwrap_or_else(|_| "UNKNOWN".to_string())
}

pub fn transaction_status_text(value: i32) -> String {
    TransactionTraceStatus::try_from(value).map(|v| v.as_str_name().to_string()).unwrap_or_else(|_| "UNKNOWN".to_string())
}

pub fn call_type_text(value: i32) -> String {
    CallType::try_from(value).map(|v| v.as_str_name().to_string()).unwrap_or_else(|_| "UNKNOWN".to_string())
}

pub fn balance_change_reason_text(value: i32) -> String {
    balance_change::Reason::try_from(value)
        .map(|v| strip_prefix(v.as_str_name(), "REASON_"))
        .unwrap_or_else(|_| "UNKNOWN".to_string())
}

pub fn gas_change_reason_text(value: i32) -> String {
    gas_change::Reason::try_from(value)
        .map(|v| strip_prefix(v.as_str_name(), "REASON_"))
        .unwrap_or_else(|_| "UNKNOWN".to_string())
}

// Keccak preimages are shipped by Firehose as hex strings; store them as bytes.
pub fn hex_to_bytes(value: &str) -> Vec<u8> {
    let stripped = value.strip_prefix("0x").unwrap_or(value);
    hex::decode(stripped).unwrap_or_else(|_| value.as_bytes().to_vec())
}

pub fn empty_to_none(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.is_empty() {
        None
    } else {
        Some(bytes.to_vec())
    }
}
