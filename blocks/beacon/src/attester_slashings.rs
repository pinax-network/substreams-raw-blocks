use common::utils::bytes_to_hex;

use crate::{
    pb::{
        pinax::beacon::v1::AttesterSlashing as RawAttesterSlashing,
        sf::beacon::r#type::v1::{AttesterSlashing, IndexedAttestation},
    },
    structs::BlockTimestamp,
};

pub fn collect_attester_slashings(attester_slashings: &Vec<AttesterSlashing>, timestamp: &BlockTimestamp) -> Vec<RawAttesterSlashing> {
    let mut vec = Vec::<RawAttesterSlashing>::new();

    for (index, attester_slashing) in attester_slashings.iter().enumerate() {
        vec.push(parse_attester_slashings(index as u64, &attester_slashing.attestation_1.as_ref().unwrap(), timestamp));
        vec.push(parse_attester_slashings(index as u64, &attester_slashing.attestation_2.as_ref().unwrap(), timestamp));
    }
    vec
}

pub fn parse_attester_slashings(index: u64, indexed_attestation: &IndexedAttestation, timestamp: &BlockTimestamp) -> RawAttesterSlashing {
    let data = indexed_attestation.data.as_ref().unwrap();
    let source = data.source.as_ref().unwrap();
    let target = data.target.as_ref().unwrap();

    RawAttesterSlashing {
        block_time: Some(timestamp.time),
        block_number: timestamp.number,
        block_date: timestamp.date.clone(),
        block_hash: timestamp.hash.clone(),
        index,
        attesting_indices: indexed_attestation.attesting_indices.clone(),
        slot: data.slot,
        committee_index: data.committee_index,
        beacon_block_root: bytes_to_hex(&data.beacon_block_root),
        source_epoch: source.epoch,
        source_root: bytes_to_hex(&source.root),
        target_epoch: target.epoch,
        target_root: bytes_to_hex(&target.root),
        signature: bytes_to_hex(&indexed_attestation.signature),
    }
}
