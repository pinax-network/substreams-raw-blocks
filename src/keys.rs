pub fn action_key(tx_hash: &str, index: u32) -> String {
    format!("{}:{}", tx_hash, index)
}

pub fn authorization_key(action_key: &str, actor: &str, permission: &str) -> String {
    format!("{}:{}:{}", action_key, actor, permission)
}

pub fn db_ops_key(tx_hash: &str, execution_index: u32, db_op_index: u32) -> String {
    format!("{}:{}:{}", tx_hash, execution_index, db_op_index)
}

pub fn db_ops_table_key(code: &str, scope: &str, table_name: &str, primary_key: &str) -> String {
    format!("{}:{}:{}:{}", code, scope, table_name, primary_key)
}
