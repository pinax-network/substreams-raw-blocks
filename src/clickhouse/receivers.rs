use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_antelope::pb::{ActionTrace, TransactionTrace};

use crate::keys::receivers_keys;
use super::blocks::insert_timestamp;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L616
pub fn insert_receiver(tables: &mut DatabaseChanges, clock: &Clock, action: &ActionTrace, transaction: &TransactionTrace) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_ordinal = &action.action_ordinal;
    let receiver = &action.receiver;
    let keys = receivers_keys(clock, &tx_hash, &action_ordinal, &receiver);
    let row = tables
        .push_change_composite("receivers", keys, 0, table_change::Operation::Create)

        // transaction
        .change("tx_hash", ("", tx_hash.to_string().as_str()))

        // action
        .change("action_ordinal", ("", action_ordinal.to_string().as_str()))
        .change("receiver", ("", receiver.to_string().as_str()));

    insert_timestamp(row, clock);
}