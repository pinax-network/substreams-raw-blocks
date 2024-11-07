use std::collections::HashMap;

pub fn blob_keys(block_hash: &str, index: u64) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("block_hash".to_string(), block_hash.to_string());
    keys.insert("index".to_string(), index.to_string());
    keys
}
