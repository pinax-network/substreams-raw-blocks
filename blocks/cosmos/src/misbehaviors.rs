use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_cosmos::Block;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::misbehavior_keys;

pub fn insert_misbehaviors(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    for (index, misbehavior) in block.misbehavior.iter().enumerate() {
        let misbehavior_type = misbehavior_type_to_string(&misbehavior.r#type);
        let validator = misbehavior.validator.as_ref().unwrap();
        let validator_address = bytes_to_hex(&validator.address);
        let validator_power = validator.power;
        let height = misbehavior.height;
        let time = misbehavior.time.as_ref().unwrap();
        let total_voting_power = misbehavior.total_voting_power;
        let index_str = index.to_string();
        let keys = misbehavior_keys(&clock.number.to_string(), &index_str);

        let row = tables
            .push_change_composite("misbehaviors", keys, 0, table_change::Operation::Create)
            .change("index", ("", index_str.as_str()))
            .change("type", ("", misbehavior_type.as_str()))
            .change("validator_address", ("", validator_address.as_str()))
            .change("validator_power", ("", validator_power.to_string().as_str()))
            .change("height", ("", height.to_string().as_str()))
            .change("time", ("", time.to_string().as_str()))
            .change("total_voting_power", ("", total_voting_power.to_string().as_str()));

        insert_timestamp(row, clock, false, true);
    }
}

fn misbehavior_type_to_string(misbehavior_type: &i32) -> String {
    match misbehavior_type {
        0 => "Unknown".to_string(),
        1 => "DuplicateVote".to_string(),
        2 => "LightClientAttack".to_string(),
        _ => "Undefined".to_string(),
    }
}
