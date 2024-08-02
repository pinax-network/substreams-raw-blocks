use common::{
    blocks::insert_timestamp,
    keys::traces_keys,
    utils::{bytes_to_hex, optional_bigint_to_string},
};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::block_view::CallView;

use crate::transactions::{is_transaction_success, transaction_status_to_string};

pub fn call_types_to_string(call_type: i32) -> String {
    match call_type {
        0 => "Unspecified".to_string(),
        1 => "Call".to_string(),
        2 => "Callcode".to_string(),
        3 => "Delegate".to_string(),
        4 => "Static".to_string(),
        5 => "Create".to_string(),
        _ => "Unknown".to_string(),
    }
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L546
pub fn insert_trace(tables: &mut DatabaseChanges, clock: &Clock, call: &CallView) {
    let transaction = call.transaction;
    let tx_index = transaction.index;
    let tx_hash = bytes_to_hex(transaction.hash.clone());
    let from = bytes_to_hex(transaction.from.clone()); // does trace contain `from`?
    let to = bytes_to_hex(transaction.to.clone()); // does trace contain `to`?
    let tx_status = transaction_status_to_string(transaction.status);
    let tx_status_code = transaction.status;
    let tx_success = is_transaction_success(transaction.status);

    // traces
    let trace = call.call;
    let address = bytes_to_hex(trace.address.clone()); // additional `trace_address`?
    let begin_ordinal = trace.begin_ordinal;
    let call_type = call_types_to_string(trace.call_type);
    let call_type_code = trace.call_type;
    let caller = bytes_to_hex(trace.caller.clone());
    let depth = trace.depth;
    let end_ordinal = trace.end_ordinal;
    let executed_code = trace.executed_code;
    let failure_reason = &trace.failure_reason;
    let gas_consumed = trace.gas_consumed;
    let gas_limit = trace.gas_limit;
    let index = trace.index; // or `subtraces`?
    let input = bytes_to_hex(trace.input.clone());
    let parent_index = trace.parent_index;
    let return_data = bytes_to_hex(trace.return_data.clone());
    let state_reverted = trace.state_reverted;
    let status_failed = trace.status_failed;
    let status_reverted = trace.status_reverted;
    let suicide = trace.suicide; // or `selfdestruct`?
    let value = optional_bigint_to_string(trace.value.clone()); // UInt256

    // TODO: trace.code_changes
    // TODO: trace.storage_changes

    let keys = traces_keys(&clock, &tx_hash, &tx_index, &index);
    let row = tables
        .push_change_composite("traces", keys, 0, table_change::Operation::Create)
        // transaction
        .change("tx_index", ("", tx_index.to_string().as_str()))
        .change("tx_hash", ("", tx_hash.as_str()))
        .change("from", ("", from.as_str()))
        .change("to", ("", to.as_str()))
        .change("tx_status", ("", tx_status.as_str()))
        .change("tx_status_code", ("", tx_status_code.to_string().as_str()))
        .change("tx_success", ("", tx_success.to_string().as_str()))
        // trace
        .change("address", ("", address.as_str()))
        .change("begin_ordinal", ("", begin_ordinal.to_string().as_str()))
        .change("call_type", ("", call_type.as_str()))
        .change("call_type_code", ("", call_type_code.to_string().as_str()))
        .change("caller", ("", caller.as_str()))
        .change("depth", ("", depth.to_string().as_str()))
        .change("end_ordinal", ("", end_ordinal.to_string().as_str()))
        .change("executed_code", ("", executed_code.to_string().as_str()))
        .change("failure_reason", ("", failure_reason.as_str()))
        .change("gas_consumed", ("", gas_consumed.to_string().as_str()))
        .change("gas_limit", ("", gas_limit.to_string().as_str()))
        .change("index", ("", index.to_string().as_str()))
        .change("input", ("", input.as_str()))
        .change("parent_index", ("", parent_index.to_string().as_str()))
        .change("return_data", ("", return_data.as_str()))
        .change("state_reverted", ("", state_reverted.to_string().as_str()))
        .change("status_failed", ("", status_failed.to_string().as_str()))
        .change("status_reverted", ("", status_reverted.to_string().as_str()))
        .change("suicide", ("", suicide.to_string().as_str()))
        .change("value", ("", value.as_str()));

    insert_timestamp(row, clock, false);
}
