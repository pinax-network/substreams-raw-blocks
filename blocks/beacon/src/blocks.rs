use common::utils::bytes_to_hex;

use crate::{
    pb::{pinax::beacon::v1::Block, sf::beacon::r#type::v1::Block as BeaconBlock},
    structs::BlockTimestamp,
};

pub fn collect_blocks(block: &BeaconBlock, spec: &str, timestamp: &BlockTimestamp) -> Vec<Block> {
    let mut blocks = Vec::new();

    blocks.push(Block {
        time: Some(timestamp.time),
        number: timestamp.number,
        date: timestamp.date.clone(),
        hash: timestamp.hash.clone(),
        version: block.version,
        spec: spec.to_string(),
        slot: block.slot,
        parent_slot: block.parent_slot,
        root: bytes_to_hex(&block.root),
        parent_root: bytes_to_hex(&block.parent_root),
        state_root: bytes_to_hex(&block.state_root),
        proposer_index: block.proposer_index,
        body_root: bytes_to_hex(&block.body_root),
        signature: bytes_to_hex(&block.signature),
    });

    blocks
}
