use common::{blocks::insert_timestamp, structs::BlockTimestamp};
use substreams::{pb::substreams::Clock, Hex};
use substreams_antelope::{
    pb::{DbOp, TransactionTrace},
    Block,
};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::db_ops_keys, pb::antelope::DbOp as RawDbOp, transactions::is_transaction_success};

use super::transactions::insert_transaction_metadata;

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
    let operation = operation_to_string(db_op.operation);
    let operation_code = db_op.operation;
    let action_index = db_op.action_index;
    let code = &db_op.code;
    let scope = &db_op.scope;
    let table_name = &db_op.table_name;
    let primary_key = &db_op.primary_key;
    let old_payer = &db_op.old_payer;
    let new_payer = &db_op.new_payer;
    let old_data = Hex::encode(&db_op.old_data.to_vec());
    let new_data = Hex::encode(&db_op.new_data.to_vec());
    let old_data_json = &db_op.old_data_json;
    let new_data_json = &db_op.new_data_json;

    // transaction
    let tx_hash = &transaction.id;

    let keys = db_ops_keys(&tx_hash, &index);
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

    insert_timestamp(row, clock, false, false);
    insert_transaction_metadata(row, transaction);
}

pub fn collect_db_ops(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawDbOp> {
    let mut db_ops = Vec::new();

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for (index, db_op) in transaction.db_ops.iter().enumerate() {
            db_ops.push(RawDbOp {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: tx_hash.clone(),
                tx_success,
                index: index as u32,
                operation: operation_to_string(db_op.operation),
                operation_code: db_op.operation as u32,
                action_index: db_op.action_index,
                code: db_op.code.clone(),
                scope: db_op.scope.clone(),
                table_name: db_op.table_name.clone(),
                primary_key: db_op.primary_key.clone(),
                old_payer: db_op.old_payer.clone(),
                new_payer: db_op.new_payer.clone(),
                old_data: Hex::encode(&db_op.old_data),
                new_data: Hex::encode(&db_op.new_data),
                old_data_json: db_op.old_data_json.clone(),
                new_data_json: db_op.new_data_json.clone(),
            });
        }
    }

    db_ops
}
