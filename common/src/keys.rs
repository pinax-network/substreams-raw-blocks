use std::collections::HashMap;

use substreams::pb::substreams::Clock;

use crate::utils::block_time_to_date;

pub fn blocks_keys(clock: &Clock) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_number = clock.number.to_string();

    HashMap::from([("date".to_string(), block_date), ("number".to_string(), block_number)])
}

pub fn transaction_keys(clock: &Clock, hash: &String) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();

    HashMap::from([("block_date".to_string(), block_date), ("hash".to_string(), hash.to_string())])
}

pub fn logs_keys(clock: &Clock, tx_hash: &String, index: &u32) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("tx_hash".to_string(), tx_hash.to_string()),
        ("index".to_string(), index.to_string()),
    ])
}

pub fn block_ordinal_keys(clock: &Clock, ordinal: &u64) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_number = clock.number.to_string();

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("block_number".to_string(), block_number),
        ("ordinal".to_string(), ordinal.to_string()),
    ])
}

pub fn traces_keys(clock: &Clock, tx_hash: &String, tx_index: &u32, index: &u32) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("tx_hash".to_string(), tx_hash.to_string()),
        ("tx_index".to_string(), tx_index.to_string()),
        ("index".to_string(), index.to_string()),
    ])
}
