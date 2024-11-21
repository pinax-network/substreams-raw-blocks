use common::structs::BlockTimestamp;
use substreams_antelope::pb::{PermissionLevel, TransactionTrace};

use crate::pb::antelope::PermOp;

pub fn perm_op_operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Update".to_string(),
        3 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn permission_to_string(permission: PermissionLevel) -> String {
    format!("{}@{}", permission.actor, permission.permission)
}

pub fn collect_tx_perm_ops(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<PermOp> {
    let mut perm_ops = Vec::new();

    for perm_op in transaction.perm_ops.iter() {
        if let Some(new_perm) = &perm_op.new_perm {
            // TEST: Check if authority is required
            let authority = new_perm.authority.as_ref().unwrap();
            let threshold = authority.threshold;

            // accounts
            let accounts = authority.accounts.iter().map(|perm| permission_to_string(perm.clone().permission.unwrap())).collect::<Vec<String>>();
            let accounts_weight = authority.accounts.iter().map(|perm| perm.weight).collect::<Vec<u32>>();

            // keys
            let keys_public_key = authority.keys.iter().map(|perm| perm.clone().public_key).collect::<Vec<String>>();
            let keys_weight = authority.keys.iter().map(|perm| perm.weight).collect::<Vec<u32>>();

            // waits
            let wait_sec = authority.waits.iter().map(|perm| perm.wait_sec).collect::<Vec<u32>>();
            let wait_weight = authority.waits.iter().map(|perm| perm.weight).collect::<Vec<u32>>();

            perm_ops.push(PermOp {
                // block
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),

                // transaction
                tx_hash: transaction.id.clone(),
                tx_success,

                // action
                action_index: perm_op.action_index,

                // permission operation
                operation: perm_op_operation_to_string(perm_op.operation),
                operation_code: perm_op.operation as u32,
                id: new_perm.id,
                parent_id: new_perm.parent_id,
                owner: new_perm.owner.clone(),
                name: new_perm.name.clone(),
                threshold,

                // -- authority --
                // accounts
                accounts,
                accounts_weight,

                // keys
                keys_public_key,
                keys_weight,

                // waits
                wait_sec,
                wait_weight,
            });
        }
    }

    perm_ops
}
