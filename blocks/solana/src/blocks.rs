use common::structs::BlockIdentity;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::pb::pinax::solana::v2::Block as BlockRow;
use crate::utils::{base58_to_bytes, is_vote_transaction};

pub fn collect_block(block: &Block, id: &BlockIdentity) -> BlockRow {
    let mut successful = 0u32;
    let mut votes = 0u32;
    let mut successful_votes = 0u32;
    for tx in &block.transactions {
        let ok = tx.meta.as_ref().map_or(true, |m| m.err.as_ref().map_or(true, |e| e.err.is_empty()));
        let vote = tx.transaction.as_ref().and_then(|t| t.message.as_ref()).is_some_and(is_vote_transaction);
        successful += ok as u32;
        votes += vote as u32;
        successful_votes += (ok && vote) as u32;
    }
    let total = block.transactions.len() as u32;

    BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        slot: block.slot,
        parent_slot: block.parent_slot,
        block_height: block.block_height.as_ref().map(|h| h.block_height),
        blockhash: base58_to_bytes(&block.blockhash),
        previous_blockhash: base58_to_bytes(&block.previous_blockhash),
        block_time: block.block_time.as_ref().map(|t| t.timestamp),
        num_transactions: total,
        num_rewards: block.rewards.len() as u32,

        num_successful_transactions: successful,
        num_failed_transactions: total - successful,
        num_vote_transactions: votes,
        num_successful_vote_transactions: successful_votes,
        num_failed_vote_transactions: votes - successful_votes,
    }
}
