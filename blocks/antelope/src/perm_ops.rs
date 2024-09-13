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

    let old_perm = perm_op.old_perm.clone().unwrap_or_default();
    let new_perm = perm_op.new_perm.clone().unwrap_or_default();
    let old_nanos = old_perm.last_updated.as_ref().map_or(0, |t| t.nanos);
    let old_seconds = old_perm.last_updated.as_ref().map_or(0, |t| t.seconds);
    let new_nanos = new_perm.last_updated.as_ref().map_or(0, |t| t.nanos);
    let new_seconds = new_perm.last_updated.as_ref().map_or(0, |t| t.seconds);

    let old_perm_last_updated_ms = old_seconds * 1000 + i64::from(old_nanos) / 1_000_000;
    let new_perm_last_updated_ms = new_seconds * 1000 + i64::from(new_nanos) / 1_000_000;

    let operation = perm_op_operation_to_string(perm_op.operation);

    let keys = perm_ops_keys(clock, tx_hash, &action_index, &operation);
    let row = tables
        .push_change_composite("perm_ops", keys, 0, table_change::Operation::Create)
        .change("tx_hash", ("", tx_hash.as_str()))
        .change("operation", ("", operation.as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("old_perm_id", ("", old_perm.id.to_string().as_str()))
        .change("old_perm_parent_id", ("", old_perm.parent_id.to_string().as_str()))
        .change("old_perm_owner", ("", old_perm.owner.to_string().as_str()))
        .change("old_perm_name", ("", old_perm.name.to_string().as_str()))
        .change("old_perm_last_updated", ("", old_perm_last_updated_ms.to_string().as_str()))
        .change("new_perm_id", ("", new_perm.id.to_string().as_str()))
        .change("new_perm_parent_id", ("", new_perm.parent_id.to_string().as_str()))
        .change("new_perm_owner", ("", new_perm.owner.to_string().as_str()))
        .change("new_perm_last_updated", ("", new_perm_last_updated_ms.to_string().as_str()))
        .change("new_perm_name", ("", new_perm.name.to_string().as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}
