use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::structs::BlockTimestamp;
use crate::{pb::solana::Block as RawBlock, structs::BlockInfo};

use crate::counters::get_block_counters;

pub fn get_block_info(block: &Block) -> BlockInfo {
    BlockInfo {
        slot: block.slot,
        height: block.block_height.as_ref().expect("Block height is missing").block_height,
        previous_block_hash: block.previous_blockhash.clone(),
        parent_slot: block.parent_slot,
    }
}

pub fn collect_block(block: &Block, timestamp: &BlockTimestamp, block_info: &BlockInfo) -> RawBlock {
    let counters = get_block_counters(block);

    RawBlock {
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
    }
}
