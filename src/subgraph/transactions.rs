use substreams::pb::substreams::Clock;
use substreams_antelope::pb::TransactionTrace;
use substreams_entity_change::tables::Tables;

use super::{actions::insert_action, db_ops::insert_db_op, receivers::{get_receivers, insert_receiver}};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_transaction(tables: &mut Tables, clock: &Clock, transaction: &TransactionTrace) {
    let hash = &transaction.id;
    let index = transaction.index;
    let elapsed = transaction.elapsed;
    let net_usage = transaction.net_usage;

    // only include successful transactions
    let header = transaction.receipt.clone().unwrap_or_default();
    if header.status != 1 { return; }

    // TABLE::Transaction
    tables
        .create_row("Transaction", hash)

        .set("block", &clock.id) // pointer to Block
        .set_bigint("index", &index.to_string())
        .set("hash", hash)
        .set_bigint("elapsed", &elapsed.to_string())
        .set_bigint("netUsage", &net_usage.to_string())
    ;

    // TABLE::Action
    for trace in transaction.action_traces.iter() {
        insert_action(tables, clock, trace, transaction);
    }

    // TABLE::Receiver
    let receivers = get_receivers(transaction);
    for receiver in receivers.iter() {
        insert_receiver(tables, transaction, receiver);
    }

    // TABLE::DbOps
    let mut db_op_index = 0;
    for db_op in transaction.db_ops.iter() {
        insert_db_op(tables, db_op, transaction, db_op_index);
        db_op_index += 1;
    }
}
