use std::collections::HashMap;

pub fn reward_keys(block_hash: &String, pubkey: &String, reward_type: &String) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("block_hash".to_string(), block_hash.to_string());
    keys.insert("pubkey".to_string(), pubkey.to_string());
    keys.insert("reward_type".to_string(), reward_type.to_string());
    keys
}
