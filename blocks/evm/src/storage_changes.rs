use common::blocks::insert_timestamp;
use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{Block, StorageChange};

use crate::keys::block_ordinal_keys;
use crate::pb::evm::StorageChange as RawStorageChange;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
// DetailLevel: EXTENDED
pub fn insert_storage_change(tables: &mut DatabaseChanges, clock: &Clock, storage_change: &StorageChange) {
    let address = bytes_to_hex(&storage_change.address);
    let key = bytes_to_hex(&storage_change.key);
    let new_value = bytes_to_hex(&storage_change.new_value);
    let old_value = bytes_to_hex(&storage_change.old_value);
    let ordinal = storage_change.ordinal;

    let keys = block_ordinal_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("storage_changes", keys, 0, table_change::Operation::Create)
        .change("address", ("", address.as_str()))
        .change("key", ("", key.as_str()))
        .change("new_value", ("", new_value.as_str()))
        .change("old_value", ("", old_value.as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));

    insert_timestamp(row, clock, false, true);
}

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
