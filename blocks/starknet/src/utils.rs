use common::utils::bytes_to_hex;

use crate::pb::sf::starknet::r#type::v1::Block;

pub struct BlockHashes {
    pub new_root: String,
    pub parent_hash: String,
    pub sequencer_address: String,
}

pub fn build_block_hashes(block: &Block) -> BlockHashes {
    BlockHashes {
        new_root: bytes_to_hex(&block.new_root),
        parent_hash: bytes_to_hex(&block.parent_hash),
        sequencer_address: bytes_to_hex(&block.sequencer_address),
    }
}

pub fn data_availability_mode_to_string(value: i32) -> String {
    match value {
        1 => "L1".to_string(),
        2 => "L2".to_string(),
        _ => "Unknown".to_string(),
    }
}
