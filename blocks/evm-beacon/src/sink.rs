use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{blobs::insert_blobs, blocks::insert_blocks, pb::sf::beacon::r#type::v1::Block as BeaconBlock};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: BeaconBlock) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();

    insert_blocks(&mut tables, &block, &clock);
    insert_blobs(&mut tables, &block, &clock);

    Ok(tables)
}
