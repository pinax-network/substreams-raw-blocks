use common::utils::build_timestamp;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;

use crate::{
    blocks::collect_blocks,
    events::{collect_block_events, collect_tx_events},
    misbehaviors::collect_misbehaviors,
    pb::cosmos::rawblocks::Events,
    transactions::collect_transactions,
};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);

    Ok(Events {
        blocks: collect_blocks(&block, &timestamp),
        transactions: collect_transactions(&block, &timestamp),
        tx_events: collect_tx_events(&block, &timestamp),
        block_events: collect_block_events(&block, &timestamp),
        misbehaviors: collect_misbehaviors(&block, &timestamp),
        validator_updates: vec![],
        consensus_param_updates: vec![],
        transaction_messages: vec![],
    })

    // TABLE::blocks
    // insert_blocks(&mut tables, &clock, &block);

    // // TABLE::block_events
    // insert_block_events(&mut tables, &clock, &block);

    // // TABLE::transactions + TABLE::tx_events + TABLE::transaction_messages
    // insert_transactions(&mut tables, &clock, &block);

    // // TABLE::misbehaviors
    // insert_misbehaviors(&mut tables, &clock, &block);

    // // TABLE::validator_updates
    // insert_validator_updates(&mut tables, &clock, &block);

    // // TABLE::consensus_param_updates
    // insert_consensus_params(&mut tables, &clock, &block);
}
