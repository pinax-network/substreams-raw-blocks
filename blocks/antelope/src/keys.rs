use std::collections::HashMap;

use common::utils::block_time_to_date;
use substreams::pb::substreams::Clock;

pub fn blocks_keys(clock: &Clock) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    let block_hash = clock.id.to_string();
    keys.insert("block_hash".to_string(), block_hash);
    keys
}

pub fn clock_keys(clock: &Clock) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(&timestamp.to_string()).to_string();
    let block_number = clock.number.to_string();
    keys.insert("block_date".to_string(), block_date);
    keys.insert("block_number".to_string(), block_number);
    keys
}

pub fn transactions_keys(hash: &String) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("hash".to_string(), hash.to_string());
    keys
}

pub fn actions_keys(tx_hash: &String, index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn db_ops_keys(tx_hash: &String, index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn authorizations_keys(tx_hash: &String, action_index: &u32, index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn auth_sequence_keys(tx_hash: &String, action_index: &u32, index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn account_ram_delta_keys(tx_hash: &String, action_index: &u32, account: &String, delta: &i64) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("account".to_string(), account.to_string());
    keys.insert("delta".to_string(), delta.to_string());
    keys
}

pub fn perm_ops_keys(tx_hash: &String, action_index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys
}

pub fn authority_keys(tx_hash: &str, action_index: &u32, index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn feature_ops_keys(feature_digest: &str) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("feature_digest".to_string(), feature_digest.to_string());
    keys
}

pub fn creation_tree_keys(tx_hash: &String, creator_action_index: &i32, execution_action_index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("creator_action_index".to_string(), creator_action_index.to_string());
    keys.insert("execution_action_index".to_string(), execution_action_index.to_string());
    keys
}

pub fn table_ops_keys(tx_hash: &String, index: &u32) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn ram_op_keys(tx_id: &str, action_index: &u32, unique_key: &str) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_id.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("unique_key".to_string(), unique_key.to_string());
    keys
}
