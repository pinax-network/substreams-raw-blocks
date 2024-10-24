use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{blocks::insert_blocks, transactions::insert_transactions};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    insert_blocks(&mut tables, &clock, &block);

    insert_transactions(&mut tables, &clock, &block);

    Ok(tables)
}
