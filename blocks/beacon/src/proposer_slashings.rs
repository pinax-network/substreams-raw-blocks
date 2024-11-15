use common::utils::bytes_to_hex;

use crate::{
    pb::{beacon::ProposerSlashing as RawProposerSlashing, sf::beacon::r#type::v1::ProposerSlashing},
    structs::BlockTimestamp,
};

pub fn collect_proposer_slashings(proposer_slashings: &[ProposerSlashing], timestamp: &BlockTimestamp) -> Vec<RawProposerSlashing> {
    let mut vec = Vec::<RawProposerSlashing>::new();

    for (index, proposer_slashing) in proposer_slashings.iter().enumerate() {
        let signed_header_1 = proposer_slashing.signed_header_1.as_ref().unwrap();
        let signed_header_1_message = signed_header_1.message.as_ref().unwrap();
        let signed_header_2 = proposer_slashing.signed_header_2.as_ref().unwrap();
        let signed_header_2_message = signed_header_2.message.as_ref().unwrap();

        vec.push(RawProposerSlashing {
            index: index as u64,
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            signed_header_1_slot: signed_header_1_message.slot,
            signed_header_1_proposer_index: signed_header_1_message.proposer_index,
            signed_header_1_parent_root: bytes_to_hex(&signed_header_1_message.parent_root),
            signed_header_1_state_root: bytes_to_hex(&signed_header_1_message.state_root),
            signed_header_1_body_root: bytes_to_hex(&signed_header_1_message.body_root),
            signed_header_1_signature: bytes_to_hex(&signed_header_1.signature),
            signed_header_2_slot: signed_header_2_message.slot,
            signed_header_2_proposer_index: signed_header_2_message.proposer_index,
            signed_header_2_parent_root: bytes_to_hex(&signed_header_2_message.parent_root),
            signed_header_2_state_root: bytes_to_hex(&signed_header_2_message.state_root),
            signed_header_2_body_root: bytes_to_hex(&signed_header_2_message.body_root),
            signed_header_2_signature: bytes_to_hex(&signed_header_2.signature),
        });
    }

    vec
}
