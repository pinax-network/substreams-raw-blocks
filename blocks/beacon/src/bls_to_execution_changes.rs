use common::utils::bytes_to_hex;

use crate::{
    pb::{pinax::beacon::v1::BlsToExecutionChange as RawBlsToExecutionChange, sf::beacon::r#type::v1::SignedBlsToExecutionChange},
    structs::BlockTimestamp,
};

pub fn collect_bls_to_execution_changes(bls_to_execution_changes: &Vec<SignedBlsToExecutionChange>, timestamp: &BlockTimestamp) -> Vec<RawBlsToExecutionChange> {
    let mut vec = Vec::<RawBlsToExecutionChange>::new();

    for (index, bls_to_execution_change) in bls_to_execution_changes.iter().enumerate() {
        let change = &bls_to_execution_change.message.as_ref().unwrap();

        vec.push(RawBlsToExecutionChange {
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u64,
            validator_index: change.validator_index,
            from_bls_pubkey: bytes_to_hex(&change.from_bls_pub_key),
            to_execution_address: bytes_to_hex(&change.to_execution_address),
            signature: bytes_to_hex(&bls_to_execution_change.signature),
        });
    }

    vec
}
