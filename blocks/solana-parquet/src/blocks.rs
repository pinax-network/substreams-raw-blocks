use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::TableChange;
use substreams_solana::{b58, pb::sf::solana::r#type::v1::Block};

use crate::structs::BlockTimestamp;
use crate::utils::get_timestamp_without_number;
use crate::{pb::solana::rawblocks::Block as RawBlock, structs::BlockInfo};

use crate::counters::get_block_counters;

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

pub fn get_block_info(block: &Block) -> BlockInfo {
    BlockInfo {
        slot: block.slot,
        height: block.block_height.as_ref().expect("Block height is missing").block_height,
        previous_block_hash: block.previous_blockhash.clone(),
        parent_slot: block.parent_slot,
    }
}

pub fn insert_blockinfo(row: &mut TableChange, block: &Block, with_prefix: bool) {
    let height = block.block_height.as_ref().map_or_else(|| "0".to_string(), |bh| bh.block_height.to_string());

    let prefix_str = if with_prefix { "block_" } else { "" };
    row.change(format!("{}slot", prefix_str).as_str(), ("", block.slot.to_string().as_str()))
        .change(format!("{}height", prefix_str).as_str(), ("", height.as_str()))
        .change(format!("{}previous_block_hash", prefix_str).as_str(), ("", block.previous_blockhash.as_str()))
        .change(format!("{}parent_slot", prefix_str).as_str(), ("", block.parent_slot.to_string().as_str()));
}

pub fn collect_block(block: &Block, timestamp: &BlockTimestamp, block_info: &BlockInfo) -> Option<RawBlock> {
    let counters = get_block_counters(block);

    Some(RawBlock {
        time: Some(timestamp.time),
        date: timestamp.date.clone(),
        hash: timestamp.hash.clone(),
        slot: block.slot,
        height: block_info.height,
        previous_block_hash: block_info.previous_block_hash.clone(),
        parent_slot: block_info.parent_slot,
        total_transactions: counters.total_transactions,
        successful_transactions: counters.successful_transactions,
        failed_transactions: counters.failed_transactions,
        total_vote_transactions: counters.total_vote_transactions,
        total_non_vote_transactions: counters.total_non_vote_transactions,
        successful_vote_transactions: counters.successful_vote_transactions,
        successful_non_vote_transactions: counters.successful_non_vote_transactions,
        failed_vote_transactions: counters.failed_vote_transactions,
        failed_non_vote_transactions: counters.failed_non_vote_transactions,
    })
}
