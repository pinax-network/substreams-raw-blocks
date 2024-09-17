use crate::{keys::perm_ops_keys, transactions::insert_transaction_metadata};
use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{PermOp, TransactionTrace};
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
            let keys = perm_ops_keys(clock, tx_hash, &action_index);
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

            // TO-DO: implement Authority
            match &new_perm.authority {
                Some(authority) => {
                    for account in &authority.accounts {
                        // TO-DO: INSERT implement Authority Accounts
                        let permission_level = account.permission.as_ref().expect("account.permission is missing");
                        let actor = permission_level.actor.as_str();
                        let permission = permission_level.permission.as_str();
                        let weight = account.weight;
                    }
                    for key in &authority.keys {
                        // TO-DO: INSERT implement Authority Keys
                        let permission = key.public_key.as_str();
                        let weight = key.weight;
                    }
                    for wait in &authority.waits {
                        // TO-DO: INSERT implement Authority waits
                        let wait_sec = wait.wait_sec;
                        let weight = wait.weight;
                    }
                }
                None => {}
            }
        }
        None => {}
    }
}
