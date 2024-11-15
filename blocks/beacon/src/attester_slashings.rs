use common::utils::{bytes_to_hex, number_array_to_string};

use crate::{
    pb::{beacon::AttesterSlashing as RawAttesterSlashing, sf::beacon::r#type::v1::AttesterSlashing},
    structs::BlockTimestamp,
};

pub fn collect_attester_slashings(attester_slashings: &Vec<AttesterSlashing>, timestamp: &BlockTimestamp) -> Vec<RawAttesterSlashing> {
    let mut vec = Vec::<RawAttesterSlashing>::new();

    for (index, attester_slashing) in attester_slashings.iter().enumerate() {
        vec.push(RawAttesterSlashing {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u64,
            attestation_1_attesting_indices: number_array_to_string(&attester_slashing.attestation_1.as_ref().unwrap().attesting_indices),
            attestation_1_slot: attester_slashing.attestation_1.as_ref().unwrap().data.as_ref().unwrap().slot,
            attestation_1_committee_index: attester_slashing.attestation_1.as_ref().unwrap().data.as_ref().unwrap().committee_index,
            attestation_1_beacon_block_root: bytes_to_hex(&attester_slashing.attestation_1.as_ref().unwrap().data.as_ref().unwrap().beacon_block_root),
            attestation_1_source_epoch: attester_slashing.attestation_1.as_ref().unwrap().data.as_ref().unwrap().source.as_ref().unwrap().epoch,
            attestation_1_source_root: bytes_to_hex(&attester_slashing.attestation_1.as_ref().unwrap().data.as_ref().unwrap().source.as_ref().unwrap().root),
            attestation_1_target_epoch: attester_slashing.attestation_1.as_ref().unwrap().data.as_ref().unwrap().target.as_ref().unwrap().epoch,
            attestation_1_target_root: bytes_to_hex(&attester_slashing.attestation_1.as_ref().unwrap().data.as_ref().unwrap().target.as_ref().unwrap().root),
            attestation_1_signature: bytes_to_hex(&attester_slashing.attestation_1.as_ref().unwrap().signature),
            attestation_2_attesting_indices: number_array_to_string(&attester_slashing.attestation_2.as_ref().unwrap().attesting_indices),
            attestation_2_slot: attester_slashing.attestation_2.as_ref().unwrap().data.as_ref().unwrap().slot,
            attestation_2_committee_index: attester_slashing.attestation_2.as_ref().unwrap().data.as_ref().unwrap().committee_index,
            attestation_2_beacon_block_root: bytes_to_hex(&attester_slashing.attestation_2.as_ref().unwrap().data.as_ref().unwrap().beacon_block_root),
            attestation_2_source_epoch: attester_slashing.attestation_2.as_ref().unwrap().data.as_ref().unwrap().source.as_ref().unwrap().epoch,
            attestation_2_source_root: bytes_to_hex(&attester_slashing.attestation_2.as_ref().unwrap().data.as_ref().unwrap().source.as_ref().unwrap().root),
            attestation_2_target_epoch: attester_slashing.attestation_2.as_ref().unwrap().data.as_ref().unwrap().target.as_ref().unwrap().epoch,
            attestation_2_target_root: bytes_to_hex(&attester_slashing.attestation_2.as_ref().unwrap().data.as_ref().unwrap().target.as_ref().unwrap().root),
            attestation_2_signature: bytes_to_hex(&attester_slashing.attestation_2.as_ref().unwrap().signature),
        });
    }
    vec
}
