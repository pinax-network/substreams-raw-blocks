use common::keys::transaction_keys;
use common::sinks::insert_timestamp;
use common::utils::bytes_to_hex;
use common::utils::optional_bigint_to_string;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::TransactionTrace;

use crate::logs::insert_log;
use crate::traces::insert_trace;

pub fn transaction_type_to_string(r#type: i32) -> String {
    match r#type {
        0 => "Legacy".to_string(),
        1 => "AccessList".to_string(),
        2 => "DynamicFee".to_string(),
        3 => "Blob".to_string(),
        100 => "ArbitrumDeposit".to_string(),
        101 => "ArbitrumUnsigned".to_string(),
        102 => "ArbitrumContract".to_string(),
        104 => "ArbitrumRetry".to_string(),
        105 => "ArbitrumSubmitRetryable".to_string(),
        106 => "ArbitrumInternal".to_string(),
        120 => "ArbitrumLegacy".to_string(),
        126 => "OptimismDeposit".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn transaction_status_to_string(status: i32) -> String {
    match status {
        0 => "Unknown".to_string(),
        1 => "Succeeded".to_string(),
        2 => "Failed".to_string(),
        3 => "Reverted".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn is_transaction_success(status: i32) -> bool {
    status == 1
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L658
pub fn insert_transaction(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace) {
    let index = transaction.index.to_string();
    let hash = bytes_to_hex(transaction.hash.clone());
    let from = bytes_to_hex(transaction.from.clone());
    let to = bytes_to_hex(transaction.to.clone());
    let nonce = transaction.nonce.to_string();
    let gas_price = optional_bigint_to_string(transaction.gas_price.clone()); // UInt256
    let gas_limit = transaction.gas_limit.to_string();
    let value = optional_bigint_to_string(transaction.value.clone());
    let input = bytes_to_hex(transaction.input.clone());
    let v = bytes_to_hex(transaction.v.clone());
    let r = bytes_to_hex(transaction.r.clone());
    let s = bytes_to_hex(transaction.s.clone());
    let gas_used = transaction.gas_used.to_string();
    let r#type = transaction_type_to_string(transaction.r#type);
    let type_code = transaction.r#type.to_string();
    let max_fee_per_gas = optional_bigint_to_string(transaction.max_fee_per_gas.clone());
    let max_priority_fee_per_gas = optional_bigint_to_string(transaction.max_priority_fee_per_gas.clone());
    let return_data = bytes_to_hex(transaction.return_data.clone());
    let public_key = bytes_to_hex(transaction.public_key.clone());
    let begin_ordinal = transaction.begin_ordinal.to_string();
    let end_ordinal = transaction.end_ordinal.to_string();
    let success = is_transaction_success(transaction.status).to_string();
    let status = transaction_status_to_string(transaction.status);
    let status_code = transaction.status.to_string();

    let keys = transaction_keys(&clock, &hash);
    let row = tables
        .push_change_composite("balance_changes", keys, 0, table_change::Operation::Create)
        .change("index", ("", index.as_str()))
        .change("hash", ("", hash.as_str()))
        .change("from", ("", from.as_str()))
        .change("to", ("", to.as_str()))
        .change("nonce", ("", nonce.as_str()))
        .change("gas_price", ("", gas_price.as_str()))
        .change("gas_limit", ("", gas_limit.as_str()))
        .change("value", ("", value.as_str()))
        .change("input", ("", input.as_str()))
        .change("v", ("", v.as_str()))
        .change("r", ("", r.as_str()))
        .change("s", ("", s.as_str()))
        .change("gas_used", ("", gas_used.as_str()))
        .change("r", ("", r.as_str()))
        .change("type", ("", r#type.as_str()))
        .change("type_code", ("", type_code.as_str()))
        .change("max_fee_per_gas", ("", max_fee_per_gas.as_str()))
        .change("max_priority_fee_per_gas", ("", max_priority_fee_per_gas.as_str()))
        .change("return_data", ("", return_data.as_str()))
        .change("public_key", ("", public_key.as_str()))
        .change("begin_ordinal", ("", begin_ordinal.as_str()))
        .change("end_ordinal", ("", end_ordinal.as_str()))
        .change("success", ("", success.as_str()))
        .change("status", ("", status.as_str()))
        .change("status_code", ("", status_code.as_str()));

    insert_timestamp(row, clock, false);

    // traces & logs
    for (log, call) in transaction.logs_with_calls() {
        insert_log(tables, clock, log, &transaction);
        insert_trace(tables, clock, &call);
    }
}
