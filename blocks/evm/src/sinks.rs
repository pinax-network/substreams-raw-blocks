use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;

use crate::blocks::insert_blocks;
use crate::logs::insert_logs;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    insert_blocks(&mut tables, &clock, &block);
    insert_logs(&mut tables, &clock, &block);
    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn map_blocks(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    insert_blocks(&mut tables, &clock, &block);
    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn map_logs(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    insert_logs(&mut tables, &clock, &block);
    Ok(tables.to_entity_changes())
}
