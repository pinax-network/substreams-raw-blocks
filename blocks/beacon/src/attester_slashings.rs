use common::{
    blocks::insert_timestamp,
    utils::{bytes_to_hex, u64_array_to_string},
};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::attester_slashing_keys, pb::sf::beacon::r#type::v1::AttesterSlashing};

pub fn insert_attester_slashings(tables: &mut DatabaseChanges, clock: &Clock, attester_slashings: &[AttesterSlashing]) {
    for (index, attester_slashing) in attester_slashings.iter().enumerate() {
        let attestation_1 = &attester_slashing.attestation_1.as_ref().unwrap();
        let attestation_data_1 = &attestation_1.data.as_ref().unwrap();
        let attestation_2 = &attester_slashing.attestation_2.as_ref().unwrap();
        let attestation_data_2 = &attestation_2.data.as_ref().unwrap();

        // Attestation 1
        let attestation_1_attesting_indices = u64_array_to_string(&attestation_1.attesting_indices);
        let attestation_1_slot = attestation_data_1.slot;
        let attestation_1_committee_index = attestation_data_1.committee_index;
        let attestation_1_beacon_block_root = bytes_to_hex(&attestation_data_1.beacon_block_root);
        let attestation_1_source = attestation_data_1.source.as_ref().unwrap();
        let attestation_1_target = attestation_data_1.target.as_ref().unwrap();
        let attestation_1_source_epoch = attestation_1_source.epoch;
        let attestation_1_source_root = bytes_to_hex(&attestation_1_source.root);
        let attestation_1_target_epoch = attestation_1_target.epoch;
        let attestation_1_target_root = bytes_to_hex(&attestation_1_target.root);
        let attestation_1_signature = bytes_to_hex(&attestation_1.signature);

        // Attestation 2
        let attestation_2_attesting_indices = u64_array_to_string(&attestation_2.attesting_indices);
        let attestation_2_slot = attestation_data_2.slot;
        let attestation_2_committee_index = attestation_data_2.committee_index;
        let attestation_2_beacon_block_root = bytes_to_hex(&attestation_data_2.beacon_block_root);
        let attestation_2_source = attestation_data_2.source.as_ref().unwrap();
        let attestation_2_target = attestation_data_2.target.as_ref().unwrap();
        let attestation_2_source_epoch = attestation_2_source.epoch;
        let attestation_2_source_root = bytes_to_hex(&attestation_2_source.root);
        let attestation_2_target_epoch = attestation_2_target.epoch;
        let attestation_2_target_root = bytes_to_hex(&attestation_2_target.root);
        let attestation_2_signature = bytes_to_hex(&attestation_2.signature);

        let keys = attester_slashing_keys(&clock.id, index as u64);

        let row = tables
            .push_change_composite("attester_slashings", keys, 0, table_change::Operation::Create)
            .change("attestation_1_attesting_indices", ("", attestation_1_attesting_indices.as_str()))
            .change("attestation_1_slot", ("", attestation_1_slot.to_string().as_str()))
            .change("attestation_1_committee_index", ("", attestation_1_committee_index.to_string().as_str()))
            .change("attestation_1_beacon_block_root", ("", attestation_1_beacon_block_root.as_str()))
            .change("attestation_1_source_epoch", ("", attestation_1_source_epoch.to_string().as_str()))
            .change("attestation_1_source_root", ("", attestation_1_source_root.as_str()))
            .change("attestation_1_target_epoch", ("", attestation_1_target_epoch.to_string().as_str()))
            .change("attestation_1_target_root", ("", attestation_1_target_root.as_str()))
            .change("attestation_1_signature", ("", attestation_1_signature.as_str()))
            .change("attestation_2_attesting_indices", ("", attestation_2_attesting_indices.as_str()))
            .change("attestation_2_slot", ("", attestation_2_slot.to_string().as_str()))
            .change("attestation_2_committee_index", ("", attestation_2_committee_index.to_string().as_str()))
            .change("attestation_2_beacon_block_root", ("", attestation_2_beacon_block_root.as_str()))
            .change("attestation_2_source_epoch", ("", attestation_2_source_epoch.to_string().as_str()))
            .change("attestation_2_source_root", ("", attestation_2_source_root.as_str()))
            .change("attestation_2_target_epoch", ("", attestation_2_target_epoch.to_string().as_str()))
            .change("attestation_2_target_root", ("", attestation_2_target_root.as_str()))
            .change("attestation_2_signature", ("", attestation_2_signature.as_str()));

        insert_timestamp(row, clock, true, false);
    }
}
