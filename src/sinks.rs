use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::blocks::insert_blocks;
use crate::blocks_clickhouse::insert_blocks_clickhouse;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::blocks
    insert_blocks_clickhouse(&mut tables, &clock, &block);

    Ok(tables)
}

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // TABLE::blocks
    insert_blocks(&mut tables, &clock, &block);

    Ok(tables.to_entity_changes())
}
