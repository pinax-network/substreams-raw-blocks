use common::structs::BlockTimestamp;
use common::utils::optional_bigint_to_string;
use common::utils::{bytes_to_hex, optional_bigint_to_u64};

// use substreams::log;
use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::pinax::evm::v1::Block as BlockHeader;

pub fn block_detail_to_string(detail_level: i32) -> String {
    match detail_level {
        0 => "Extended".to_string(),
        1 => "Trace".to_string(),
        2 => "Base".to_string(),
        _ => "Base".to_string(),
    }
}

// https://github.com/streamingfast/firehose-ethereum/blob/develop/proto/sf/ethereum/type/v2/type.proto
// DetailLevel: BASE
pub fn collect_block(block: &Block, timestamp: &BlockTimestamp) -> BlockHeader {
    let header = block.header.as_ref().unwrap();

    // counters
    let total_transactions = block.transaction_traces.len() as u32;
    let successful_transactions = block.transaction_traces.iter().filter(|t| t.status == 1).count() as u32;
    let failed_transactions = total_transactions - successful_transactions;
    let total_withdrawals = block.balance_changes.iter().filter(|t| t.reason == 16).count() as u32;

    // blob price
    let blob_gas_price = block.transaction_traces.iter().find_map(|t| t.receipt.as_ref().and_then(|r| r.blob_gas_price.clone()));
    let blob_hashes: Vec<String> = block.transaction_traces.iter().flat_map(|t| t.blob_hashes.iter().map(|hash| bytes_to_hex(hash))).collect();
    let total_blobs: u32 = blob_hashes.len() as u32;
    let blob_transactions = block.transaction_traces.iter().filter(|t| t.r#type == 3).map(|t| bytes_to_hex(&t.hash)).collect::<Vec<String>>();
    let total_blob_transactions = blob_transactions.len() as u32;

    BlockHeader {
        // clock
        time: timestamp.time.to_string(),
        number: header.number,
        date: timestamp.date.clone(),
        hash: bytes_to_hex(&block.hash),

        // roots
        ommers_hash: bytes_to_hex(&header.uncle_hash),
        logs_bloom: bytes_to_hex(&header.logs_bloom),
        transactions_root: bytes_to_hex(&header.transactions_root),
        state_root: bytes_to_hex(&header.state_root),
        receipts_root: bytes_to_hex(&header.receipt_root),
        withdrawals_root: bytes_to_hex(&header.withdrawals_root),
        parent_beacon_root: bytes_to_hex(&header.parent_beacon_root),

        // header
        parent_hash: bytes_to_hex(&header.parent_hash),
        nonce: header.nonce,
        miner: bytes_to_hex(&header.coinbase),
        difficulty: optional_bigint_to_u64(&header.difficulty),
        total_difficulty_hex: bytes_to_hex(&header.total_difficulty.clone().unwrap_or_default().bytes),
        mix_hash: bytes_to_hex(&header.mix_hash),
        extra_data: bytes_to_hex(&header.extra_data),
        extra_data_utf8: String::from_utf8(header.extra_data.clone()).unwrap_or_default(),
        gas_limit: header.gas_limit,
        gas_used: header.gas_used,
        base_fee_per_gas: optional_bigint_to_string(&header.base_fee_per_gas, ""),

        // blobs
        blob_gas_used: header.blob_gas_used(),
        excess_blob_gas: header.excess_blob_gas(),
        blob_gas_price_hex: bytes_to_hex(&blob_gas_price.unwrap_or_default().bytes),
        blob_hashes,
        blob_transactions,
        total_blob_transactions,
        total_blobs,

        // counters
        size: block.size,
        total_transactions: block.transaction_traces.len() as u32,
        successful_transactions,
        failed_transactions,
        total_balance_changes: block.balance_changes.len() as u32,
        total_withdrawals,

        // block detail level
        detail_level: block_detail_to_string(block.detail_level),
        detail_level_code: block.detail_level as u32,
    }
}
