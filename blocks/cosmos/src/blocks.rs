use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::Block;

use crate::{pb::pinax::cosmos::v1::Block as RawBlock, size::get_size};

pub fn collect_block(block: &Block, timestamp: &BlockTimestamp) -> RawBlock {
    let header = block.header.as_ref().unwrap();
    let consensus = header.version.as_ref().unwrap();

    let size = get_size(block);

    RawBlock {
        time: Some(timestamp.time),
        number: timestamp.number,
        date: timestamp.date.clone(),
        hash: Hex::encode(&block.hash),
        version_consensus_block: consensus.block,
        version_consensus_app: consensus.app,
        chain_id: header.chain_id.clone(),
        last_block_id: match &header.last_block_id {
            Some(block_id) => Hex::encode(&block_id.hash),
            None => String::new(),
        },
        last_commit_hash: Hex::encode(&header.last_commit_hash),
        data_hash: Hex::encode(&header.data_hash),
        validators_hash: Hex::encode(&header.validators_hash),
        next_validators_hash: Hex::encode(&header.next_validators_hash),
        consensus_hash: Hex::encode(&header.consensus_hash),
        app_hash: Hex::encode(&header.app_hash),
        last_results_hash: Hex::encode(&header.last_results_hash),
        evidence_hash: Hex::encode(&header.evidence_hash),
        proposer_address: Hex::encode(&header.proposer_address),
        total_transactions: size.total_transactions,
        successful_transactions: size.successful_transactions,
        failed_transactions: size.failed_transactions,
    }
}
