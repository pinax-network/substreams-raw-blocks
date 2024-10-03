use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {}
