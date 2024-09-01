use std::collections::HashSet;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::TransactionTrace;
use substreams_entity_change::tables::Tables;

use crate::actions::insert_action;
use crate::receivers::insert_receiver;

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
    let mut receivers = HashSet::new();
    for trace in transaction.action_traces.iter() {
        insert_action(tables, clock, trace, transaction);
        receivers.insert(&trace.receiver);
    }

    // TABLE::Receiver
    for receiver in receivers.iter() {
        insert_receiver(tables, transaction, receiver);
    }
}
