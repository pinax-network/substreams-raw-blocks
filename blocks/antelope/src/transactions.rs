use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_antelope::{pb::TransactionTrace, Block};

use crate::pb::pinax::antelope::v1::Transaction;

pub fn transaction_status_to_string(status: i32) -> String {
    match status {
        0 => "None".to_string(),
        1 => "Executed".to_string(),
        2 => "Softfail".to_string(),
        3 => "Hardfail".to_string(),
        4 => "Delayed".to_string(),
        5 => "Expired".to_string(),
        6 => "Unknown".to_string(),
        7 => "Canceled".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn is_transaction_success(status: i32) -> bool {
    status == 1
}

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn collect_transaction(block: &Block, transaction: &TransactionTrace, timestamp: &BlockTimestamp, success: bool) -> Transaction {
    let header = block.header.clone().unwrap_or_default();
    let receipt = transaction.receipt.clone().unwrap_or_default();
    let status_code = receipt.status;
    let status = transaction_status_to_string(status_code);

    // make execution_action_index as vector
    let creator_action_indexes = transaction.creation_tree.iter().map(|tree| tree.creator_action_index).collect::<Vec<i32>>();
    let execution_action_indexes = transaction.creation_tree.iter().map(|tree| tree.execution_action_index).collect::<Vec<u32>>();

    Transaction {
        // block
        block_time: timestamp.time.to_string(),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        hash: transaction.id.clone(),
        index: transaction.index,
        elapsed: transaction.elapsed,
        net_usage: transaction.net_usage,
        scheduled: transaction.scheduled,
        cpu_usage_micro_seconds: receipt.cpu_usage_micro_seconds,
        net_usage_words: receipt.net_usage_words,
        status,
        status_code,
        success,
        transaction_mroot: Hex::encode(&header.transaction_mroot.to_vec()),

        // creation flat node
        creator_action_indexes,
        execution_action_indexes,
    }
}
