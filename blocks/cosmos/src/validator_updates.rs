use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::{pb::public_key, Block};

use crate::pb::cosmos::ValidatorUpdate as RawValidatorUpdate;

pub fn collect_validator_updates(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawValidatorUpdate> {
    let mut vec: Vec<RawValidatorUpdate> = vec![];

    for (index, validator_update) in block.validator_updates.iter().enumerate() {
        let public_key = match validator_update.pub_key.as_ref().unwrap().sum.as_ref().unwrap() {
            public_key::Sum::Ed25519(bytes) => bytes,
            public_key::Sum::Secp256k1(bytes) => bytes,
        };

        vec.push(RawValidatorUpdate {
            block_time: Some(timestamp.time),
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
