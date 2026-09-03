use common::structs::BlockIdentity;
use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::pinax::evm::v2::{Block as BlockRow, Uncle};
use crate::utils::{detail_level_text, optional_bigint_to_string};

// https://github.com/streamingfast/firehose-ethereum/blob/develop/proto/sf/ethereum/type/v2/type.proto
// DetailLevel: BASE
// `BlockHeader::total_difficulty` is deprecated upstream (constant since the merge, dropped by geth v1.15),
// but raw blocks keep extracting it for historical blocks where it is set.
#[allow(deprecated)]
pub fn collect_block(block: &Block, id: &BlockIdentity) -> BlockRow {
    let header = block.header.clone().unwrap_or_default();

    BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        number: block.number,
        hash: block.hash.clone(),
        parent_hash: header.parent_hash.clone(),
        gas_used: header.gas_used,
        gas_limit: header.gas_limit,
        base_fee_per_gas: optional_bigint_to_string(&header.base_fee_per_gas),
        coinbase: header.coinbase.clone(),
        size: block.size,
        nonce: header.nonce,
        state_root: header.state_root.clone(),
        transactions_root: header.transactions_root.clone(),
        receipt_root: header.receipt_root.clone(),
        difficulty: optional_bigint_to_string(&header.difficulty),
        mix_hash: header.mix_hash.clone(),
        extra_data: header.extra_data.clone(),
        num_transactions: block.transaction_traces.len() as u32,
        detail_level: detail_level_text(block.detail_level),

        uncle_hash: header.uncle_hash.clone(),
        logs_bloom: header.logs_bloom.clone(),
        total_difficulty: optional_bigint_to_string(&header.total_difficulty),
        withdrawals_root: header.withdrawals_root.clone(),
        parent_beacon_root: header.parent_beacon_root.clone(),
        blob_gas_used: header.blob_gas_used,
        excess_blob_gas: header.excess_blob_gas,
        requests_hash: header.requests_hash.clone(),
        num_uncles: block.uncles.len() as u32,
        ver: block.ver,
    }
}

pub fn collect_uncles(block: &Block, id: &BlockIdentity) -> Vec<Uncle> {
    block
        .uncles
        .iter()
        .enumerate()
        .map(|(index, uncle)| Uncle {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: block.number,
            index: index as u32,
            hash: uncle.hash.clone(),
            number: uncle.number,
            parent_hash: uncle.parent_hash.clone(),
            coinbase: uncle.coinbase.clone(),
            difficulty: optional_bigint_to_string(&uncle.difficulty),
            gas_limit: uncle.gas_limit,
            gas_used: uncle.gas_used,
            uncle_timestamp: uncle.timestamp.clone(),
            state_root: uncle.state_root.clone(),
            transactions_root: uncle.transactions_root.clone(),
            receipt_root: uncle.receipt_root.clone(),
            mix_hash: uncle.mix_hash.clone(),
            nonce: uncle.nonce,
            extra_data: uncle.extra_data.clone(),
        })
        .collect()
}
