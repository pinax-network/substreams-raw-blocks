use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::{
    pb::pinax::solana::v1::Reward,
    structs::{BlockInfo, BlockTimestamp},
};

pub fn collect_rewards(block: &Block, timestamp: &BlockTimestamp, block_info: &BlockInfo) -> Vec<Reward> {
    let mut rewards = Vec::new();
    for reward in block.rewards.iter() {
        let reward_type = reward_type(reward.reward_type);
        let pre_balance = reward.post_balance as i128 - reward.lamports as i128;

        rewards.push(Reward {
            block_slot: block.slot,
            block_height: block_info.height,
            block_previous_block_hash: block_info.previous_block_hash.clone(),
            block_parent_slot: block_info.parent_slot,
            block_time: timestamp.time.to_string(),
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            pubkey: reward.pubkey.clone(),
            lamports: reward.lamports,
            pre_balance: pre_balance as u64,
            post_balance: reward.post_balance,
            reward_type,
            commission: reward.commission.clone(),
        });
    }

    rewards
}

pub fn reward_type(reward_type: i32) -> String {
    match reward_type {
        0 => "Unspecified".to_string(),
        1 => "Fee".to_string(),
        2 => "Rent".to_string(),
        3 => "Staking".to_string(),
        4 => "Voting".to_string(),
        _ => "Unknown".to_string(),
    }
}
