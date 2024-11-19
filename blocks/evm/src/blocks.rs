use common::structs::BlockTimestamp;
use common::utils::optional_bigint_to_string;
use common::utils::{bytes_to_hex, optional_bigint_to_u64, optional_u64_to_string};

use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::evm::Block as BlockHeader;

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

    let total_transactions = block.transaction_traces.len() as u64;
    let successful_transactions = block.transaction_traces.iter().filter(|t| t.status == 1).count() as u64;
    let failed_transactions = total_transactions - successful_transactions;
    let total_withdrawals = block.balance_changes.iter().filter(|t| t.reason == 16).count() as u64;

    BlockHeader {
        time: Some(timestamp.time),
        number: header.number,
        date: timestamp.date.clone(),
        hash: bytes_to_hex(&block.hash),
        parent_hash: bytes_to_hex(&header.parent_hash),
        nonce: header.nonce,
        ommers_hash: bytes_to_hex(&header.uncle_hash),
        logs_bloom: bytes_to_hex(&header.logs_bloom),
        transactions_root: bytes_to_hex(&header.transactions_root),
        state_root: bytes_to_hex(&header.state_root),
        receipts_root: bytes_to_hex(&header.receipt_root),
        withdrawals_root: bytes_to_hex(&header.withdrawals_root),
        parent_beacon_root: bytes_to_hex(&header.parent_beacon_root),
        miner: bytes_to_hex(&header.coinbase),
        difficulty: optional_bigint_to_u64(&header.difficulty),
        total_difficulty: optional_bigint_to_string(&header.total_difficulty, "0"),
        mix_hash: bytes_to_hex(&header.mix_hash),
        extra_data: bytes_to_hex(&header.extra_data),
        extra_data_utf8: String::from_utf8(header.extra_data.clone()).unwrap_or_default(),
        gas_limit: header.gas_limit,
        gas_used: header.gas_used,
        base_fee_per_gas: optional_bigint_to_string(&header.base_fee_per_gas, ""),
        blob_gas_used: optional_u64_to_string(&header.blob_gas_used, ""),
        excess_blob_gas: optional_u64_to_string(&header.excess_blob_gas, ""),
        size: block.size,
        total_transactions: block.transaction_traces.len() as u64,
        successful_transactions,
        failed_transactions,
        total_balance_changes: block.balance_changes.len() as u64,
        total_withdrawals,
        detail_level: block_detail_to_string(block.detail_level),
        detail_level_code: block.detail_level as u32,
    }
}
