use common::structs::BlockTimestamp;

use substreams::Hex;
use substreams_antelope::pb::TransactionTrace;
use substreams_antelope::Block;

use crate::pb::antelope::Action as RawAction;
use crate::transactions::is_transaction_success;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn collect_actions(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawAction> {
    let header = block.header.clone().unwrap_or_default();
    let mut actions = Vec::new();

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for trace in transaction.action_traces.iter() {
            let action = trace.action.clone().unwrap_or_default();
            let receipt = trace.receipt.clone().unwrap_or_default();

            actions.push(RawAction {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: tx_hash.clone(),
                tx_success,
                abi_sequence: receipt.abi_sequence,
                code_sequence: receipt.code_sequence,
                digest: receipt.digest,
                global_sequence: receipt.global_sequence,
                receipt_receiver: receipt.receiver,
                recv_sequence: receipt.recv_sequence,
                account: action.account,
                name: action.name,
                json_data: action.json_data,
                raw_data: Hex::encode(&action.raw_data),
                index: trace.execution_index,
                action_ordinal: trace.action_ordinal,
                receiver: trace.receiver.clone(),
                context_free: trace.context_free,
                elapsed: trace.elapsed,
                console: trace.console.clone(),
                raw_return_value: Hex::encode(&trace.raw_return_value),
                json_return_value: trace.json_return_value.clone(),
                creator_action_ordinal: trace.creator_action_ordinal,
                closest_unnotified_ancestor_action_ordinal: trace.closest_unnotified_ancestor_action_ordinal,
                action_mroot: Hex::encode(&header.action_mroot),
            });
        }
    }

    actions
}

pub fn collect_tx_actions(block: &Block, transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<RawAction> {
    let header = block.header.clone().unwrap_or_default();
    let mut actions: Vec<RawAction> = Vec::new();

    for trace in transaction.action_traces.iter() {
        let action = trace.action.clone().unwrap_or_default();
        let receipt = trace.receipt.clone().unwrap_or_default();

        actions.push(RawAction {
            block_time: Some(timestamp.time.clone()),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),
            tx_hash: transaction.id.clone(),
            tx_success,
            abi_sequence: receipt.abi_sequence,
            code_sequence: receipt.code_sequence,
            digest: receipt.digest,
            global_sequence: receipt.global_sequence,
            receipt_receiver: receipt.receiver,
            recv_sequence: receipt.recv_sequence,
            account: action.account,
            name: action.name,
            json_data: action.json_data,
            raw_data: Hex::encode(&action.raw_data),
            index: trace.execution_index,
            action_ordinal: trace.action_ordinal,
            receiver: trace.receiver.clone(),
            context_free: trace.context_free,
            elapsed: trace.elapsed,
            console: trace.console.clone(),
            raw_return_value: Hex::encode(&trace.raw_return_value),
            json_return_value: trace.json_return_value.clone(),
            creator_action_ordinal: trace.creator_action_ordinal,
            closest_unnotified_ancestor_action_ordinal: trace.closest_unnotified_ancestor_action_ordinal,
            action_mroot: Hex::encode(&header.action_mroot),
        });
    }

    actions
}
