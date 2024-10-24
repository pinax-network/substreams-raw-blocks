use substreams::pb::substreams::Clock;
use substreams_cosmos::Block;
use substreams_database_change::tables::Tables;

pub fn insert_validator_updates(tables: &mut Tables, clock: &Clock, block: &Block) {}
