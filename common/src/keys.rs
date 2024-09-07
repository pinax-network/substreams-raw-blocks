use std::collections::HashMap;

use substreams::pb::substreams::Clock;

use crate::utils::{add_prefix_to_hex, block_time_to_date};

pub fn blocks_keys(clock: &Clock, is_block: bool) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_number = clock.number.to_string();
    let block_hash = add_prefix_to_hex(&clock.id);
    let prefix = if is_block { "" } else { "block_" };
    let block_date_key = format!("{}date", prefix);
    let block_number_key = format!("{}number", prefix);
    let block_hash_key = format!("{}hash", prefix);
    HashMap::from([(block_date_key, block_date), (block_number_key, block_number), (block_hash_key, block_hash)])
}
