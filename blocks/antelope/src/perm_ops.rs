use crate::{pb::antelope::PermOp as RawPermOp, transactions::is_transaction_success};
use common::structs::BlockTimestamp;
use substreams_antelope::Block;

pub fn perm_op_operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Update".to_string(),
        3 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn collect_perm_ops(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawPermOp> {
    let mut perm_ops: Vec<RawPermOp> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for perm_op in transaction.perm_ops.iter() {
            if let Some(new_perm) = &perm_op.new_perm {
                let threshold = new_perm.authority.as_ref().map_or(0, |authority| authority.threshold);

                perm_ops.push(RawPermOp {
                    block_time: Some(timestamp.time.clone()),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    tx_hash: tx_hash.clone(),
                    tx_success,
                    operation: perm_op_operation_to_string(perm_op.operation),
                    operation_code: perm_op.operation as u32,
                    action_index: perm_op.action_index,
                    id: new_perm.id,
                    parent_id: new_perm.parent_id,
                    owner: new_perm.owner.clone(),
                    name: new_perm.name.clone(),
                    threshold,
                });
            }
        }
    }

    perm_ops
}
