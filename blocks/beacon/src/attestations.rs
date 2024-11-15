use common::utils::bytes_to_hex;

use crate::{
    pb::{beacon::Attestation as RawAttestation, sf::beacon::r#type::v1::Attestation},
    structs::BlockTimestamp,
};

pub fn collect_attestations(attestations: &Vec<Attestation>, timestamp: &BlockTimestamp) -> Vec<RawAttestation> {
    let mut vec = Vec::<RawAttestation>::new();

    for (index, attestation) in attestations.iter().enumerate() {
        vec.push(RawAttestation {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u64,
            aggregation_bits: bytes_to_hex(&attestation.aggregation_bits),
            slot: attestation.data.as_ref().unwrap().slot,
            committee_index: attestation.data.as_ref().unwrap().committee_index,
            beacon_block_root: bytes_to_hex(&attestation.data.as_ref().unwrap().beacon_block_root),
            source_epoch: attestation.data.as_ref().unwrap().source.as_ref().unwrap().epoch,
            source_root: bytes_to_hex(&attestation.data.as_ref().unwrap().source.as_ref().unwrap().root),
            target_epoch: attestation.data.as_ref().unwrap().target.as_ref().unwrap().epoch,
            target_root: bytes_to_hex(&attestation.data.as_ref().unwrap().target.as_ref().unwrap().root),
            signature: bytes_to_hex(&attestation.signature),
        });
    }

    vec
}
