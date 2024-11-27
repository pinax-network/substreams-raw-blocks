use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;

use crate::pb::pinax::antelope::v1::TableOp;

pub fn table_op_operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn collect_tx_table_ops(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<TableOp> {
    let mut table_ops = Vec::new();

    for (index, table_op) in transaction.table_ops.iter().enumerate() {
        table_ops.push(TableOp {
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),
            tx_hash: transaction.id.clone(),
            tx_success,
            index: index as u32,
            action_index: table_op.action_index,
            operation: table_op_operation_to_string(table_op.operation),
            operation_code: table_op.operation,
            payer: table_op.payer.clone(),
            code: table_op.code.clone(),
            scope: table_op.scope.clone(),
            table_name: table_op.table_name.clone(),
        });
    }

    table_ops
}
