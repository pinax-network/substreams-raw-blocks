use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::AuthSequence;
use substreams_antelope::pb::{ActionTrace, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::auth_sequence_keys;

pub fn insert_auth_sequence(tables: &mut DatabaseChanges, clock: &Clock, action: &ActionTrace, transaction: &TransactionTrace, auth_sequence: &AuthSequence) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_index = &action.execution_index;

    // auth_sequence
    let account_name = &auth_sequence.account_name;
    let sequence = &auth_sequence.sequence;

    let keys = auth_sequence_keys(clock, &tx_hash, &action_index, account_name, sequence);
    let row = tables
        .push_change_composite("auth_sequences", keys, 0, table_change::Operation::Create)
        .change("tx_hash", ("", tx_hash.as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("account_name", ("", account_name.as_str()))
        .change("sequence", ("", sequence.to_string().as_str()));

    insert_timestamp(row, clock, false, false);
}