use crate::pb::antelope::{Account, Key, Wait};
use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;

pub struct AuthorityVectors {
    pub accounts: Vec<Account>,
    pub keys: Vec<Key>,
    pub waits: Vec<Wait>,
}

pub fn collect_tx_authority_vectors(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> AuthorityVectors {
    let mut accounts: Vec<Account> = Vec::new();
    let mut keys: Vec<Key> = Vec::new();
    let mut waits: Vec<Wait> = Vec::new();

    for perm_op in transaction.perm_ops.iter() {
        if let Some(new_perm) = &perm_op.new_perm {
            let authority = new_perm.authority.as_ref().unwrap();
            let action_index = perm_op.action_index;

            // Process authority accounts
            for (index, account) in authority.accounts.iter().enumerate() {
                if let Some(permission) = &account.permission {
                    accounts.push(Account {
                        block_time: Some(timestamp.time.clone()),
                        block_number: timestamp.number,
                        block_hash: timestamp.hash.clone(),
                        block_date: timestamp.date.clone(),
                        tx_hash: transaction.id.clone(),
                        tx_success,
                        index: index as u32,
                        action_index: action_index as u32,
                        actor: permission.actor.clone(),
                        permission: permission.permission.clone(),
                        weight: account.weight,
                    });
                }
            }

            // Process authority keys
            for (index, key) in authority.keys.iter().enumerate() {
                keys.push(Key {
                    block_time: Some(timestamp.time.clone()),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    tx_hash: transaction.id.clone(),
                    tx_success,
                    index: index as u32,
                    action_index: action_index as u32,
                    public_key: key.public_key.clone(),
                    weight: key.weight,
                });
            }

            // Process authority waits
            for (index, wait) in authority.waits.iter().enumerate() {
                waits.push(Wait {
                    block_time: Some(timestamp.time.clone()),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    tx_hash: transaction.id.clone(),
                    tx_success,
                    index: index as u32,
                    action_index: action_index as u32,
                    wait_sec: wait.wait_sec,
                    weight: wait.weight,
                });
            }
        }
    }

    AuthorityVectors { accounts, keys, waits }
}
