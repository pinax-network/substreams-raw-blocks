use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::Block;

use crate::pb::pinax::cosmos::v1::ConsensusParamUpdate;

pub fn collect_consensus_params(block: &Block, timestamp: &BlockTimestamp) -> Vec<ConsensusParamUpdate> {
    let mut vec: Vec<ConsensusParamUpdate> = vec![];

    if let Some(consensus_params) = &block.consensus_param_updates {
        vec.push(ConsensusParamUpdate {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: Hex::encode(&block.hash),
            block_max_bytes: consensus_params.block.as_ref().map(|b| b.max_bytes),
            block_max_gas: consensus_params.block.as_ref().map(|b| b.max_gas),
            evidence_max_age_num_blocks: consensus_params.evidence.as_ref().map(|e| e.max_age_num_blocks),
            evidence_max_age_duration: consensus_params.evidence.as_ref().and_then(|e| e.max_age_duration.as_ref().map(|d| d.to_string())),
            evidence_max_bytes: consensus_params.evidence.as_ref().map(|e| e.max_bytes),
            validator_pub_key_types: consensus_params.validator.as_ref().map(|v| v.pub_key_types.clone()).unwrap_or_default(),
            app_version: consensus_params.version.as_ref().map(|v| v.app),
        });
    }

    vec
}
