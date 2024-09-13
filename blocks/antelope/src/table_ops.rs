use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::TableOp;
use substreams_antelope::pb::TransactionTrace;
use substreams_database_change::pb::database::table_change;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::keys::table_ops_keys;
use crate::transactions::insert_transaction_metadata;

pub fn table_op_operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn insert_table_op(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace, table_op: &TableOp) {
    let action_index = &table_op.action_index;
    let operation = &table_op_operation_to_string(table_op.operation);
    let payer = &table_op.payer;
    let code = &table_op.code;
    let scope = &table_op.scope;
    let table_name = &table_op.table_name;

    let tx_hash = &transaction.id;

    let keys = table_ops_keys(clock, tx_hash, action_index, operation, payer, code, scope, table_name);
    let row = tables
        .push_change_composite("table_ops", keys, 0, table_change::Operation::Create)
        .change("operation", ("", operation.to_string().as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("payer", ("", payer.as_str()))
        .change("code", ("", code.as_str()))
        .change("scope", ("", scope.as_str()))
        .change("table_name", ("", table_name.as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, true);
}
