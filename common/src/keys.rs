use std::collections::HashMap;

use substreams::pb::substreams::Clock;

use crate::utils::block_time_to_date;

pub fn blocks_keys(clock: &Clock, is_block: bool) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_number = clock.number.to_string();
    let prefix = if is_block { "" } else { "block_" };
    let block_date_key = format!("{}date", prefix);
    let block_number_key = format!("{}number", prefix);
    HashMap::from([(block_date_key, block_date), (block_number_key, block_number)])
}

pub fn transaction_keys(clock: &Clock, hash: &String) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock, false);
    keys.insert("hash".to_string(), hash.to_string());
    keys
}

pub fn logs_keys(clock: &Clock, tx_hash: &String, index: &u32) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock, false);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn block_ordinal_keys(clock: &Clock, ordinal: &u64) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock, false);
    keys.insert("ordinal".to_string(), ordinal.to_string());
    keys
}

pub fn traces_keys(clock: &Clock, tx_hash: &String, tx_index: &u32, index: &u32) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock, false);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("tx_index".to_string(), tx_index.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn system_traces_keys(clock: &Clock, index: &u32) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock, false);
    keys.insert("index".to_string(), index.to_string());
    keys
}
