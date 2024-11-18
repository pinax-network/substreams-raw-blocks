use crate::{
    authority::insert_authority,
    keys::perm_ops_keys,
    pb::antelope::PermOp as RawPermOp,
    transactions::{insert_transaction_metadata, is_transaction_success},
};
use common::{blocks::insert_timestamp, structs::BlockTimestamp};
use substreams::pb::substreams::Clock;
use substreams_antelope::{
    pb::{PermOp, TransactionTrace},
    Block,
};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn perm_op_operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Update".to_string(),
        3 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn insert_perm_op(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace, perm_op: &PermOp) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_index = perm_op.action_index;

    // perm_op
    let operation = perm_op_operation_to_string(perm_op.operation);
    match &perm_op.new_perm {
        Some(new_perm) => {
            let keys = perm_ops_keys(tx_hash, &action_index);
            let threshold = new_perm.authority.as_ref().map_or(0, |authority| authority.threshold);
            let row = tables
                .push_change_composite("perm_ops", keys, 0, table_change::Operation::Create)
                .change("operation", ("", operation.as_str()))
                .change("operation_code", ("", perm_op.operation.to_string().as_str()))
                .change("action_index", ("", action_index.to_string().as_str()))
                .change("id", ("", new_perm.id.to_string().as_str()))
                .change("parent_id", ("", new_perm.parent_id.to_string().as_str()))
                .change("owner", ("", new_perm.owner.to_string().as_str()))
                .change("name", ("", new_perm.name.to_string().as_str()))
                .change("threshold", ("", threshold.to_string().as_str()));
            insert_transaction_metadata(row, transaction);
            insert_timestamp(row, clock, false, false);

            match &new_perm.authority {
                Some(authority) => {
                    insert_authority(tables, clock, transaction, action_index, authority);
                }
                None => {}
            }
        }
        None => {}
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
