use std::collections::HashMap;

use substreams::pb::substreams::Clock;

use crate::utils::block_time_to_date;

pub fn block_keys(clock: &Clock) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_time = timestamp.seconds.to_string();
    let block_number = clock.number.to_string();
    let block_hash = format!("0x{}", clock.id);

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("block_time".to_string(), block_time),
        ("block_number".to_string(), block_number),
        ("block_hash".to_string(), block_hash)
    ])
}