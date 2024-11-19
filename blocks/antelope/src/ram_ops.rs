use common::structs::BlockTimestamp;
use substreams_antelope::Block;

use crate::pb::antelope::RamOp as RawRamOp;
use crate::transactions::is_transaction_success;

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

pub fn collect_ram_ops(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawRamOp> {
    let mut ram_ops: Vec<RawRamOp> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for ram_op in transaction.ram_ops.iter() {
            ram_ops.push(RawRamOp {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: tx_hash.clone(),
                tx_success,
                operation: operation_to_string(ram_op.operation),
                action_index: ram_op.action_index,
                payer: ram_op.payer.clone(),
                delta: ram_op.delta,
                usage: ram_op.usage,
                namespace: namespace_to_string(ram_op.namespace),
                action: action_to_string(ram_op.action),
                unique_key: ram_op.unique_key.clone(),
                operation_code: ram_op.operation as u32,
                namespace_code: ram_op.namespace as u32,
                action_code: ram_op.action as u32,
            });
        }
    }

    ram_ops
}
