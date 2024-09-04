use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::subgraph::blocks::insert_blocks_subgraph;
use crate::clickhouse::blocks::insert_blocks_clickhouse;

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // TABLE::blocks
    insert_blocks_subgraph(&params, &mut tables, &clock, &block);

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn ch_out(params: String, clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::Block
    insert_blocks_clickhouse(&params, &mut tables, &clock, &block);

    Ok(tables)
}
