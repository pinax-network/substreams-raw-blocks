// State changes recorded inside calls (DetailLevel: EXTENDED).
//
// `collect_*` functions take a transaction and one of its calls and fill the
// transaction-scoped tables; `collect_system_*` take a system call and fill the
// `system_*` tables; `collect_block_*` take the block's own changes (rewards,
// withdrawals, genesis...) which have neither a transaction nor a call.
use common::structs::BlockIdentity;
use substreams_ethereum::pb::eth::v2::{Block, Call, TransactionTrace};

use crate::pb::pinax::evm::v2::{
    AccountCreation, BalanceChange, CodeChange, GasChange, KeccakPreimage, NonceChange, StorageChange, SystemAccountCreation, SystemBalanceChange, SystemCodeChange, SystemGasChange,
    SystemKeccakPreimage, SystemNonceChange, SystemStorageChange,
};
use crate::utils::{balance_change_reason_text, bigint_to_string, gas_change_reason_text, hex_to_bytes};

pub fn collect_balance_changes(tx: &TransactionTrace, call: &Call, id: &BlockIdentity) -> Vec<BalanceChange> {
    call.balance_changes
        .iter()
        .map(|change| BalanceChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            ordinal: change.ordinal,
            address: change.address.clone(),
            old_value: bigint_to_string(&change.old_value),
            new_value: bigint_to_string(&change.new_value),
            reason: balance_change_reason_text(change.reason),

            tx_index: tx.index,
            call_index: call.index,
        })
        .collect()
}

pub fn collect_code_changes(tx: &TransactionTrace, call: &Call, id: &BlockIdentity) -> Vec<CodeChange> {
    call.code_changes
        .iter()
        .map(|change| CodeChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            ordinal: change.ordinal,
            address: change.address.clone(),
            old_hash: change.old_hash.clone(),
            new_hash: change.new_hash.clone(),
            old_code: change.old_code.clone(),
            new_code: change.new_code.clone(),

            tx_index: tx.index,
            call_index: call.index,
        })
        .collect()
}

pub fn collect_storage_changes(tx: &TransactionTrace, call: &Call, id: &BlockIdentity) -> Vec<StorageChange> {
    call.storage_changes
        .iter()
        .map(|change| StorageChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            ordinal: change.ordinal,
            address: change.address.clone(),
            key: change.key.clone(),
            old_value: change.old_value.clone(),
            new_value: change.new_value.clone(),

            tx_index: tx.index,
            call_index: call.index,
        })
        .collect()
}

pub fn collect_nonce_changes(tx: &TransactionTrace, call: &Call, id: &BlockIdentity) -> Vec<NonceChange> {
    call.nonce_changes
        .iter()
        .map(|change| NonceChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            ordinal: change.ordinal,
            address: change.address.clone(),
            old_value: change.old_value,
            new_value: change.new_value,

            tx_index: tx.index,
            call_index: call.index,
        })
        .collect()
}

pub fn collect_gas_changes(tx: &TransactionTrace, call: &Call, id: &BlockIdentity) -> Vec<GasChange> {
    call.gas_changes
        .iter()
        .map(|change| GasChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            ordinal: change.ordinal,
            old_value: change.old_value,
            new_value: change.new_value,
            reason: gas_change_reason_text(change.reason),

            tx_index: tx.index,
            call_index: call.index,
        })
        .collect()
}

// `Call::account_creations` is deprecated upstream (not populated since block version 4),
// but raw blocks keep extracting it for historical blocks where it is set.
#[allow(deprecated)]
pub fn collect_account_creations(tx: &TransactionTrace, call: &Call, id: &BlockIdentity) -> Vec<AccountCreation> {
    call.account_creations
        .iter()
        .map(|creation| AccountCreation {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            ordinal: creation.ordinal,
            account: creation.account.clone(),

            tx_index: tx.index,
            call_index: call.index,
        })
        .collect()
}

pub fn collect_keccak_preimages(tx: &TransactionTrace, call: &Call, id: &BlockIdentity) -> Vec<KeccakPreimage> {
    let mut preimages: Vec<(&String, &String)> = call.keccak_preimages.iter().collect();
    preimages.sort(); // HashMap iteration order is not deterministic
    preimages
        .into_iter()
        .map(|(hash, preimage)| KeccakPreimage {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            tx_index: tx.index,
            call_index: call.index,
            hash: hex_to_bytes(hash),
            preimage: hex_to_bytes(preimage),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Block-level changes (no transaction, no call)
// ---------------------------------------------------------------------------

pub fn collect_block_balance_changes(block: &Block, id: &BlockIdentity) -> Vec<SystemBalanceChange> {
    block.balance_changes.iter().map(|change| system_balance_change(change, None, id)).collect()
}

pub fn collect_block_code_changes(block: &Block, id: &BlockIdentity) -> Vec<SystemCodeChange> {
    block.code_changes.iter().map(|change| system_code_change(change, None, id)).collect()
}

// ---------------------------------------------------------------------------
// System call changes
// ---------------------------------------------------------------------------

pub fn collect_system_balance_changes(call: &Call, id: &BlockIdentity) -> Vec<SystemBalanceChange> {
    call.balance_changes.iter().map(|change| system_balance_change(change, Some(call.index), id)).collect()
}

pub fn collect_system_code_changes(call: &Call, id: &BlockIdentity) -> Vec<SystemCodeChange> {
    call.code_changes.iter().map(|change| system_code_change(change, Some(call.index), id)).collect()
}

pub fn collect_system_storage_changes(call: &Call, id: &BlockIdentity) -> Vec<SystemStorageChange> {
    call.storage_changes
        .iter()
        .map(|change| SystemStorageChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            ordinal: change.ordinal,
            address: change.address.clone(),
            key: change.key.clone(),
            old_value: change.old_value.clone(),
            new_value: change.new_value.clone(),

            call_index: call.index,
        })
        .collect()
}

pub fn collect_system_nonce_changes(call: &Call, id: &BlockIdentity) -> Vec<SystemNonceChange> {
    call.nonce_changes
        .iter()
        .map(|change| SystemNonceChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            ordinal: change.ordinal,
            address: change.address.clone(),
            old_value: change.old_value,
            new_value: change.new_value,

            call_index: call.index,
        })
        .collect()
}

pub fn collect_system_gas_changes(call: &Call, id: &BlockIdentity) -> Vec<SystemGasChange> {
    call.gas_changes
        .iter()
        .map(|change| SystemGasChange {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            ordinal: change.ordinal,
            old_value: change.old_value,
            new_value: change.new_value,
            reason: gas_change_reason_text(change.reason),

            call_index: call.index,
        })
        .collect()
}

#[allow(deprecated)]
pub fn collect_system_account_creations(call: &Call, id: &BlockIdentity) -> Vec<SystemAccountCreation> {
    call.account_creations
        .iter()
        .map(|creation| SystemAccountCreation {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            ordinal: creation.ordinal,
            account: creation.account.clone(),

            call_index: call.index,
        })
        .collect()
}

pub fn collect_system_keccak_preimages(call: &Call, id: &BlockIdentity) -> Vec<SystemKeccakPreimage> {
    let mut preimages: Vec<(&String, &String)> = call.keccak_preimages.iter().collect();
    preimages.sort();
    preimages
        .into_iter()
        .map(|(hash, preimage)| SystemKeccakPreimage {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            call_index: call.index,
            hash: hex_to_bytes(hash),
            preimage: hex_to_bytes(preimage),
        })
        .collect()
}

fn system_balance_change(change: &substreams_ethereum::pb::eth::v2::BalanceChange, call_index: Option<u32>, id: &BlockIdentity) -> SystemBalanceChange {
    SystemBalanceChange {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        block_number: id.block_num,
        ordinal: change.ordinal,
        address: change.address.clone(),
        old_value: bigint_to_string(&change.old_value),
        new_value: bigint_to_string(&change.new_value),
        reason: balance_change_reason_text(change.reason),

        call_index,
    }
}

fn system_code_change(change: &substreams_ethereum::pb::eth::v2::CodeChange, call_index: Option<u32>, id: &BlockIdentity) -> SystemCodeChange {
    SystemCodeChange {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        block_number: id.block_num,
        ordinal: change.ordinal,
        address: change.address.clone(),
        old_hash: change.old_hash.clone(),
        new_hash: change.new_hash.clone(),
        old_code: change.old_code.clone(),
        new_code: change.new_code.clone(),

        call_index,
    }
}
