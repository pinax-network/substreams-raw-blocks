use common::utils::bytes_to_hex;

use crate::{
    pb::{
        pinax::beacon::v1::ProposerSlashing as RawProposerSlashing,
        sf::beacon::r#type::v1::{BeaconBlockHeader, ProposerSlashing},
    },
    structs::BlockTimestamp,
};

pub fn collect_proposer_slashings(proposer_slashings: &[ProposerSlashing], timestamp: &BlockTimestamp) -> Vec<RawProposerSlashing> {
    let mut vec = Vec::<RawProposerSlashing>::new();

    for (index, proposer_slashing) in proposer_slashings.iter().enumerate() {
        let signed_header_1 = proposer_slashing.signed_header_1.as_ref().unwrap();
        let signed_header_2 = proposer_slashing.signed_header_2.as_ref().unwrap();
        let message_1 = signed_header_1.message.as_ref().unwrap();
        let message_2 = signed_header_2.message.as_ref().unwrap();
        vec.push(parse_proposer_slashings(index as u64, message_1, signed_header_1.signature.clone(), timestamp));
        vec.push(parse_proposer_slashings(index as u64, message_2, signed_header_2.signature.clone(), timestamp));
    }

    vec
}

pub fn parse_proposer_slashings(index: u64, message: &BeaconBlockHeader, signature: Vec<u8>, timestamp: &BlockTimestamp) -> RawProposerSlashing {
    RawProposerSlashing {
        block_time: timestamp.time.to_string(),
        block_number: timestamp.number,
        block_date: timestamp.date.clone(),
        block_hash: timestamp.hash.clone(),
        index,
        slot: message.slot,
        proposer_index: message.proposer_index,
        parent_root: bytes_to_hex(&message.parent_root),
        state_root: bytes_to_hex(&message.state_root),
        body_root: bytes_to_hex(&message.body_root),
        signature: bytes_to_hex(&signature),
    }
}
