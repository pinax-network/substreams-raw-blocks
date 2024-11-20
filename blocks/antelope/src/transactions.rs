use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_antelope::{pb::TransactionTrace, Block};

use crate::pb::antelope::Transaction;

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
pub fn collect_transaction(block: &Block, transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Transaction {
    let header = block.header.clone().unwrap_or_default();
    let receipt = transaction.receipt.clone().unwrap_or_default();
    let status_code = receipt.status;
    let status = transaction_status_to_string(status_code);

    Transaction {
        block_time: Some(timestamp.time.clone()),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),
        hash: transaction.id.clone(),
        index: transaction.index as u64,
        elapsed: transaction.elapsed,
        net_usage: transaction.net_usage,
        scheduled: transaction.scheduled,
        cpu_usage_micro_seconds: receipt.cpu_usage_micro_seconds,
        net_usage_words: receipt.net_usage_words,
        status,
        status_code: status_code as u32,
        success: tx_success,
        transaction_mroot: Hex::encode(&header.transaction_mroot.to_vec()),
    }
}
