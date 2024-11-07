use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::pb::sf::beacon::r#type::v1::Block as BeaconBlock;

pub fn insert_blobs(tables: &mut DatabaseChanges, block: &BeaconBlock, clock: &Clock) {}
