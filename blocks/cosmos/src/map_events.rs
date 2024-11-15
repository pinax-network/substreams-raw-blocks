use common::utils::build_timestamp;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;

use crate::{
    blocks::collect_blocks,
    consensus_param_updates::collect_consensus_params,
    events::{collect_block_events, collect_tx_events},
    misbehaviors::collect_misbehaviors,
    pb::cosmos::Events,
    transaction_messages::collect_transaction_messages,
    transactions::collect_transactions,
    validator_updates::collect_validator_updates,
};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);

    Ok(Events {
        blocks: collect_blocks(&block, &timestamp),
        transactions: collect_transactions(&block, &timestamp),
        transaction_events: collect_tx_events(&block, &timestamp),
        block_events: collect_block_events(&block, &timestamp),
        misbehaviors: collect_misbehaviors(&block, &timestamp),
        validator_updates: collect_validator_updates(&block, &timestamp),
        consensus_param_updates: collect_consensus_params(&block, &timestamp),
        transaction_messages: collect_transaction_messages(&block, &timestamp),
    })
}
