use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{blocks::insert_blocks, events::insert_block_events, transactions::insert_transactions};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::blocks
    insert_blocks(&mut tables, &clock, &block);

    // TABLE::block_events
    insert_block_events(&mut tables, &clock, &block);

    // TABLE::transactions + TABLE::tx_events
    insert_transactions(&mut tables, &clock, &block);

    Ok(tables)
}
