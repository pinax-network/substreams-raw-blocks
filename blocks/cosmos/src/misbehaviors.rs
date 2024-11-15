use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::Block;

use crate::pb::cosmos::Misbehavior as RawMisbehavior;

pub fn collect_misbehaviors(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawMisbehavior> {
    let mut vec: Vec<RawMisbehavior> = vec![];

    for (index, misbehavior) in block.misbehavior.iter().enumerate() {
        let validator = misbehavior.validator.as_ref().unwrap();
        vec.push(RawMisbehavior {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: Hex::encode(&block.hash),
            index: index as u32,
            r#type: misbehavior_type_to_string(&misbehavior.r#type),
            validator_address: Hex::encode(&validator.address),
            validator_power: validator.power,
            height: misbehavior.height,
            time: misbehavior.time,
            total_voting_power: misbehavior.total_voting_power,
        });
    }

    vec
}

fn misbehavior_type_to_string(misbehavior_type: &i32) -> String {
    match misbehavior_type {
        0 => "Unknown".to_string(),
        1 => "DuplicateVote".to_string(),
        2 => "LightClientAttack".to_string(),
        _ => "Undefined".to_string(),
    }
}
