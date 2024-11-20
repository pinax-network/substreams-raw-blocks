use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;

use crate::pb::antelope::AuthSequence;

pub fn collect_tx_auth_sequences(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<AuthSequence> {
    let mut auth_sequences = Vec::new();

    for action_trace in transaction.action_traces.iter() {
        let receipt = action_trace.receipt.as_ref().expect("Action trace receipt is required");
        let action_index = action_trace.execution_index;

        for (index, auth) in receipt.auth_sequence.iter().enumerate() {
            auth_sequences.push(AuthSequence {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: transaction.id.clone(),
                tx_success,
                action_index,
                index: index as u32,
                account_name: auth.account_name.clone(),
                sequence: auth.sequence,
            });
        }
    }

    auth_sequences
}
