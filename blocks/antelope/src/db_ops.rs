use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_antelope::pb::TransactionTrace;

use crate::pb::antelope::DbOp;

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
pub fn collect_tx_db_ops(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<DbOp> {
    let mut db_ops: Vec<DbOp> = Vec::new();

    for (index, db_op) in transaction.db_ops.iter().enumerate() {
        db_ops.push(DbOp {
            // block
            block_time: Some(timestamp.time.clone()),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),

            // transaction
            tx_hash: transaction.id.clone(),
            tx_success,

            // database operation
            index: index as u32,
            operation: operation_to_string(db_op.operation),
            operation_code: db_op.operation,
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

    db_ops
}
