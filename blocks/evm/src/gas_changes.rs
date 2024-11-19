use common::{structs::BlockTimestamp, utils::bytes_to_hex};
use substreams_ethereum::pb::eth::v2::{Block, GasChange, TransactionTrace};

use crate::pb::evm::GasChange as GasChangeEvent;

pub fn gas_change_reason_to_string(reason: i32) -> String {
    match reason {
        0 => "Unknown".to_string(),
        1 => "Call".to_string(),
        2 => "CallCode".to_string(),
        3 => "CallDataCopy".to_string(),
        4 => "CodeCopy".to_string(),
        5 => "CodeStorage".to_string(),
        6 => "ContractCreation".to_string(),
        7 => "ContractCreation2".to_string(),
        8 => "DelegateCall".to_string(),
        9 => "EventLog".to_string(),
        10 => "ExtCodeCopy".to_string(),
        11 => "FailedExecution".to_string(),
        12 => "IntrinsicGas".to_string(),
        13 => "PrecompiledContract".to_string(),
        14 => "RefundAfterExecution".to_string(),
        15 => "Return".to_string(),
        16 => "ReturnDataCopy".to_string(),
        17 => "Revert".to_string(),
        18 => "SelfDestruct".to_string(),
        19 => "StaticCall".to_string(),
        20 => "StateColdAccess".to_string(),
        21 => "TxInitialBalance".to_string(),
        22 => "TxRefunds".to_string(),
        23 => "TxLeftOverReturned".to_string(),
        24 => "CallInitialBalance".to_string(),
        25 => "CallLeftOverReturned".to_string(),
        _ => "Unknown".to_string(),
    }
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L726C9-L726C20
// DetailLevel: EXTENDED
pub fn collect_gas_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<GasChangeEvent> {
    let mut gas_changes: Vec<GasChangeEvent> = vec![];

    // Collect gas changes from system calls
    for call in &block.system_calls {
        for gas_change in &call.gas_changes {
            gas_changes.push(parse_gas_changes(gas_change, &TransactionTrace::default(), timestamp));
        }
    }

    // Collect gas changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for gas_change in &call.gas_changes {
                gas_changes.push(parse_gas_changes(gas_change, transaction, timestamp));
            }
        }
    }

    gas_changes
}

pub fn parse_gas_changes(gas_change: &GasChange, transaction: &TransactionTrace, timestamp: &BlockTimestamp) -> GasChangeEvent {
    GasChangeEvent {
        // block
        block_time: Some(timestamp.time),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: Some(bytes_to_hex(&transaction.hash)),

        // gas changes
        old_value: gas_change.old_value,
        new_value: gas_change.new_value,
        reason: gas_change_reason_to_string(gas_change.reason),
        reason_code: gas_change.reason as u32,
        ordinal: gas_change.ordinal,
    }
}
