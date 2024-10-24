use std::collections::HashMap;

pub fn reward_keys(block_hash: &String, pubkey: &String, reward_type: &String) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("block_hash".to_string(), block_hash.to_string());
    keys.insert("pubkey".to_string(), pubkey.to_string());
    keys.insert("reward_type".to_string(), reward_type.to_string());
    keys
}

pub fn instruction_keys(transaction_id: &str, instruction_index: &str) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("transaction_id".to_string(), transaction_id.to_string());
    keys.insert("instruction_index".to_string(), instruction_index.to_string());
    keys
}

pub fn inner_instruction_keys(transaction_id: &str, instruction_index: &str, inner_index: &str) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("transaction_id".to_string(), transaction_id.to_string());
    keys.insert("instruction_index".to_string(), instruction_index.to_string());
    keys.insert("inner_instruction_index".to_string(), inner_index.to_string());
    keys
}

pub fn account_activity_keys(transaction_id: &str, address: &str) -> HashMap<String, String> {
    let mut keys = HashMap::new();
    keys.insert("transaction_id".to_string(), transaction_id.to_string());
    keys.insert("address".to_string(), address.to_string());
    keys
}
