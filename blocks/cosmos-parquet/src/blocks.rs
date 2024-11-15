use common::{structs::BlockTimestamp, utils::bytes_to_hex};
use substreams_cosmos::Block;

use crate::{pb::cosmos::rawblocks::Block as RawBlock, size::get_size};

pub fn collect_blocks(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawBlock> {
    let mut vec: Vec<RawBlock> = vec![];

    let header = block.header.as_ref().unwrap();
    let consensus = header.version.as_ref().unwrap();

    let (total_transactions, successful_transactions, failed_transactions) = get_size(block);

    vec.push(RawBlock {
        time: Some(timestamp.time),
        number: timestamp.number,
        date: timestamp.date.clone(),
        hash: bytes_to_hex(&block.hash),
        version_consensus_block: consensus.block,
        version_consensus_app: consensus.app,
        chain_id: header.chain_id.clone(),
        last_block_id: match &header.last_block_id {
            Some(block_id) => bytes_to_hex(&block_id.hash),
            None => String::new(),
        },
        last_commit_hash: bytes_to_hex(&header.last_commit_hash),
        data_hash: bytes_to_hex(&header.data_hash),
        validators_hash: bytes_to_hex(&header.validators_hash),
        next_validators_hash: bytes_to_hex(&header.next_validators_hash),
        consensus_hash: bytes_to_hex(&header.consensus_hash),
        app_hash: bytes_to_hex(&header.app_hash),
        last_results_hash: bytes_to_hex(&header.last_results_hash),
        evidence_hash: bytes_to_hex(&header.evidence_hash),
        proposer_address: bytes_to_hex(&header.proposer_address),
        total_transactions,
        successful_transactions,
        failed_transactions,
    });

    vec
}
