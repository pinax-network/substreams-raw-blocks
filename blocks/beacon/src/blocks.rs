use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::pb::sf::beacon::r#type::v1::Block as BeaconBlock;

pub fn insert_blocks(tables: &mut DatabaseChanges, block: &BeaconBlock, clock: &Clock) {
    let version = block.version;
    let spec = spec_to_string(block.spec);
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let root = bytes_to_hex(&block.root);
    let parent_root = bytes_to_hex(&block.parent_root);
    let state_root = bytes_to_hex(&block.state_root);
    let proposer_index = block.proposer_index;
    let body_root = bytes_to_hex(&block.body_root);
    let signature = bytes_to_hex(&block.signature);

    let row = tables
        .push_change("blocks", &clock.id, 0, table_change::Operation::Create)
        .change("version", ("", version.to_string().as_str()))
        .change("spec", ("", spec.as_str()))
        .change("slot", ("", slot.to_string().as_str()))
        .change("parent_slot", ("", parent_slot.to_string().as_str()))
        .change("root", ("", root.as_str()))
        .change("parent_root", ("", parent_root.as_str()))
        .change("state_root", ("", state_root.as_str()))
        .change("proposer_index", ("", proposer_index.to_string().as_str()))
        .change("body_root", ("", body_root.as_str()))
        .change("signature", ("", signature.as_str()));

    insert_timestamp(row, clock, true, false);
}

fn spec_to_string(spec: i32) -> String {
    match spec {
        0 => "Unspecified".to_string(),
        1 => "Phase0".to_string(),
        2 => "Altair".to_string(),
        3 => "Bellatrix".to_string(),
        4 => "Capella".to_string(),
        5 => "Deneb".to_string(),
        _ => "Unknown".to_string(),
    }
}
