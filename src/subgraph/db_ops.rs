use substreams::Hex;
use substreams_antelope::pb::{DbOp, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::{keys::{action_key, db_ops_key}, utils::is_match};

pub fn operation_to_string(operation: i32) -> String {
    match operation {
        0 => "Unknown".to_string(),
        1 => "Insert".to_string(),
        2 => "Update".to_string(),
        3 => "Remove".to_string(),
        _ => "Unknown".to_string(),
    }
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
pub fn insert_db_op(params: &String, tables: &mut Tables, db_op: &DbOp, transaction: &TransactionTrace, index: u32) {
	let operation = operation_to_string(db_op.operation);
	let action_ordinal = db_op.action_index;
	let code = &db_op.code;
	let scope = &db_op.scope;
	let table_name = &db_op.table_name;
	let primary_key = &db_op.primary_key;
	let old_data = Hex::encode(&db_op.old_data.to_vec());
	let new_data = Hex::encode(&db_op.new_data.to_vec());
	let old_data_json = &db_op.old_data_json;
	let new_data_json = &db_op.new_data_json;

    // transaction
    let tx_hash = &transaction.id;

    // TABLE::DbOps
    let action_key = action_key(tx_hash, &action_ordinal);
    let key = db_ops_key(&tx_hash, &action_ordinal, &index);
    if is_match("table:DbOp", params) {
        tables
            .create_row("DbOp", key)
            .set("transaction", tx_hash)
            .set("action", action_key)
            .set_bigint("index", &index.to_string())
            .set("operation", operation.to_string())
            .set_bigint("actionOrdinal", &action_ordinal.to_string())
            .set("code", code.to_string())
            .set("scope", scope.to_string())
            .set("tableName", table_name.to_string())
            .set("primaryKey", primary_key.to_string())
            .set("oldData", old_data.to_string())
            .set("newData", new_data.to_string())
            .set("oldDataJson", old_data_json.to_string())
            .set("newDataJson", new_data_json.to_string())
            ;
    }
}
