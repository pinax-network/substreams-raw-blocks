pub fn action_key(tx_hash: &String, index: &u32) -> String {
    format!("{}:{}", tx_hash, index)
}

pub fn authorization_key(action_key: &String, actor: &String, permission: &String) -> String {
    format!("{}:{}:{}", action_key, actor, permission)
}

pub fn db_ops_key(tx_hash: &String, execution_index: &u32, db_op_index: &u32) -> String {
    format!("{}:{}:{}", tx_hash, execution_index, db_op_index)
}