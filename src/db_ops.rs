use std::collections::{HashMap, HashSet};

use substreams::{pb::substreams::Clock, Hex};
use substreams_antelope::pb::{DbOp, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::{
    index::{collect_db_op_keys, is_match},
    keys::{action_key, db_ops_key, db_ops_table_key},
};

pub fn operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Update".to_string(),
        3 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn collapse_db_ops(transaction: &TransactionTrace) -> HashMap<String, DbOp> {
    let mut collapsed_db_ops: HashMap<String, DbOp> = HashMap::new();
    for db_op in transaction.db_ops.iter() {
        let code = db_op.code.as_str();
        let scope = db_op.scope.as_str();
        let table_name = db_op.table_name.as_str();
        let primary_key = db_op.primary_key.as_str();
        let table_key = db_ops_table_key(code, scope, table_name, primary_key);

        // first db ops, no need to inherit from previous db ops
        if !collapsed_db_ops.contains_key(&table_key) {
            collapsed_db_ops.insert(table_key, db_op.clone());
        // inherit from previous db ops
        // new_data and new_data_json are updated
        } else {
            let collapsed_db_op = collapsed_db_ops.get_mut(&table_key).unwrap();
            collapsed_db_op.new_data = db_op.new_data.clone();
            collapsed_db_op.new_data_json = db_op.new_data_json.clone();
        }
    }
    collapsed_db_ops
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
pub fn insert_db_op(params: &str, tables: &mut Tables, clock: &Clock, db_op: &DbOp, transaction: &TransactionTrace, index: u32, action_keys: &HashSet<String>) -> bool {
    let operation = operation_to_string(db_op.operation);
    let action_index = db_op.action_index;
    let code = db_op.code.as_str();
    let scope = db_op.scope.as_str();
    let table_name = db_op.table_name.as_str();
    let primary_key = db_op.primary_key.as_str();
    let old_data = Hex::encode(db_op.old_data.to_vec());
    let new_data = Hex::encode(db_op.new_data.to_vec());
    let old_data_json = db_op.old_data_json.as_str();
    let new_data_json = db_op.new_data_json.as_str();

    // transaction
    let tx_hash = transaction.id.as_str();

    // TABLE::DbOps
    let action_key = action_key(tx_hash, action_index);
    let key = db_ops_key(tx_hash, action_index, index);
    if is_match(collect_db_op_keys(db_op), params) || action_keys.contains(&action_key) {
        tables
            .create_row("DbOp", key)
            // pointers
            .set("transaction", tx_hash)
            .set("action", action_key)
            .set("block", &clock.id)
            // DbOp
            .set_bigint("index", &index.to_string())
            .set("operation", operation)
            .set_bigint("actionIndex", &action_index.to_string())
            .set("code", code)
            .set("scope", scope)
            .set("tableName", table_name)
            .set("primaryKey", primary_key)
            .set("oldData", old_data)
            .set("newData", new_data)
            .set("oldDataJson", old_data_json)
            .set("newDataJson", new_data_json);
        return true;
    }
    return false;
}
