use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_antelope::pb::Block;

use crate::blocks::insert_blocks;
use crate::transactions::insert_transactions;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::blocks
    insert_blocks(&mut tables, &clock, &block);

    // TABLE::transactions
    insert_transactions(&mut tables, &clock, &block);

    Ok(tables)
}
