use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::account_activity::collect_account_activity;
use crate::blocks::collect_block;
use crate::instructions::{collect_account_lookups, collect_instructions};
use crate::messages::collect_message;
use crate::pb::pinax::solana::v2::Events;
use crate::rewards::{collect_block_rewards, collect_transaction_rewards};
use crate::token_balances::collect_token_balances;
use crate::transactions::{collect_transaction, to_vote_transaction};
use crate::utils::{base58_to_bytes, is_vote_transaction};

// Input is `common:blocks_without_votes`, so no vote transactions reach this module.
#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    Ok(collect_events(&clock, &block))
}

// Full block: vote transactions land in `vote_transactions`, everything else as in `map_events`.
#[substreams::handlers::map]
pub fn map_events_with_votes(clock: Clock, block: Block) -> Result<Events, Error> {
    Ok(collect_events(&clock, &block))
}

pub fn block_identity(clock: &Clock, block: &Block) -> BlockIdentity {
    BlockIdentity::new(
        block.slot,
        base58_to_bytes(&block.blockhash),
        block.parent_slot,
        base58_to_bytes(&block.previous_blockhash),
        &clock.timestamp,
    )
}

fn collect_events(clock: &Clock, block: &Block) -> Events {
    let id = block_identity(clock, block);
    let mut events = Events::default();
    events.blocks.push(collect_block(block, &id));
    events.rewards.extend(collect_block_rewards(block, &id));

    for (index, tx) in block.transactions.iter().enumerate() {
        let Some(message) = tx.transaction.as_ref().and_then(|t| t.message.as_ref()) else {
            continue;
        };
        let index = index as u32;
        let row = collect_transaction(tx, index, &id);

        if is_vote_transaction(message) {
            events.vote_transactions.push(to_vote_transaction(row));
            continue;
        }

        let signature = row.signature.clone();
        events.transactions.push(row);
        events.messages.push(collect_message(tx, index, &signature, &id));
        events.instructions.extend(collect_instructions(tx, index, &signature, &id));
        events.account_lookups.extend(collect_account_lookups(tx, index, &signature, &id));
        events.token_balances.extend(collect_token_balances(tx, index, &signature, &id));
        events.rewards.extend(collect_transaction_rewards(tx, index, events.rewards.len() as u32, &id));
        events.account_activity.extend(collect_account_activity(tx, index, &signature, &id));
    }

    events
}
