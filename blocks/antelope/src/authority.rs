use crate::{
    pb::antelope::{Account as RawAccount, Key as RawKey, Wait as RawWait},
    transactions::is_transaction_success,
};
use common::structs::BlockTimestamp;
use substreams_antelope::Block;

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
