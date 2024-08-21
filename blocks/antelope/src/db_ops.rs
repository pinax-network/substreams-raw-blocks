use common::utils::add_prefix_to_hex;
use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use common::keys::logs_keys;
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
pub fn insert_db_op(tables: &mut DatabaseChanges, clock: &Clock, db_op: &DbOp, transaction: &TransactionTrace, index: u32) {
    // storage change
	let operation = operation_to_string(db_op.operation);
    let operation_code = db_op.operation;
	let action_index = db_op.action_index;
	let code = &db_op.code;
	let scope = &db_op.scope;
	let table_name = &db_op.table_name;
	let primary_key = &db_op.primary_key;
	let old_payer = &db_op.old_payer;
	let new_payer = &db_op.new_payer;
	let old_data = bytes_to_hex(&db_op.old_data.to_vec());
	let new_data = bytes_to_hex(&db_op.new_data.to_vec());
	let old_data_json = &db_op.old_data_json;
	let new_data_json = &db_op.new_data_json;

    // transaction
    let tx_hash = add_prefix_to_hex(&transaction.id);

    let keys = logs_keys(&clock, &tx_hash, &index);
    let row = tables
        .push_change_composite("db_ops", keys, 0, table_change::Operation::Create)
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