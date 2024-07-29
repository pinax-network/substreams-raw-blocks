use std::collections::HashMap;

use substreams::pb::substreams::Clock;

use crate::utils::block_time_to_date;

pub fn blocks_keys(clock: &Clock) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_time = timestamp.seconds.to_string();
    let block_number = clock.number.to_string();
    let block_hash = format!("0x{}", clock.id);

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("block_time".to_string(), block_time),
        ("block_number".to_string(), block_number),
        ("block_hash".to_string(), block_hash),
    ])
}

pub fn logs_keys(clock: &Clock, log_index: &String, tx_hash: &String) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_time = timestamp.seconds.to_string();
    let block_number = clock.number.to_string();

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("block_time".to_string(), block_time),
        ("block_number".to_string(), block_number),
        ("log_index".to_string(), log_index.to_string()),
        ("tx_hash".to_string(), tx_hash.to_string()),
    ])
}

pub fn balance_changes_keys(clock: &Clock, ordinal: &String) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_time = timestamp.seconds.to_string();
    let block_number = clock.number.to_string();

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("block_time".to_string(), block_time),
        ("block_number".to_string(), block_number),
        ("ordinal".to_string(), ordinal.to_string()),
    ])
}

pub fn traces_keys(clock: &Clock, tx_hash: &String, tx_index: &String, index: &String) -> HashMap<String, String> {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_time = timestamp.seconds.to_string();
    let block_number = clock.number.to_string();

    HashMap::from([
        ("block_date".to_string(), block_date),
        ("block_time".to_string(), block_time),
        ("block_number".to_string(), block_number),
        ("tx_hash".to_string(), tx_hash.to_string()),
        ("tx_index".to_string(), tx_index.to_string()),
        ("index".to_string(), index.to_string()),
    ])
}
