use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::blocks::collect_block;
use crate::pb::solana::rawblocks::Events;

#[substreams::handlers::map]
pub fn ch_out_without_votes(clock: Clock, block: Block) -> Result<Events, Error> {
    Ok(Events {
        blocks: vec![collect_block(&block, &clock).unwrap()],
        rewards: vec![],
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
    Ok(Events {
        blocks: vec![collect_block(&block, &clock).unwrap()],
        rewards: vec![],
        transactions: vec![],
        instruction_calls: vec![],
        account_activity: vec![],
        vote_transactions: vec![],
        vote_instruction_calls: vec![],
        vote_account_activity: vec![],
    })
}
