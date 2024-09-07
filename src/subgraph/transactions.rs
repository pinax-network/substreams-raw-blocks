use std::collections::HashSet;

use substreams::pb::substreams::Clock;
use substreams_antelope::pb::TransactionTrace;
use substreams_entity_change::tables::Tables;

use super::{actions::insert_action, db_ops::insert_db_op};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_transaction(params: &String, tables: &mut Tables, clock: &Clock, transaction: &TransactionTrace) -> bool {
    let hash = &transaction.id;
    let index = transaction.index;
    let elapsed = transaction.elapsed;
    let net_usage = transaction.net_usage;

    // only include successful transactions
    let header = transaction.receipt.clone().unwrap_or_default();
    if header.status != 1 { return false; }

    // TABLE::Action
    let mut is_match = false;
    let mut action_keys = HashSet::new();
    for trace in transaction.action_traces.iter() {
        match insert_action(params, tables, clock, trace, transaction) {
            Some(action_key) => {
                action_keys.insert(action_key);
                is_match = true;
            },
            None => {}
        }
    }

    // TABLE::DbOps
    let mut db_op_index = 0;
    for db_op in transaction.db_ops.iter() {
        if insert_db_op(params, tables, db_op, transaction, db_op_index, &action_keys) { is_match = true;}
        db_op_index += 1;
    }

    // TABLE::Transaction
    if is_match {
        tables
            .create_row("Transaction", hash)
            .set("block", &clock.id) // pointer to Block
            .set_bigint("index", &index.to_string())
            .set_bigint("elapsed", &elapsed.to_string())
            .set_bigint("netUsage", &net_usage.to_string())
        ;
        return true;
    }
    return false;
}
