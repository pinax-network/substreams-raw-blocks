use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_cosmos::Block;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn insert_consensus_params(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
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

        let json_str = json.to_string();

        let row = tables
            .push_change("consensus_param_updates", &clock.number.to_string(), 0, table_change::Operation::Create)
            .change("json", ("", json_str.as_str()));

        insert_timestamp(row, clock, false, true);
    }
}
