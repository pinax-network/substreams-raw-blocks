use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{
    blocks::insert_blocks, consensus_param_updates::insert_consensus_params, events::insert_block_events, misbehaviors::insert_misbehaviors, transactions::insert_transactions,
    validator_updates::insert_validator_updates,
};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::blocks
    insert_blocks(&mut tables, &clock, &block);

    // TABLE::block_events
    insert_block_events(&mut tables, &clock, &block);

    // TABLE::transactions + TABLE::tx_events + TABLE::transaction_messages
    insert_transactions(&mut tables, &clock, &block);

    // TABLE::misbehaviors
    insert_misbehaviors(&mut tables, &clock, &block);

    // TABLE::validator_updates
    insert_validator_updates(&mut tables, &clock, &block);

    // TABLE::consensus_param_updates
    insert_consensus_params(&mut tables, &clock, &block);

    Ok(tables)
}
