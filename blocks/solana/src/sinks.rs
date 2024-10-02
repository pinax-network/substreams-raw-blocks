use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::blocks::insert_blocks;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::blocks
    insert_blocks(&mut tables, &clock, &block);

    Ok(tables)
}
