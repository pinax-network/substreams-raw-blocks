use std::collections::HashMap;

use common::keys::blocks_keys;
use substreams::pb::substreams::Clock;

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

pub fn traces_keys(clock: &Clock, tx_hash: &String, tx_index: &u64, index: &u32) -> HashMap<String, String> {
    let mut keys = blocks_keys(clock, false);
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("tx_index".to_string(), tx_index.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}
