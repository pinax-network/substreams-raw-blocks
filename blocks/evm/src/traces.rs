use common::{
    structs::BlockTimestamp,
    utils::{bytes_to_hex, optional_bigint_to_string},
};
use substreams_ethereum::pb::eth::v2::{Block, Call, TransactionTrace};

use crate::{
    pb::pinax::evm::v1::Trace,
    transactions::{is_transaction_success, transaction_status_to_string},
};

pub fn call_types_to_string(call_type: i32) -> String {
    match call_type {
        0 => "Unspecified".to_string(),
        1 => "Call".to_string(),
        2 => "Callcode".to_string(),
        3 => "Delegate".to_string(),
        4 => "Static".to_string(),
        5 => "Create".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn collect_traces(block: &Block, timestamp: &BlockTimestamp) -> Vec<Trace> {
    let mut traces: Vec<Trace> = vec![];

    // https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L121-L124
    // DetailLevel: EXTENDED
    // System calls are introduced in Cancun, along with blobs. They are executed outside of transactions but affect the state.
    for call in &block.system_calls {
        traces.push(parse_traces(call, &TransactionTrace::default(), timestamp));
    }

    // Collect transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            traces.push(parse_traces(call, transaction, timestamp));
        }
    }
    traces
}

pub fn parse_traces(call: &Call, transaction: &TransactionTrace, timestamp: &BlockTimestamp) -> Trace {
    let is_system_call = transaction.hash.is_empty();

    Trace {
        // block
        block_time: Some(timestamp.time),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: bytes_to_hex(&transaction.hash),
        tx_index: transaction.index,
        tx_status: transaction_status_to_string(if is_system_call { 1 } else { transaction.status }),
        tx_status_code: if is_system_call { 1 } else { transaction.status } as u32,
        tx_success: is_transaction_success(if is_system_call { 1 } else { transaction.status }),

        // trace
        from: bytes_to_hex(&call.caller),
        to: bytes_to_hex(&call.address),
        index: call.index,
        parent_index: call.parent_index,
        depth: call.depth,
        caller: bytes_to_hex(&call.caller),
        call_type: call_types_to_string(call.call_type),
        call_type_code: call.call_type as u32,
        address: bytes_to_hex(&call.address),
        value: optional_bigint_to_string(&call.value, "0"),
        gas_limit: call.gas_limit,
        gas_consumed: call.gas_consumed,
        input: bytes_to_hex(&call.input),
        return_data: bytes_to_hex(&call.return_data),
        failure_reason: call.failure_reason.clone(),
        begin_ordinal: call.begin_ordinal,
        end_ordinal: call.end_ordinal,
        executed_code: call.executed_code,
        state_reverted: call.state_reverted,
        status_failed: call.status_failed,
        status_reverted: call.status_reverted,
        suicide: call.suicide,
    }
}
