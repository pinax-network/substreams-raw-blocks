use common::structs::BlockIdentity;
use substreams_antelope::pb::TransactionTrace;

use crate::pb::pinax::antelope::v2::DbOp;
use crate::utils::{db_op_operation_text, non_empty, non_empty_bytes};

pub fn collect_db_ops(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<DbOp> {
    tx.db_ops
        .iter()
        .enumerate()
        .map(|(index, op)| DbOp {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            operation: db_op_operation_text(op.operation),
            action_index: op.action_index,
            code: op.code.clone(),
            scope: op.scope.clone(),
            table_name: op.table_name.clone(),
            primary_key: op.primary_key.clone(),
            old_payer: op.old_payer.clone(),
            new_payer: op.new_payer.clone(),
            old_data: non_empty_bytes(&op.old_data),
            new_data: non_empty_bytes(&op.new_data),
            old_data_json: non_empty(&op.old_data_json),
            new_data_json: non_empty(&op.new_data_json),

            tx_hash: tx.id.clone(),
            tx_index: tx.index,
            index: index as u32,
        })
        .collect()
}
