use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;

use crate::pb::antelope::Authorization;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L616
pub fn collect_tx_authorizations(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<Authorization> {
    let mut authorizations = Vec::new();

    for action_trace in transaction.action_traces.iter() {
        for (index, authorization) in action_trace.action.as_ref().unwrap().authorization.iter().enumerate() {
            authorizations.push(Authorization {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: transaction.id.clone(),
                tx_success,
                action_index: action_trace.execution_index,
                index: index as u32,
                actor: authorization.actor.clone(),
                permission: authorization.permission.clone(),
            });
        }
    }

    authorizations
}
