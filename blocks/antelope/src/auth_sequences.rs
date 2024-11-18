use common::blocks::insert_timestamp;
use common::structs::BlockTimestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::AuthSequence;
use substreams_antelope::pb::{ActionTrace, TransactionTrace};
use substreams_antelope::Block;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::auth_sequence_keys;
use crate::pb::antelope::AuthSequence as RawAuthSequence;
use crate::transactions::insert_transaction_metadata;
use crate::transactions::is_transaction_success;

pub fn insert_auth_sequence(tables: &mut DatabaseChanges, clock: &Clock, action: &ActionTrace, transaction: &TransactionTrace, auth_sequence: &AuthSequence, index: &u32) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_index = &action.execution_index;

    // auth_sequence
    let account_name = &auth_sequence.account_name;
    let sequence = &auth_sequence.sequence;

    let keys = auth_sequence_keys(&tx_hash, &action_index, index);
    let row = tables
        .push_change_composite("auth_sequences", keys, 0, table_change::Operation::Create)
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("account_name", ("", account_name.as_str()))
        .change("sequence", ("", sequence.to_string().as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}

pub fn collect_auth_sequences(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawAuthSequence> {
    let mut auth_sequences: Vec<RawAuthSequence> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for action_trace in transaction.action_traces.iter() {
            let receipt = action_trace.receipt.as_ref().expect("Action trace receipt is required");

            let action_index = action_trace.execution_index;

            for (index, auth) in receipt.auth_sequence.iter().enumerate() {
                auth_sequences.push(RawAuthSequence {
                    block_time: Some(timestamp.time.clone()),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    tx_hash: tx_hash.clone(),
                    tx_success,
                    action_index,
                    index: index as u32,
                    account_name: auth.account_name.clone(),
                    sequence: auth.sequence,
                });
            }
        }
    }

    auth_sequences
}
