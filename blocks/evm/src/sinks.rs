use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2::Block;

use crate::blocks::insert_blocks;

#[substreams::handlers::map]
pub fn ch_out(params: String, clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::blocks
    insert_blocks(&params, &mut tables, &clock, &block);

    Ok(tables)
}

// // TO-DO: Implement the `graph_out` function using EntityChanges
// #[substreams::handlers::map]
// pub fn graph_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
//     let mut tables: DatabaseChanges = DatabaseChanges::default();
//     insert_blocks(&mut tables, &clock, &block);
//     // TO-DO: Convert DatabaseChanges to EntityChanges
//     Ok(tables)
// }
