use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;

use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{Block, Call, StorageChange, TransactionTrace};

use crate::pb::pinax::evm::v1::StorageChange as StorageChangeEvent;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
// DetailLevel: EXTENDED
pub fn collect_storage_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<StorageChangeEvent> {
    let mut storage_changes: Vec<StorageChangeEvent> = vec![];

    // Collect storage changes from system calls
    for call in &block.system_calls {
        for storage_change in &call.storage_changes {
            storage_changes.push(parse_storage_change(storage_change, &TransactionTrace::default(), call, timestamp));
        }
    }

    // Collect storage changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for storage_change in &call.storage_changes {
                storage_changes.push(parse_storage_change(storage_change, transaction, call, timestamp));
            }
        }
    }

    storage_changes
}

pub fn parse_storage_change(storage_change: &StorageChange, transaction: &TransactionTrace, call: &Call, timestamp: &BlockTimestamp) -> StorageChangeEvent {
    StorageChangeEvent {
        // block
        block_time: timestamp.time.to_string(),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: Some(bytes_to_hex(&transaction.hash)),

        // call
        call_index: call.index,
        call_parent_index: call.parent_index,
        call_begin_ordinal: call.begin_ordinal,
        call_end_ordinal: call.end_ordinal,
        call_depth: call.depth,

        // storage changes
        address: bytes_to_hex(&storage_change.address),
        key: bytes_to_hex(&storage_change.key),
        new_value: bytes_to_hex(&storage_change.new_value),
        new_value_number: BigInt::from_unsigned_bytes_be(&storage_change.new_value).to_string(),
        old_value: bytes_to_hex(&storage_change.old_value),
        old_value_number: BigInt::from_unsigned_bytes_be(&storage_change.old_value).to_string(),
        ordinal: storage_change.ordinal,
    }
}
