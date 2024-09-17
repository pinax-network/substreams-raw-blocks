use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{RamOp, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::ram_op_keys;
use crate::transactions::insert_transaction_metadata;

pub fn namespace_to_string(namespace: i32) -> String {
    match namespace {
        0 => "Unknown".to_string(),
        1 => "Abi".to_string(),
        2 => "Account".to_string(),
        3 => "Auth".to_string(),
        4 => "AuthLink".to_string(),
        5 => "Code".to_string(),
        6 => "DeferredTrx".to_string(),
        7 => "SecondaryIndex".to_string(),
        8 => "Table".to_string(),
        9 => "TableRow".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn action_to_string(action: i32) -> String {
    match action {
        0 => "Unknown".to_string(),
        1 => "Add".to_string(),
        2 => "Cancel".to_string(),
        3 => "Correction".to_string(),
        4 => "Push".to_string(),
        5 => "Remove".to_string(),
        6 => "Update".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "CreateTable".to_string(),
        2 => "DeferredTrxAdd".to_string(),
        3 => "DeferredTrxCancel".to_string(),
        4 => "DeferredTrxPushed".to_string(),
        5 => "DeferredTrxRamCorrection".to_string(),
        6 => "DeferredTrxRemoved".to_string(),
        7 => "DeleteAuth".to_string(),
        8 => "LinkAuth".to_string(),
        9 => "NewAccount".to_string(),
        10 => "PrimaryIndexAdd".to_string(),
        11 => "PrimaryIndexRemove".to_string(),
        12 => "PrimaryIndexUpdate".to_string(),
        13 => "PrimaryIndexUpdateAddNewPayer".to_string(),
        14 => "PrimaryIndexUpdateRemoveOldPayer".to_string(),
        15 => "RemoveTable".to_string(),
        16 => "SecondaryIndexAdd".to_string(),
        17 => "SecondaryIndexRemove".to_string(),
        18 => "SecondaryIndexUpdateAddNewPayer".to_string(),
        19 => "SecondaryIndexUpdateRemoveOldPayer".to_string(),
        20 => "SetAbi".to_string(),
        21 => "SetCode".to_string(),
        22 => "UnlinkAuth".to_string(),
        23 => "UpdateAuthCreate".to_string(),
        24 => "UpdateAuthUpdate".to_string(),
        25 => "Deprecated".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn insert_ram_op(tables: &mut DatabaseChanges, clock: &Clock, ram_op: &RamOp, transaction: &TransactionTrace) {
    let operation = operation_to_string(ram_op.operation);
    let action_index = ram_op.action_index;
    let payer = &ram_op.payer;
    let delta = ram_op.delta;
    let usage = ram_op.usage;
    let namespace = namespace_to_string(ram_op.namespace);
    let action = action_to_string(ram_op.action);
    let unique_key = &ram_op.unique_key;

    let keys = ram_op_keys(&transaction.id, &action_index, unique_key);
    let row = tables
        .push_change_composite("ram_ops", keys, 0, table_change::Operation::Create)
        .change("operation", ("", operation.as_str()))
        .change("operation_code", ("", ram_op.operation.to_string().as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("payer", ("", payer.as_str()))
        .change("delta", ("", delta.to_string().as_str()))
        .change("usage", ("", usage.to_string().as_str()))
        .change("namespace", ("", namespace.as_str()))
        .change("namespace_code", ("", ram_op.namespace.to_string().as_str()))
        .change("action", ("", action.as_str()))
        .change("action_code", ("", ram_op.action.to_string().as_str()))
        .change("unique_key", ("", unique_key.as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}
