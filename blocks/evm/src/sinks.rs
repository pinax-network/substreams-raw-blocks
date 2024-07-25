use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2::Block;

// use crate::blocks::insert_blocks;
use crate::logs::insert_logs;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();
    // insert_blocks(&mut tables, &clock, &block);
    // insert_logs(&mut tables, &clock, &block);
    Ok(tables)
}
