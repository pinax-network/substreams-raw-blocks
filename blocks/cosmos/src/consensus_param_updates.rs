use common::{structs::BlockTimestamp, utils::bytes_to_hex};
use substreams_cosmos::Block;

use crate::pb::cosmos::ConsensusParamUpdate as RawConsensusParamUpdate;

pub fn collect_consensus_params(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawConsensusParamUpdate> {
    let mut vec: Vec<RawConsensusParamUpdate> = vec![];

    if let Some(consensus_params) = &block.consensus_param_updates {
        let mut json = serde_json::json!({});

        if let Some(block_params) = &consensus_params.block {
            json["block"] = serde_json::json!({
                "max_bytes": block_params.max_bytes,
                "max_gas": block_params.max_gas,
            });
        }

        if let Some(evidence_params) = &consensus_params.evidence {
            json["evidence"] = serde_json::json!({
                "max_age_num_blocks": evidence_params.max_age_num_blocks,
                "max_age_duration": evidence_params.max_age_duration.as_ref().map(|d| d.to_string()),
                "max_bytes": evidence_params.max_bytes
            });
        }

        if let Some(validator_params) = &consensus_params.validator {
            json["validator"] = serde_json::json!({
                "pub_key_types": validator_params.pub_key_types
            });
        }

        if let Some(version_params) = &consensus_params.version {
            json["version"] = serde_json::json!({
                "app_version": version_params.app
            });
        }

        vec.push(RawConsensusParamUpdate {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: bytes_to_hex(&block.hash),
            json: json.to_string(),
        });
    }

    vec
}
