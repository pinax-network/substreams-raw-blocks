use common::blocks::insert_timestamp;
use common::keys::logs_keys;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_antelope::pb::{DbOp, TransactionTrace};

use crate::transactions::insert_transaction_metadata;

pub fn operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Update".to_string(),
        3 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
pub fn insert_storage_change(tables: &mut DatabaseChanges, clock: &Clock, storage_change: &DbOp, transaction: &TransactionTrace, index: u32) {
    // storage change
	let operation = operation_to_string(storage_change.operation);
    let operation_code = storage_change.operation;
	let action_index = storage_change.action_index;
	let code = &storage_change.code;
	let scope = &storage_change.scope;
	let table_name = &storage_change.table_name;
	let primary_key = &storage_change.primary_key;
	let old_payer = &storage_change.old_payer;
	let new_payer = &storage_change.new_payer;
	let old_data = bytes_to_hex(&storage_change.old_data.to_vec());
	let new_data = bytes_to_hex(&storage_change.new_data.to_vec());
	let old_data_json = &storage_change.old_data_json;
	let new_data_json = &storage_change.new_data_json;

    let keys = logs_keys(&clock, &transaction.id, &index);
    let row = tables
        .push_change_composite("storage_changes", keys, 0, table_change::Operation::Create)
        .change("index", ("", index.to_string().as_str()))
        .change("operation", ("", operation.to_string().as_str()))
        .change("operation_code", ("", operation_code.to_string().as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("code", ("", code.to_string().as_str()))
        .change("scope", ("", scope.to_string().as_str()))
        .change("table_name", ("", table_name.to_string().as_str()))
        .change("primary_key", ("", primary_key.to_string().as_str()))
        .change("old_payer", ("", old_payer.to_string().as_str()))
        .change("new_payer", ("", new_payer.to_string().as_str()))
        .change("old_data", ("", old_data.to_string().as_str()))
        .change("new_data", ("", new_data.to_string().as_str()))
        .change("old_data_json", ("", old_data_json.to_string().as_str()))
        .change("new_data_json", ("", new_data_json.to_string().as_str()));

    insert_timestamp(row, clock, false);
    insert_transaction_metadata(row, transaction);
}
