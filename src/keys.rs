use std::collections::HashMap;

use substreams::pb::substreams::Clock;

use crate::utils::block_time_to_date;

pub fn blocks_keys(clock: &Clock ) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_number = clock.number.to_string();
    let block_hash = clock.id.to_string();
    keys.insert("block_date".to_string(), block_date);
    keys.insert("block_number".to_string(), block_number);
    keys.insert("block_hash".to_string(), block_hash);
    keys
}

pub fn transactions_keys(clock: &Clock, hash: &String) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock);
    keys.insert("hash".to_string(), hash.to_string());
    keys
}

pub fn actions_keys(clock: &Clock, tx_hash: &String, tx_index: &u64, action_ordinal: &u32) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("tx_index".to_string(), tx_index.to_string());
    keys.insert("action_ordinal".to_string(), action_ordinal.to_string());
    keys
}

pub fn receivers_keys(clock: &Clock, tx_hash: &String, action_ordinal: &u32, receiver: &String) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_ordinal".to_string(), action_ordinal.to_string());
    keys.insert("receiver".to_string(), receiver.to_string());
    keys
}

pub fn authorizations_keys(clock: &Clock, tx_hash: &String, action_ordinal: &u32, actor: &String, permission: &String) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_ordinal".to_string(), action_ordinal.to_string());
    keys.insert("actor".to_string(), actor.to_string());
    keys.insert("permission".to_string(), permission.to_string());
    keys
}