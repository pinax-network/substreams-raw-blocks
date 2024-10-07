use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::{blocks::insert_blockinfo, keys::reward_keys, utils::insert_timestamp_without_number};

pub fn insert_rewards(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    block.rewards.iter().for_each(|reward| {
        let reward_type = reward_type(reward.reward_type);
        let pre_balance = reward.post_balance as i64 - reward.lamports;

        let keys = reward_keys(&block.blockhash, &reward.pubkey, &reward_type);
        let row = tables
            .push_change_composite("rewards", keys, 0, table_change::Operation::Create)
            .change("pubkey", ("", reward.pubkey.as_str()))
            .change("lamports", ("", reward.lamports.to_string().as_str()))
            .change("pre_balance", ("", pre_balance.to_string().as_str()))
            .change("post_balance", ("", reward.post_balance.to_string().as_str()))
            .change("reward_type", ("", reward_type.as_str()))
            .change("commission", ("", reward.commission.as_str()));

        insert_blockinfo(row, block, true);
        insert_timestamp_without_number(row, clock, false, false);
    });
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
