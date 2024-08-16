use common::blocks::{insert_timestamp, insert_transaction_counts};
use common::utils::{bytes_to_hex, optional_u64_to_string};
use common::{keys::blocks_keys, utils::optional_bigint_to_string};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::Block;

use crate::balance_changes::{insert_balance_change, insert_balance_change_counts};
use crate::code_changes::insert_code_change;
use crate::traces::insert_system_trace;

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
pub fn insert_blocks(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, block_header_only: bool) {
    let header = block.header.clone().unwrap_or_default();
    let parent_hash = bytes_to_hex(&header.parent_hash);
    let nonce = header.nonce;
    let ommers_hash = bytes_to_hex(&header.uncle_hash);
    let logs_bloom = bytes_to_hex(&header.logs_bloom);
    let transactions_root = bytes_to_hex(&header.transactions_root);
    let state_root = bytes_to_hex(&header.state_root);
    let receipts_root = bytes_to_hex(&header.receipt_root);
    let miner = bytes_to_hex(&header.coinbase); // EVM Address
    let size = block.size;
    let mix_hash = bytes_to_hex(&header.mix_hash);
    let extra_data = bytes_to_hex(&header.extra_data.clone());
    let extra_data_utf8 = String::from_utf8(header.extra_data).unwrap_or_default();
    let gas_limit = header.gas_limit;
    let gas_used = header.gas_used;
    let difficulty = optional_bigint_to_string(header.difficulty, "0"); // UInt64
    let total_difficulty = optional_bigint_to_string(header.total_difficulty.clone(), "0"); // UInt256

    // block detail levels
    // https://streamingfastio.medium.com/new-block-model-to-accelerate-chain-integration-9f65126e5425
    let detail_level_code = block.detail_level;
    let detail_level = block_detail_to_string(detail_level_code);

    // forks
    let withdrawals_root = bytes_to_hex(&header.withdrawals_root); // EIP-4895 (Shangai Fork)
    let parent_beacon_root = bytes_to_hex(&header.parent_beacon_root); // EIP-4788 (Dencun Fork)
    let base_fee_per_gas = optional_bigint_to_string(header.base_fee_per_gas, ""); // UInt256 - EIP-1559 (London Fork)
    let excess_blob_gas = optional_u64_to_string(header.excess_blob_gas, ""); // UInt64 - EIP-4844 (Dencun Fork)
    let blob_gas_used = optional_u64_to_string(header.blob_gas_used, ""); // UInt64 - EIP-4844 (Dencun Fork)

    // blocks
    let keys = blocks_keys(&clock, true);
    let row = tables
        .push_change_composite("blocks", keys, 0, table_change::Operation::Create)
        .change("parent_hash", ("", parent_hash.as_str()))
        .change("nonce", ("", nonce.to_string().as_str()))
        .change("ommers_hash", ("", ommers_hash.as_str()))
        .change("logs_bloom", ("", logs_bloom.as_str()))
        .change("transactions_root", ("", transactions_root.as_str()))
        .change("state_root", ("", state_root.as_str()))
        .change("receipts_root", ("", receipts_root.as_str()))
        .change("miner", ("", miner.as_str()))
        .change("size", ("", size.to_string().as_str()))
        .change("mix_hash", ("", mix_hash.as_str()))
        .change("extra_data", ("", extra_data.as_str()))
        .change("extra_data_utf8", ("", extra_data_utf8.as_str()))
        .change("gas_limit", ("", gas_limit.to_string().as_str()))
        .change("gas_used", ("", gas_used.to_string().as_str()))
        .change("difficulty", ("", difficulty.as_str()))
        .change("total_difficulty", ("", total_difficulty.as_str()))
        // EIP-1559 (London Fork)
        .change("base_fee_per_gas", ("", base_fee_per_gas.as_str()))
        // EIP-4895 (Shangai Fork)
        .change("withdrawals_root", ("", withdrawals_root.as_str()))
        // EIP-4844 & EIP-4788 (Dencun Fork)
        .change("parent_beacon_root", ("", parent_beacon_root.as_str()))
        .change("excess_blob_gas", ("", excess_blob_gas.as_str()))
        .change("blob_gas_used", ("", blob_gas_used.as_str()))

        // block detail levels
        .change("detail_level", ("", detail_level.as_str()))
        .change("detail_level_code", ("", detail_level_code.to_string().as_str()))
        ;

    insert_timestamp(row, clock, true);

    // transaction status counts
    let all_transaction_status: Vec<i32> = block.transaction_traces.iter().map(|transaction| transaction.status).collect();
    insert_transaction_counts(row, all_transaction_status);

    // balance changes counts
    let all_balance_changes_reason: Vec<i32> = block.balance_changes.iter().map(|balance_change| balance_change.reason).collect();
    insert_balance_change_counts(row, all_balance_changes_reason);

    // skip the rest of the block if we only want the header
    if block_header_only {
        return;
    }

    // TABLE::code_changes
    for code_change in block.code_changes.iter() {
        insert_code_change(tables, clock, code_change);
    }
    // TABLE::traces
    for system_call in block.system_calls.iter() {
        insert_system_trace(tables, clock, system_call);
    }
    // TABLE::balance_changes
    for balance_change in block.balance_changes.iter() {
        insert_balance_change(tables, clock, balance_change);
    }
}
