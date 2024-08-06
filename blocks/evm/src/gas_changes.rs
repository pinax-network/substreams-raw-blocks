use common::blocks::insert_timestamp;
use common::keys::balance_changes_keys;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{Call, GasChange, TransactionTrace};

use crate::traces::insert_trace_metadata;
use crate::transactions::insert_transaction_metadata;

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
pub fn insert_gas_change(tables: &mut DatabaseChanges, clock: &Clock, gas_change: &GasChange, transaction: &TransactionTrace, trace: &Call) {
    let old_value = gas_change.old_value;
    let new_value = gas_change.new_value;
    let delta_value = new_value as i128 - old_value as i128;

    let reason = gas_change_reason_to_string(gas_change.reason);
    let reason_code = gas_change.reason;
    let ordinal = gas_change.ordinal;

    let keys = balance_changes_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("gas_changes", keys, 0, table_change::Operation::Create)
        .change("reason", ("", reason.as_str()))
        .change("reason_code", ("", reason_code.to_string().as_str()))
        .change("old_value", ("", old_value.to_string().as_str()))
        .change("new_value", ("", new_value.to_string().as_str()))
        .change("delta_value", ("", delta_value.to_string().as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));

    insert_timestamp(row, clock, false);
    insert_transaction_metadata(row, transaction);
    insert_trace_metadata(row, trace);
}
