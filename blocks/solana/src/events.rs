use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

use crate::account_activity::collect_account_activities;
use crate::blocks::{collect_block, get_block_info};
use crate::instruction_calls::collect_instruction_calls;
use crate::pb::solana::Events;
use crate::rewards::collect_rewards;
use crate::transactions::collect_transactions;
use crate::utils::get_timestamp_without_number;

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = get_timestamp_without_number(&clock);
    let block_info = get_block_info(&block);

    let transactions_with_index: Vec<(usize, &ConfirmedTransaction)> = block.transactions.iter().enumerate().collect();

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp, &block_info).unwrap()],
        rewards: collect_rewards(&block, &timestamp, &block_info),
        transactions: collect_transactions(&transactions_with_index, &block_info, &timestamp),
        instruction_calls: collect_instruction_calls(&block, &timestamp, &block_info),
        account_activity: collect_account_activities(&block_info, &timestamp, &transactions_with_index),
        vote_transactions: vec![],
        vote_instruction_calls: vec![],
        vote_account_activity: vec![],
    })
}

#[substreams::handlers::map]
pub fn map_events_with_votes(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = get_timestamp_without_number(&clock);
    let block_info = get_block_info(&block);

    let (non_vote_trx, vote_trx): (Vec<(usize, &ConfirmedTransaction)>, Vec<(usize, &ConfirmedTransaction)>) = block.transactions.iter().enumerate().partition(|(_index, trx)| {
        !trx.transaction
            .as_ref()
            .and_then(|t| t.message.as_ref())
            .map_or(false, |message| message.account_keys.iter().any(|key| key == &VOTE_INSTRUCTION))
    });

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp, &block_info).unwrap()],
        rewards: collect_rewards(&block, &timestamp, &block_info),
        transactions: collect_transactions(&non_vote_trx, &block_info, &timestamp),
        instruction_calls: collect_instruction_calls(&block, &timestamp, &block_info),
        account_activity: collect_account_activities(&block_info, &timestamp, &non_vote_trx),
        vote_transactions: collect_transactions(&vote_trx, &block_info, &timestamp),
        vote_instruction_calls: collect_instruction_calls(&block, &timestamp, &block_info),
        vote_account_activity: collect_account_activities(&block_info, &timestamp, &vote_trx),
    })
}