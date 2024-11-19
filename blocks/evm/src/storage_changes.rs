use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;

use substreams_ethereum::pb::eth::v2::{Block, StorageChange, TransactionTrace};

use crate::pb::evm::StorageChange as StorageChangeEvent;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
// DetailLevel: EXTENDED
pub fn collect_storage_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<StorageChangeEvent> {
    let mut storage_changes: Vec<StorageChangeEvent> = vec![];

    // Collect storage changes from system calls
    for call in &block.system_calls {
        for storage_change in &call.storage_changes {
            storage_changes.push(parse_storage_change(storage_change, &TransactionTrace::default(), timestamp));
        }
    }

    // Collect storage changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for storage_change in &call.storage_changes {
                storage_changes.push(parse_storage_change(storage_change, transaction, timestamp));
            }
        }
    }

    storage_changes
}

pub fn parse_storage_change(storage_change: &StorageChange, transaction: &TransactionTrace, timestamp: &BlockTimestamp) -> StorageChangeEvent {
    StorageChangeEvent {
        // block
        block_time: Some(timestamp.time),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: Some(bytes_to_hex(&transaction.hash)),

        // storage changes
        address: bytes_to_hex(&storage_change.address),
        key: bytes_to_hex(&storage_change.key),
        new_value: bytes_to_hex(&storage_change.new_value),
        old_value: bytes_to_hex(&storage_change.old_value),
        ordinal: storage_change.ordinal,
    }
}
