use std::collections::HashMap;

pub fn event_keys(tx_hash: &str, index: &str) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("tx_hash".to_string(), tx_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}
