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
