use common::{blocks::insert_timestamp, structs::BlockTimestamp};
use substreams::pb::substreams::Clock;
use substreams_antelope::{
    pb::{TableOp, TransactionTrace},
    Block,
};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{
    keys::table_ops_keys,
    pb::antelope::TableOp as RawTableOp,
    transactions::{insert_transaction_metadata, is_transaction_success},
};

pub fn table_op_operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn insert_table_op(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace, table_op: &TableOp, index: &u32) {
    let action_index = &table_op.action_index;
    let operation = &table_op_operation_to_string(table_op.operation);
    let payer = &table_op.payer;
    let code = &table_op.code;
    let scope = &table_op.scope;
    let table_name = &table_op.table_name;

    // transaction
    let tx_hash = &transaction.id;

    let keys = table_ops_keys(tx_hash, index);
    let row = tables
        .push_change_composite("table_ops", keys, 0, table_change::Operation::Create)
        .change("index", ("", index.to_string().as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("operation", ("", operation.as_str()))
        .change("operation_code", ("", table_op.operation.to_string().as_str()))
        .change("payer", ("", payer.as_str()))
        .change("code", ("", code.as_str()))
        .change("scope", ("", scope.as_str()))
        .change("table_name", ("", table_name.as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}

pub fn collect_table_ops(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawTableOp> {
    let mut table_ops: Vec<RawTableOp> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for (index, table_op) in transaction.table_ops.iter().enumerate() {
            table_ops.push(RawTableOp {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: tx_hash.clone(),
                tx_success,
                index: index as u32,
                action_index: table_op.action_index,
                operation: table_op_operation_to_string(table_op.operation),
                operation_code: table_op.operation as u32,
                payer: table_op.payer.clone(),
                code: table_op.code.clone(),
                scope: table_op.scope.clone(),
                table_name: table_op.table_name.clone(),
            });
        }
    }

    table_ops
}
