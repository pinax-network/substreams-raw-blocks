use substreams::pb::substreams::Clock;
use substreams_solana::{
    b58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{
    account_activity::collect_tx_account_activities,
    blocks::{collect_block, get_block_info},
    instruction_calls::collect_tx_instruction_calls,
    pb::solana::Events,
    rewards::collect_rewards,
    transactions::collect_transaction,
    utils::get_timestamp_without_number,
};

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

pub fn collect_events_without_votes(block: &Block, clock: &Clock) -> Events {
    let block_info = get_block_info(block);
    let timestamp = get_timestamp_without_number(&clock);

    let mut events = Events {
        blocks: vec![collect_block(block, &timestamp, &block_info)],
        rewards: collect_rewards(block, &timestamp, &block_info),
        transactions: vec![],
        instruction_calls: vec![],
        account_activity: vec![],
        vote_transactions: vec![],
        vote_instruction_calls: vec![],
        vote_account_activity: vec![],
    };

    for (index, trx) in block.transactions.iter().enumerate() {
        events.transactions.push(collect_transaction(trx, index, &block_info, &timestamp));
        events.instruction_calls.extend(collect_tx_instruction_calls(trx, index, &block_info, &timestamp));
        events.account_activity.extend(collect_tx_account_activities(trx, index, &block_info, &timestamp));
    }

    events
}

pub fn collect_events_with_votes(block: &Block, clock: &Clock) -> Events {
    let block_info = get_block_info(block);
    let timestamp = get_timestamp_without_number(&clock);

    let mut events = Events {
        blocks: vec![collect_block(block, &timestamp, &block_info)],
        rewards: collect_rewards(block, &timestamp, &block_info),
        transactions: vec![],
        instruction_calls: vec![],
        account_activity: vec![],
        vote_transactions: vec![],
        vote_instruction_calls: vec![],
        vote_account_activity: vec![],
    };

    let (non_vote_trx, vote_trx): (Vec<(usize, &ConfirmedTransaction)>, Vec<(usize, &ConfirmedTransaction)>) = block.transactions.iter().enumerate().partition(|(_index, trx)| {
        !trx.transaction
            .as_ref()
            .and_then(|t| t.message.as_ref())
            .map_or(false, |message| message.account_keys.iter().any(|key| key == &VOTE_INSTRUCTION))
    });

    for (index, trx) in non_vote_trx {
        events.transactions.push(collect_transaction(trx, index, &block_info, &timestamp));
        events.instruction_calls.extend(collect_tx_instruction_calls(trx, index, &block_info, &timestamp));
        events.account_activity.extend(collect_tx_account_activities(trx, index, &block_info, &timestamp));
    }

    for (index, trx) in vote_trx {
        events.vote_transactions.push(collect_transaction(trx, index, &block_info, &timestamp));
        events.vote_instruction_calls.extend(collect_tx_instruction_calls(trx, index, &block_info, &timestamp));
        events.vote_account_activity.extend(collect_tx_account_activities(trx, index, &block_info, &timestamp));
    }

    events
}
