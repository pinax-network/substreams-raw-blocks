use common::utils::bytes_to_hex;

use crate::{
    pb::{pinax::beacon::v1::VoluntaryExit as RawVoluntaryExit, sf::beacon::r#type::v1::SignedVoluntaryExit},
    structs::BlockTimestamp,
};

pub fn collect_voluntary_exits(voluntary_exits: &[SignedVoluntaryExit], timestamp: &BlockTimestamp) -> Vec<RawVoluntaryExit> {
    let mut vec: Vec<RawVoluntaryExit> = Vec::new();

    for (index, voluntary_exit) in voluntary_exits.iter().enumerate() {
        let message = voluntary_exit.message.as_ref().unwrap();

        vec.push(RawVoluntaryExit {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u64,
            epoch: message.epoch,
            validator_index: message.validator_index,
            signature: bytes_to_hex(&voluntary_exit.signature),
        });
    }

    vec
}
