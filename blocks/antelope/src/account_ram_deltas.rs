use crate::pb::antelope::AccountRamDelta as RawAccountRamDelta;
use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;

pub fn collect_tx_account_ram_deltas(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<RawAccountRamDelta> {
    let mut account_ram_deltas = Vec::new();

    for action_trace in transaction.action_traces.iter() {
        let action_index = action_trace.execution_index;

        for (index, delta) in action_trace.account_ram_deltas.iter().enumerate() {
            account_ram_deltas.push(RawAccountRamDelta {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: transaction.id.clone(),
                tx_success,
                action_index,
                index: index as u32,
                account: delta.account.clone(),
                delta: delta.delta,
            });
        }
    }

    account_ram_deltas
}
