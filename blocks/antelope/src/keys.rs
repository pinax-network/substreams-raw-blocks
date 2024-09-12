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

pub fn transactions_keys(clock: &Clock, hash: &String) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    keys.insert("hash".to_string(), hash.to_string());
    keys
}

pub fn actions_keys(clock: &Clock, tx_hash: &String, index: &u32) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn db_ops_keys(clock: &Clock, tx_hash: &String, index: &u32) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}

pub fn authorizations_keys(clock: &Clock, tx_hash: &String, action_index: &u32, actor: &String, permission: &String) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("actor".to_string(), actor.to_string());
    keys.insert("permission".to_string(), permission.to_string());
    keys
}

pub fn auth_sequence_keys(clock: &Clock, tx_hash: &String, action_index: &u32, account_name: &String, sequence: &u64) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("account_name".to_string(), account_name.to_string());
    keys.insert("sequence".to_string(), sequence.to_string());
    keys
}

pub fn account_ram_delta_keys(clock: &Clock, tx_hash: &String, action_index: &u32, account: &String, delta: &i64) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("account".to_string(), account.to_string());
    keys.insert("delta".to_string(), delta.to_string());
    keys
}

pub fn perm_ops_keys(clock: &Clock, tx_hash: &String, action_index: &u32, operation: &String) -> HashMap<String, String> {
    let mut keys = clock_keys(clock);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("action_index".to_string(), action_index.to_string());
    keys.insert("operation".to_string(), operation.to_string());
    keys
}
