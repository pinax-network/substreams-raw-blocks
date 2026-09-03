use common::structs::BlockIdentity;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, Reward as RewardSource};

use crate::pb::pinax::solana::v2::Reward;
use crate::utils::{non_empty, reward_type_text};

pub fn collect_block_rewards(block: &Block, id: &BlockIdentity) -> Vec<Reward> {
    block.rewards.iter().enumerate().map(|(i, r)| reward(r, i as u32, "block", None, id)).collect()
}

pub fn collect_transaction_rewards(tx: &ConfirmedTransaction, index: u32, first_reward_index: u32, id: &BlockIdentity) -> Vec<Reward> {
    let Some(meta) = tx.meta.as_ref() else {
        return vec![];
    };
    meta.rewards
        .iter()
        .enumerate()
        .map(|(i, r)| reward(r, first_reward_index + i as u32, "transaction", Some(index), id))
        .collect()
}

fn reward(r: &RewardSource, reward_index: u32, source: &str, transaction_index: Option<u32>, id: &BlockIdentity) -> Reward {
    Reward {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        slot: id.block_num,
        reward_index,
        pubkey: r.pubkey.clone(),
        lamports: r.lamports,
        post_balance: r.post_balance,
        reward_type: reward_type_text(r.reward_type),
        commission: non_empty(&r.commission),
        source: source.to_string(),
        transaction_index,

        pre_balance: (r.post_balance as i128 - r.lamports as i128).max(0) as u64,
    }
}
