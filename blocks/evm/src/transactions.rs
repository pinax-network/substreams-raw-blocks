use common::structs::BlockIdentity;
use substreams_ethereum::pb::eth::v2::TransactionTrace;

use crate::pb::pinax::evm::v2::{AccessListEntry, SetCodeAuthorization, Transaction};
use crate::utils::{bigint_to_string, bytes_to_uint256_string, optional_bigint_to_string, transaction_status_text, transaction_type_text};

// DetailLevel: BASE (all transactions, including failed/reverted ones)
pub fn collect_transaction(tx: &TransactionTrace, id: &BlockIdentity) -> Transaction {
    let receipt = tx.receipt.as_ref();

    Transaction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        block_number: id.block_num,
        index: tx.index,
        hash: tx.hash.clone(),
        from: tx.from.clone(),
        to: tx.to.clone(),
        value: bigint_to_string(&tx.value),
        gas_limit: tx.gas_limit,
        gas_used: tx.gas_used,
        gas_price: optional_bigint_to_string(&tx.gas_price),
        r#type: transaction_type_text(tx.r#type),
        status: transaction_status_text(tx.status),
        nonce: tx.nonce,
        input: tx.input.clone(),
        max_fee_per_gas: optional_bigint_to_string(&tx.max_fee_per_gas),
        max_priority_fee_per_gas: optional_bigint_to_string(&tx.max_priority_fee_per_gas),
        cumulative_gas_used: receipt.map(|r| r.cumulative_gas_used),

        v: tx.v.clone(),
        r: tx.r.clone(),
        s: tx.s.clone(),
        public_key: tx.public_key.clone(),
        return_data: tx.return_data.clone(),
        begin_ordinal: tx.begin_ordinal,
        end_ordinal: tx.end_ordinal,
        blob_gas: tx.blob_gas,
        blob_gas_fee_cap: optional_bigint_to_string(&tx.blob_gas_fee_cap),
        blob_hashes: tx.blob_hashes.clone(),
        receipt_state_root: receipt.map(|r| r.state_root.clone()).unwrap_or_default(),
        receipt_logs_bloom: receipt.map(|r| r.logs_bloom.clone()).unwrap_or_default(),
        receipt_blob_gas_used: receipt.and_then(|r| r.blob_gas_used),
        receipt_blob_gas_price: receipt.and_then(|r| optional_bigint_to_string(&r.blob_gas_price)),
        num_logs: receipt.map(|r| r.logs.len()).unwrap_or_default() as u32,
        num_calls: tx.calls.len() as u32,
    }
}

// EIP-2930 access lists
pub fn collect_access_lists(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<AccessListEntry> {
    tx.access_list
        .iter()
        .enumerate()
        .map(|(index, entry)| AccessListEntry {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            tx_index: tx.index,
            index: index as u32,
            address: entry.address.clone(),
            storage_keys: entry.storage_keys.clone(),
        })
        .collect()
}

// EIP-7702 set-code authorizations
pub fn collect_set_code_authorizations(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<SetCodeAuthorization> {
    tx.set_code_authorizations
        .iter()
        .enumerate()
        .map(|(index, auth)| SetCodeAuthorization {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            tx_index: tx.index,
            index: index as u32,
            discarded: auth.discarded,
            chain_id: bytes_to_uint256_string(&auth.chain_id),
            address: auth.address.clone(),
            nonce: auth.nonce,
            v: auth.v,
            r: auth.r.clone(),
            s: auth.s.clone(),
            authority: auth.authority.clone(),
        })
        .collect()
}
