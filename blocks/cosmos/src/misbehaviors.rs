use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::Block;

use crate::pb::pinax::cosmos::v1::Misbehavior;

pub fn collect_misbehaviors(block: &Block, timestamp: &BlockTimestamp) -> Vec<Misbehavior> {
    let mut vec: Vec<Misbehavior> = vec![];

    for (index, misbehavior) in block.misbehavior.iter().enumerate() {
        let validator = misbehavior.validator.as_ref().unwrap();
        vec.push(Misbehavior {
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
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
