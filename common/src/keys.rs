use std::collections::HashMap;

use substreams::pb::substreams::Clock;

use crate::utils::block_time_to_date;

pub fn block_keys(clock: &Clock) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_time = timestamp.to_string();
    let block_number = clock.number.to_string();
    let block_hash = format!("0x{}", clock.id);
    let block_date = block_time_to_date(block_time.as_str()).to_string();

    HashMap::from([
        ("block_time".to_string(), block_time),
        ("block_number".to_string(), block_number),
        ("block_hash".to_string(), block_hash),
        ("block_date".to_string(), block_date)
    ])
}