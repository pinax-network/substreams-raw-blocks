use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::attestation_keys, pb::sf::beacon::r#type::v1::Attestation};

pub fn insert_attestations(tables: &mut DatabaseChanges, clock: &Clock, attestations: &Vec<Attestation>) {
    for (index, attestation) in attestations.iter().enumerate() {
        let aggregation_bits = bytes_to_hex(&attestation.aggregation_bits);
        let attestation_data = attestation.data.as_ref().unwrap();
        let slot = attestation_data.slot;
        let committee_index = attestation_data.committee_index;
        let beacon_block_root = bytes_to_hex(&attestation_data.beacon_block_root);
        let source_checkpoint = attestation_data.source.as_ref().unwrap();
        let source_epoch = source_checkpoint.epoch;
        let source_root = bytes_to_hex(&source_checkpoint.root);
        let target_checkpoint = attestation_data.target.as_ref().unwrap();
        let target_epoch = target_checkpoint.epoch;
        let target_root = bytes_to_hex(&target_checkpoint.root);
        let signature = bytes_to_hex(&attestation.signature);

        let keys = attestation_keys(&clock.id, index as u64);

        let row = tables
            .push_change_composite("attestations", keys, 0, table_change::Operation::Create)
            .change("aggregation_bits", ("", aggregation_bits.as_str()))
            .change("slot", ("", slot.to_string().as_str()))
            .change("committee_index", ("", committee_index.to_string().as_str()))
            .change("beacon_block_root", ("", beacon_block_root.as_str()))
            .change("source_epoch", ("", source_epoch.to_string().as_str()))
            .change("source_root", ("", source_root.as_str()))
            .change("target_epoch", ("", target_epoch.to_string().as_str()))
            .change("target_root", ("", target_root.as_str()))
            .change("signature", ("", signature.as_str()));

        insert_timestamp(row, clock, false, true);
    }
}