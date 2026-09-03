use common::structs::BlockIdentity;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

use crate::pb::pinax::solana::v2::{AccountLookup, Instruction};
use crate::utils::account_keys_extended;

// Top-level instructions first, then the inner instruction sets, sharing one
// running `instruction_index` per transaction (firehose-parquet ordering).
pub fn collect_instructions(tx: &ConfirmedTransaction, index: u32, signature: &[u8], id: &BlockIdentity) -> Vec<Instruction> {
    let message = tx.transaction.as_ref().and_then(|t| t.message.as_ref());
    let keys = account_keys_extended(tx);
    let resolve = |i: u32| keys.get(i as usize).cloned().unwrap_or_default();
    let resolve_all = |accounts: &[u8]| accounts.iter().map(|&i| resolve(i as u32)).collect::<Vec<_>>();
    let mut rows = Vec::new();
    let mut next = 0u32;

    let row = |instruction_index: u32, program_id_index: u32, accounts: &[u8], data: &[u8], is_inner: bool, inner_index: Option<u32>, stack_height: Option<u32>| Instruction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        slot: id.block_num,
        transaction_index: index,
        instruction_index,
        program_id_index,
        accounts: accounts.to_vec(),
        data: data.to_vec(),
        is_inner,
        inner_index,
        stack_height,

        signature: signature.to_vec(),
        program_id: resolve(program_id_index),
        account_addresses: resolve_all(accounts),
    };

    if let Some(message) = message {
        for instruction in &message.instructions {
            rows.push(row(next, instruction.program_id_index, &instruction.accounts, &instruction.data, false, None, None));
            next += 1;
        }
    }
    if let Some(meta) = tx.meta.as_ref() {
        for set in &meta.inner_instructions {
            for inner in &set.instructions {
                rows.push(row(next, inner.program_id_index, &inner.accounts, &inner.data, true, Some(set.index), inner.stack_height));
                next += 1;
            }
        }
    }
    rows
}

pub fn collect_account_lookups(tx: &ConfirmedTransaction, index: u32, signature: &[u8], id: &BlockIdentity) -> Vec<AccountLookup> {
    let Some(message) = tx.transaction.as_ref().and_then(|t| t.message.as_ref()) else {
        return vec![];
    };
    message
        .address_table_lookups
        .iter()
        .enumerate()
        .map(|(lookup_index, lookup)| AccountLookup {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            slot: id.block_num,
            transaction_index: index,
            lookup_index: lookup_index as u32,
            account_key: lookup.account_key.clone(),
            writable_indexes: lookup.writable_indexes.clone(),
            readonly_indexes: lookup.readonly_indexes.clone(),

            signature: signature.to_vec(),
        })
        .collect()
}
