use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;

use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::evm::StorageChange as RawStorageChange;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
// DetailLevel: EXTENDED
pub fn collect_storage_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawStorageChange> {
    let mut storage_changes: Vec<RawStorageChange> = vec![];

    // Collect storage changes from system calls
    for call in &block.system_calls {
        for storage_change in &call.storage_changes {
            storage_changes.push(RawStorageChange {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                address: bytes_to_hex(&storage_change.address),
                key: bytes_to_hex(&storage_change.key),
                new_value: bytes_to_hex(&storage_change.new_value),
                old_value: bytes_to_hex(&storage_change.old_value),
                ordinal: storage_change.ordinal,
            });
        }
    }

    // Collect storage changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for storage_change in &call.storage_changes {
                storage_changes.push(RawStorageChange {
                    block_time: Some(timestamp.time),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    address: bytes_to_hex(&storage_change.address),
                    key: bytes_to_hex(&storage_change.key),
                    new_value: bytes_to_hex(&storage_change.new_value),
                    old_value: bytes_to_hex(&storage_change.old_value),
                    ordinal: storage_change.ordinal,
                });
            }
        }
    }

    storage_changes
}
