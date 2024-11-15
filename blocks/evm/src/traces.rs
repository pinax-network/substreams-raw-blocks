use common::{
    blocks::insert_timestamp,
    structs::BlockTimestamp,
    utils::{bytes_to_hex, optional_bigint_to_string},
};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges, TableChange};
use substreams_ethereum::pb::eth::v2::{Block, Call, TransactionTrace};

use crate::{
    account_creations::insert_account_creation,
    balance_changes::insert_balance_change,
    code_changes::insert_code_change,
    gas_changes::insert_gas_change,
    keys::traces_keys,
    logs::insert_log,
    nonce_changes::insert_nonce_change,
    pb::evm::Trace as RawTrace,
    storage_changes::insert_storage_change,
    transactions::{insert_empty_transaction_metadata, insert_transaction_metadata, is_transaction_success, transaction_status_to_string},
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
    let keys = traces_keys(&clock, &tx_hash, &tx_index.into(), &call.index);
    let row = tables.push_change_composite("traces", keys, 0, table_change::Operation::Create);
    insert_trace_row(row, call);
    insert_timestamp(row, clock, false, true);
    insert_transaction_metadata(row, transaction, true);

    // TABLE::logs
    for log in call.logs.iter() {
        insert_log(tables, clock, log, transaction);
    }
    // TABLE::balance_changes
    for balance_change in call.balance_changes.iter() {
        insert_balance_change(tables, clock, balance_change);
    }
    // TABLE::storage_changes
    for storage_change in call.storage_changes.iter() {
        insert_storage_change(tables, clock, &storage_change);
    }
    // TABLE::code_changes
    for code_change in call.code_changes.iter() {
        insert_code_change(tables, clock, &code_change);
    }
    // TABLE::account_creations
    for account_creation in call.account_creations.iter() {
        insert_account_creation(tables, clock, &account_creation);
    }
    // TABLE::nonce_changes
    for nonce_change in call.nonce_changes.iter() {
        insert_nonce_change(tables, clock, &nonce_change);
    }
    // TABLE::gas_changes
    for gas_change in call.gas_changes.iter() {
        insert_gas_change(tables, clock, &gas_change);
    }
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L121-L124
// DetailLevel: EXTENDED
// System calls are introduced in Cancun, along with blobs. They are executed outside of transactions but affect the state.
pub fn insert_system_trace(tables: &mut DatabaseChanges, clock: &Clock, call: &Call) {
    // system does not create any transaction, key is empty and only uses call index
    let keys = traces_keys(&clock, &"".to_string(), &0, &call.index);
    let row = tables.push_change_composite("traces", keys, 0, table_change::Operation::Create);
    insert_trace_row(row, call);
    insert_timestamp(row, clock, false, true);
    insert_empty_transaction_metadata(row, true);
}

pub fn insert_trace_row(row: &mut TableChange, call: &Call) {
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
    let index = call.index; // or `sub_traces`?
    let input = bytes_to_hex(&call.input); // TO-DO: fallback to 0x?
    let parent_index = call.parent_index;
    let state_reverted = call.state_reverted;
    let status_failed = call.status_failed;
    let status_reverted = call.status_reverted;
    let suicide = call.suicide; // or `selfdestruct`?
    let value = optional_bigint_to_string(&call.value, "0"); // UInt256

    // not available in system traces
    let failure_reason = &call.failure_reason;
    let return_data = bytes_to_hex(&call.return_data);

    // missing?
    // - output
    // - refund_address

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
        .change("parent_index", ("", parent_index.to_string().as_str()))
        .change("state_reverted", ("", state_reverted.to_string().as_str()))
        .change("status_failed", ("", status_failed.to_string().as_str()))
        .change("status_reverted", ("", status_reverted.to_string().as_str()))
        .change("suicide", ("", suicide.to_string().as_str()))
        .change("value", ("", value.as_str()))
        .change("failure_reason", ("", failure_reason.as_str()))
        .change("return_data", ("", return_data.as_str()));
}

pub fn collect_traces(block: &Block, timestamp: &BlockTimestamp, detail_level: &str) -> Vec<RawTrace> {
    // Only required DetailLevel=EXTENDED
    if detail_level != "Extended" {
        return vec![];
    }

    let mut traces: Vec<RawTrace> = vec![];

    // Collect system traces
    for call in &block.system_calls {
        traces.push(RawTrace {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),
            // As this is a system call, tx_hash is empty
            // tx_index, tx_status, tx_status_code, tx_success are irrelevant as well
            tx_hash: String::new(),
            tx_index: 0,
            tx_status: transaction_status_to_string(1),
            tx_status_code: 1,
            tx_success: true,
            from: bytes_to_hex(&call.caller),
            to: bytes_to_hex(&call.address),
            index: call.index,
            parent_index: call.parent_index,
            depth: call.depth,
            caller: bytes_to_hex(&call.caller),
            call_type: call_types_to_string(call.call_type),
            call_type_code: call.call_type as u32,
            address: bytes_to_hex(&call.address),
            value: optional_bigint_to_string(&call.value, "0"),
            gas_limit: call.gas_limit,
            gas_consumed: call.gas_consumed,
            input: bytes_to_hex(&call.input),
            return_data: bytes_to_hex(&call.return_data),
            failure_reason: call.failure_reason.clone(),
            begin_ordinal: call.begin_ordinal,
            end_ordinal: call.end_ordinal,
            executed_code: call.executed_code,
            state_reverted: call.state_reverted,
            status_failed: call.status_failed,
            status_reverted: call.status_reverted,
            suicide: call.suicide,
        });
    }

    // Collect transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            traces.push(RawTrace {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: bytes_to_hex(&transaction.hash),
                tx_index: transaction.index,
                tx_status: transaction_status_to_string(transaction.status),
                tx_status_code: transaction.status as u32,
                tx_success: is_transaction_success(transaction.status),
                from: bytes_to_hex(&transaction.from),
                to: bytes_to_hex(&transaction.to),
                index: call.index,
                parent_index: call.parent_index,
                depth: call.depth,
                caller: bytes_to_hex(&call.caller),
                call_type: call_types_to_string(call.call_type),
                call_type_code: call.call_type as u32,
                address: bytes_to_hex(&call.address),
                value: optional_bigint_to_string(&call.value, "0"),
                gas_limit: call.gas_limit,
                gas_consumed: call.gas_consumed,
                input: bytes_to_hex(&call.input),
                return_data: bytes_to_hex(&call.return_data),
                failure_reason: call.failure_reason.clone(),
                begin_ordinal: call.begin_ordinal,
                end_ordinal: call.end_ordinal,
                executed_code: call.executed_code,
                state_reverted: call.state_reverted,
                status_failed: call.status_failed,
                status_reverted: call.status_reverted,
                suicide: call.suicide,
            });
        }
    }

    traces
}
