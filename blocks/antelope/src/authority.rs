use crate::{
    keys::authority_keys,
    pb::antelope::{Account as RawAccount, Key as RawKey, Wait as RawWait},
    transactions::{insert_transaction_metadata, is_transaction_success},
};
use common::{blocks::insert_timestamp, structs::BlockTimestamp};
use substreams::pb::substreams::Clock;
use substreams_antelope::{
    pb::{Authority, TransactionTrace},
    Block,
};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub struct AuthorityVectors {
    pub accounts: Vec<RawAccount>,
    pub keys: Vec<RawKey>,
    pub waits: Vec<RawWait>,
}

pub fn collect_authority_vectors(block: &Block, timestamp: &BlockTimestamp) -> AuthorityVectors {
    let mut accounts: Vec<RawAccount> = vec![];
    let mut keys: Vec<RawKey> = vec![];
    let mut waits: Vec<RawWait> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for perm_op in transaction.perm_ops.iter() {
            if let Some(new_perm) = &perm_op.new_perm {
                let authority = new_perm.authority.as_ref().unwrap();
                let action_index = perm_op.action_index;

                // Process authority
                for (index, account) in authority.accounts.iter().enumerate() {
                    if let Some(permission) = &account.permission {
                        accounts.push(RawAccount {
                            block_time: Some(timestamp.time.clone()),
                            block_number: timestamp.number,
                            block_hash: timestamp.hash.clone(),
                            block_date: timestamp.date.clone(),
                            tx_hash: tx_hash.clone(),
                            tx_success,
                            index: index as u32,
                            action_index: perm_op.action_index as u32,
                            actor: permission.actor.clone(),
                            permission: permission.permission.clone(),
                            weight: account.weight,
                        });
                    }
                }

                // Process keys
                for (index, key) in authority.keys.iter().enumerate() {
                    keys.push(RawKey {
                        block_time: Some(timestamp.time.clone()),
                        block_number: timestamp.number,
                        block_hash: timestamp.hash.clone(),
                        block_date: timestamp.date.clone(),
                        tx_hash: tx_hash.clone(),
                        tx_success,
                        index: index as u32,
                        action_index: action_index as u32,
                        public_key: key.public_key.clone(),
                        weight: key.weight,
                    });
                }

                // Process waits
                for (index, wait) in authority.waits.iter().enumerate() {
                    waits.push(RawWait {
                        block_time: Some(timestamp.time.clone()),
                        block_number: timestamp.number,
                        block_hash: timestamp.hash.clone(),
                        block_date: timestamp.date.clone(),
                        tx_hash: tx_hash.clone(),
                        tx_success,
                        index: index as u32,
                        action_index: action_index as u32,
                        wait_sec: wait.wait_sec,
                        weight: wait.weight,
                    });
                }
            }
        }
    }

    AuthorityVectors { accounts, keys, waits }
}

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
