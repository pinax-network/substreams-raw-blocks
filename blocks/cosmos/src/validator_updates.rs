use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::{pb::public_key, Block};

use crate::pb::pinax::cosmos::v1::ValidatorUpdate;

pub fn collect_validator_updates(block: &Block, timestamp: &BlockTimestamp) -> Vec<ValidatorUpdate> {
    let mut vec: Vec<ValidatorUpdate> = vec![];

    for (index, validator_update) in block.validator_updates.iter().enumerate() {
        let public_key = match validator_update.pub_key.as_ref().unwrap().sum.as_ref().unwrap() {
            public_key::Sum::Ed25519(bytes) => bytes,
            public_key::Sum::Secp256k1(bytes) => bytes,
        };

        vec.push(ValidatorUpdate {
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u32,
            public_key: Hex::encode(public_key),
            power: validator_update.power,
        });
    }
    vec
}
