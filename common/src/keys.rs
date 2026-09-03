use std::collections::HashMap;

use prost_types::Timestamp;

use crate::utils::{add_prefix_to_hex, block_time_to_date};

// Takes the `sf.substreams.v1.Clock` fields rather than the `Clock` type so this
// crate does not depend on a specific `substreams` version.
pub fn blocks_keys(timestamp: &Option<Timestamp>, id: &str, number: u64, is_block: bool) -> HashMap<String, String> {
    let timestamp = timestamp.clone().unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_number = number.to_string();
    let block_hash = add_prefix_to_hex(id);
    let prefix = if is_block { "" } else { "block_" };
    let block_date_key = format!("{}date", prefix);
    let block_number_key = format!("{}number", prefix);
    let block_hash_key = format!("{}hash", prefix);
    HashMap::from([(block_date_key, block_date), (block_number_key, block_number), (block_hash_key, block_hash)])
}
