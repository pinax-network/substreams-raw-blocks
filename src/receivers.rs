use substreams_antelope::pb::{ActionTrace, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::keys::action_key;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L616
pub fn insert_receiver(tables: &mut Tables, action: &ActionTrace, transaction: &TransactionTrace, receiver: &String) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_ordinal = &action.action_ordinal;
    // let receiver = &action.receiver;

    let key = action_key(tx_hash, action_ordinal);
    tables
        .create_row("Receiver", &key)
        .set("action", &key)
        .set("receiver", receiver.to_string())
    ;
}