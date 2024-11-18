use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::AccountRamDelta;
use substreams_antelope::pb::{ActionTrace, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::account_ram_delta_keys;
use crate::pb::antelope::AccountRamDelta as RawAccountRamDelta;
use crate::transactions::insert_transaction_metadata;
use crate::transactions::is_transaction_success;
use common::structs::BlockTimestamp;
use substreams_antelope::Block;

pub fn insert_account_ram_delta(tables: &mut DatabaseChanges, clock: &Clock, action: &ActionTrace, transaction: &TransactionTrace, account_ram_delta: &AccountRamDelta, index: &u32) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_index = &action.execution_index;

    // account_ram_delta
    let account = &account_ram_delta.account;
    let delta = &account_ram_delta.delta;

    let keys = account_ram_delta_keys(&tx_hash, &action_index, account, delta);
    let row = tables
        .push_change_composite("account_ram_deltas", keys, 0, table_change::Operation::Create)
        .change("index", ("", index.to_string().as_str()))
        .change("tx_hash", ("", tx_hash.as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("account", ("", account.as_str()))
        .change("delta", ("", delta.to_string().as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}

pub fn collect_account_ram_deltas(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawAccountRamDelta> {
    let mut account_ram_deltas: Vec<RawAccountRamDelta> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for action_trace in transaction.action_traces.iter() {
            let action_index = action_trace.execution_index;

            for (index, delta) in action_trace.account_ram_deltas.iter().enumerate() {
                account_ram_deltas.push(RawAccountRamDelta {
                    block_time: Some(timestamp.time.clone()),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    tx_hash: tx_hash.clone(),
                    tx_success,
                    action_index,
                    index: index as u32,
                    account: delta.account.clone(),
                    delta: delta.delta,
                });
            }
        }
    }

    account_ram_deltas
}
