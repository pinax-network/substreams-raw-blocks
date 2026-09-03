use common::structs::BlockIdentity;
use substreams_antelope::pb::TransactionTrace;

use crate::pb::pinax::antelope::v2::Action;
use crate::utils::{auth_sequence_json, exception_json, format_authorization, non_empty, non_empty_bytes, ram_deltas_json, transaction_status_text};

pub fn collect_actions(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<Action> {
    let tx_status = transaction_status_text(tx.receipt.as_ref().map(|r| r.status).unwrap_or_default());

    tx.action_traces
        .iter()
        .map(|trace| {
            let action = trace.action.clone().unwrap_or_default();
            let receipt = trace.receipt.clone().unwrap_or_default();

            Action {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                tx_hash: tx.id.clone(),
                action_ordinal: trace.action_ordinal,
                creator_action_ordinal: trace.creator_action_ordinal,
                closest_unnotified_ancestor_action_ordinal: trace.closest_unnotified_ancestor_action_ordinal,
                execution_index: trace.execution_index,
                receiver: trace.receiver.clone(),
                account: action.account.clone(),
                name: action.name.clone(),
                authorization: format_authorization(&action.authorization),
                json_data: non_empty(&action.json_data),
                raw_data: non_empty_bytes(&action.raw_data),
                context_free: trace.context_free,
                elapsed: trace.elapsed,
                console: non_empty(&trace.console),
                transaction_id: trace.transaction_id.clone(),
                trace_block_num: trace.block_num,
                producer_block_id: trace.producer_block_id.clone(),
                block_time: trace.block_time.clone(),
                raw_return_value: non_empty_bytes(&trace.raw_return_value),
                json_return_value: non_empty(&trace.json_return_value),
                exception: exception_json(&trace.exception),
                error_code: trace.error_code,
                receipt_receiver: receipt.receiver.clone(),
                receipt_digest: receipt.digest.clone(),
                receipt_global_sequence: receipt.global_sequence,
                receipt_auth_sequence: auth_sequence_json(&receipt.auth_sequence),
                receipt_recv_sequence: receipt.recv_sequence,
                receipt_code_sequence: receipt.code_sequence,
                receipt_abi_sequence: receipt.abi_sequence,

                tx_index: tx.index,
                tx_status: tx_status.clone(),
                account_ram_deltas: ram_deltas_json(&trace.account_ram_deltas),
                filtering_matched: trace.filtering_matched,
            }
        })
        .collect()
}
