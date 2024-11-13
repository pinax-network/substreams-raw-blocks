use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::blocks::{collect_block, get_block_info};
use crate::pb::solana::rawblocks::Events;
use crate::rewards::collect_rewards;
use crate::utils::get_timestamp_without_number;

#[substreams::handlers::map]
pub fn ch_out_without_votes(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = get_timestamp_without_number(&clock);
    let block_info = get_block_info(&block);

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp, &block_info).unwrap()],
        rewards: collect_rewards(&block, &timestamp, &block_info),
        transactions: vec![],
        instruction_calls: vec![],
        account_activity: vec![],
        vote_transactions: vec![],
        vote_instruction_calls: vec![],
        vote_account_activity: vec![],
    })
}

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = get_timestamp_without_number(&clock);
    let block_info = get_block_info(&block);

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp, &block_info).unwrap()],
        rewards: collect_rewards(&block, &timestamp, &block_info),
        transactions: vec![],
        instruction_calls: vec![],
        account_activity: vec![],
        vote_transactions: vec![],
        vote_instruction_calls: vec![],
        vote_account_activity: vec![],
    })
}
