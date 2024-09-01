pub fn action_key(tx_hash: &String, action_ordinal: &u32) -> String {
    format!("{}:{}", tx_hash, action_ordinal)
}

pub fn authorization_key(action_key: &String, actor: &String, permission: &String) -> String {
    format!("{}:{}:{}", action_key, actor, permission)
}