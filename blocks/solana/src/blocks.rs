use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::size::insert_size;

pub fn insert_blocks(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    let slot = block.slot;
    let hash = block.blockhash.as_str();
    let parent_hash = block.previous_blockhash.as_str();
    let parent_slot = block.parent_slot;

    substreams::log::debug!("insert_blocks: slot: {}", slot);
    substreams::log::debug!("insert_blocks: parent_slot: {}", parent_slot);
    substreams::log::debug!("insert_blocks: parent_hash: {}", parent_hash);
    substreams::log::debug!("insert_blocks: hash: {}", hash);

    let row = tables
        .push_change("blocks", hash, 0, table_change::Operation::Create)
        .change("slot", ("", slot.to_string().as_str()))
        .change("parent_hash", ("", parent_hash))
        .change("parent_slot", ("", parent_slot.to_string().as_str()));

    insert_timestamp(row, clock, true, false);
    insert_size(row, block);
}
