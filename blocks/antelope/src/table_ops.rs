use common::structs::BlockTimestamp;
use substreams_antelope::Block;

use crate::{pb::antelope::TableOp as RawTableOp, transactions::is_transaction_success};

pub fn table_op_operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
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
