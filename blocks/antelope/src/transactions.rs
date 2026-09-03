use common::structs::BlockIdentity;
use substreams_antelope::pb::{TransactionReceipt, TransactionTrace};

use crate::pb::pinax::antelope::v2::Transaction;
use crate::utils::{exception_json, transaction_status_text};

// All transaction traces, including failed/expired ones (status != EXECUTED).
pub fn collect_transaction(tx: &TransactionTrace, receipt: Option<&TransactionReceipt>, id: &BlockIdentity) -> Transaction {
    let header = tx.receipt.clone().unwrap_or_default();
    let packed = receipt.and_then(|r| r.packed_transaction.as_ref());

    Transaction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        tx_hash: tx.id.clone(),
        index: tx.index,
        status: transaction_status_text(header.status),
        cpu_usage_us: header.cpu_usage_micro_seconds,
        net_usage: tx.net_usage,
        elapsed: tx.elapsed,

        scheduled: tx.scheduled,
        net_usage_words: header.net_usage_words,
        producer_block_id: tx.producer_block_id.clone(),
        error_code: tx.error_code,
        exception: exception_json(&tx.exception),
        creator_action_indexes: tx.creation_tree.iter().map(|n| n.creator_action_index).collect(),
        execution_action_indexes: tx.creation_tree.iter().map(|n| n.execution_action_index).collect(),
        signatures: packed.map(|p| p.signatures.clone()).unwrap_or_default(),
        compression: packed.map(|p| p.compression),
        num_actions: tx.action_traces.len() as u32,
        num_db_ops: tx.db_ops.len() as u32,
    }
}
