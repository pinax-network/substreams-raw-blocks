use common::structs::BlockTimestamp;
use substreams_antelope::Block;

use crate::pb::antelope::AuthSequence as RawAuthSequence;
use crate::transactions::is_transaction_success;

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
