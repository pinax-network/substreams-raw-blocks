use substreams_antelope::pb::TransactionTrace;
use substreams_entity_change::tables::Tables;

use crate::keys::receiver_key;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L616
pub fn insert_receiver(tables: &mut Tables, transaction: &TransactionTrace, receiver: &String) {
    // transaction
    let tx_hash = &transaction.id;

    // TABLE::Receiver
    let key = receiver_key(tx_hash, receiver);
    tables
        .create_row("Receiver", &key)
        .set("transaction", tx_hash)
        .set("receiver", receiver)
    ;
}