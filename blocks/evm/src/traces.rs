use common::{
    blocks::insert_timestamp,
    keys::traces_keys,
    utils::{bytes_to_hex, extract_method_id, optional_bigint_to_string},
};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges, TableChange};
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::{
    account_creations::insert_account_creation, balance_changes::insert_trace_balance_change, code_changes::insert_trace_code_change, gas_changes::insert_gas_change, logs::insert_log,
    nonce_changes::insert_nonce_change, storage_changes::insert_storage_change, transactions::insert_transaction_metadata,
};

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
// DetailLevel: EXTENDED
pub fn insert_trace(tables: &mut DatabaseChanges, clock: &Clock, call: &Call, transaction: &TransactionTrace) {
    // transaction
    let tx_index = transaction.index;
    let tx_hash = bytes_to_hex(&transaction.hash);
    let keys = traces_keys(&clock, &tx_hash, &tx_index, &call.index);
    let row = tables.push_change_composite("traces", keys, 0, table_change::Operation::Create);
    insert_trace_values(row, call);
    insert_timestamp(row, clock, false);
    insert_transaction_metadata(row, transaction, true);

    // TABLE::logs
    for log in call.logs.iter() {
        insert_log(tables, clock, log, transaction);
    }

    // TABLE::balance_changes
    for balance_change in call.balance_changes.iter() {
        insert_trace_balance_change(tables, clock, balance_change, transaction, call);
    }
    // TABLE::storage_changes
    for storage_change in call.storage_changes.iter() {
        insert_storage_change(tables, clock, &storage_change, transaction, call);
    }
    // TABLE::code_changes
    for code_change in call.code_changes.iter() {
        insert_trace_code_change(tables, clock, &code_change, transaction, call);
    }
    // TABLE::account_creations
    for account_creation in call.account_creations.iter() {
        insert_account_creation(tables, clock, &account_creation, transaction, call);
    }
    // TABLE::nonce_changes
    for nonce_change in call.nonce_changes.iter() {
        insert_nonce_change(tables, clock, &nonce_change, transaction, call);
    }
    // TABLE::gas_changes
    for gas_change in call.gas_changes.iter() {
        insert_gas_change(tables, clock, &gas_change, transaction, call);
    }
}

pub fn insert_trace_values(row: &mut TableChange, call: &Call) {
    // trace
    let address = bytes_to_hex(&call.address); // additional `trace_address`?
    let begin_ordinal = call.begin_ordinal;
    let call_type = call_types_to_string(call.call_type);
    let call_type_code = call.call_type;
    let caller = bytes_to_hex(&call.caller);
    let depth = call.depth;
    let end_ordinal = call.end_ordinal;
    let executed_code = call.executed_code;
    let gas_consumed = call.gas_consumed;
    let gas_limit = call.gas_limit;
    let index = call.index; // or `subtraces`?
    let input = bytes_to_hex(&call.input);
    let parent_index = call.parent_index;
    let state_reverted = call.state_reverted;
    let status_failed = call.status_failed;
    let status_reverted = call.status_reverted;
    let suicide = call.suicide; // or `selfdestruct`?
    let value = optional_bigint_to_string(call.value.clone(), "0"); // UInt256
    let method_id = extract_method_id(&call.input);

    // not available in system traces
    let failure_reason = call.failure_reason.as_str();
    let return_data = bytes_to_hex(&call.return_data);

    row.change("address", ("", address.as_str()))
        .change("begin_ordinal", ("", begin_ordinal.to_string().as_str()))
        .change("call_type", ("", call_type.as_str()))
        .change("call_type_code", ("", call_type_code.to_string().as_str()))
        .change("caller", ("", caller.as_str()))
        .change("depth", ("", depth.to_string().as_str()))
        .change("end_ordinal", ("", end_ordinal.to_string().as_str()))
        .change("executed_code", ("", executed_code.to_string().as_str()))
        .change("gas_consumed", ("", gas_consumed.to_string().as_str()))
        .change("gas_limit", ("", gas_limit.to_string().as_str()))
        .change("index", ("", index.to_string().as_str()))
        .change("input", ("", input.as_str()))
        .change("method_id", ("", method_id.as_str()))
        .change("parent_index", ("", parent_index.to_string().as_str()))
        .change("state_reverted", ("", state_reverted.to_string().as_str()))
        .change("status_failed", ("", status_failed.to_string().as_str()))
        .change("status_reverted", ("", status_reverted.to_string().as_str()))
        .change("suicide", ("", suicide.to_string().as_str()))
        .change("value", ("", value.as_str()))
        .change("failure_reason", ("", failure_reason))
        .change("return_data", ("", return_data.as_str()));
}

pub fn insert_trace_metadata(row: &mut TableChange, trace: &Call) {
    let trace_index = trace.index;
    let trace_parent_index = trace.parent_index;
    let trace_depth = trace.depth;
    let trace_caller = bytes_to_hex(&trace.caller);

    // TODO: could add additional trace metadata here
    row.change("trace_index", ("", trace_index.to_string().as_str()))
        .change("trace_parent_index", ("", trace_parent_index.to_string().as_str()))
        .change("trace_depth", ("", trace_depth.to_string().as_str()))
        .change("trace_caller", ("", trace_caller.to_string().as_str()));
}
