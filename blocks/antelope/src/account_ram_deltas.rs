use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::AccountRamDelta;
use substreams_antelope::pb::{ActionTrace, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::account_ram_delta_keys;
use crate::transactions::insert_transaction_metadata;

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
