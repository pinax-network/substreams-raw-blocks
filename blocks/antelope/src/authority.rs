use crate::{keys::authority_keys, transactions::insert_transaction_metadata};
use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{Authority, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn insert_authority(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace, action_index: u32, authority: &Authority) {
    let mut index = 0;
    let tx_hash = &transaction.id;

    // `accounts` TABLE
    for account in &authority.accounts {
        let permission_level = account.permission.as_ref().expect("account.permission is missing");
        let actor = permission_level.actor.as_str();
        let permission = permission_level.permission.as_str();
        let weight = account.weight;

        let keys = authority_keys(tx_hash, &action_index, &index);
        let row = tables
            .push_change_composite("accounts", keys, 0, table_change::Operation::Create)
            .change("index", ("", index.to_string().as_str()))
            .change("action_index", ("", action_index.to_string().as_str()))
            .change("actor", ("", actor))
            .change("permission", ("", permission))
            .change("weight", ("", weight.to_string().as_str()));
        insert_transaction_metadata(row, transaction);
        insert_timestamp(row, clock, false, false);
        index += 1;
    }
    // `keys` TABLE
    for key in &authority.keys {
        let public_key = key.public_key.as_str();
        let weight = key.weight;

        let keys = authority_keys(tx_hash, &action_index, &index);
        let row = tables
            .push_change_composite("keys", keys, 0, table_change::Operation::Create)
            .change("index", ("", index.to_string().as_str()))
            .change("action_index", ("", action_index.to_string().as_str()))
            .change("public_key", ("", public_key))
            .change("weight", ("", weight.to_string().as_str()));
        insert_transaction_metadata(row, transaction);
        insert_timestamp(row, clock, false, false);
        index += 1;
    }
    // `waits` TABLE
    for wait in &authority.waits {
        let wait_sec = wait.wait_sec;
        let weight = wait.weight;

        let keys = authority_keys(tx_hash, &action_index, &index);
        let row = tables
            .push_change_composite("waits", keys, 0, table_change::Operation::Create)
            .change("index", ("", index.to_string().as_str()))
            .change("action_index", ("", action_index.to_string().as_str()))
            .change("wait_sec", ("", wait_sec.to_string().as_str()))
            .change("weight", ("", weight.to_string().as_str()));
        insert_transaction_metadata(row, transaction);
        insert_timestamp(row, clock, false, false);

        index += 1;
    }
}
