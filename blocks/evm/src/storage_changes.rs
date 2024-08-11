use common::blocks::insert_timestamp;
use common::keys::block_ordinal_keys;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{Call, StorageChange, TransactionTrace};

use crate::traces::insert_trace_metadata;
use crate::transactions::insert_transaction_metadata;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
// DetailLevel: EXTENDED
pub fn insert_storage_change(tables: &mut DatabaseChanges, clock: &Clock, storage_change: &StorageChange, transaction: &TransactionTrace, trace: &Call) {
    let address = bytes_to_hex(storage_change.address.clone());
    let key = bytes_to_hex(storage_change.key.clone());
    let new_value = bytes_to_hex(storage_change.new_value.clone());
    let old_value = bytes_to_hex(storage_change.old_value.clone());
    let ordinal = storage_change.ordinal;

    let keys = block_ordinal_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("storage_changes", keys, 0, table_change::Operation::Create)
        .change("address", ("", address.as_str()))
        .change("key", ("", key.as_str()))
        .change("new_value", ("", new_value.as_str()))
        .change("old_value", ("", old_value.as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));

    insert_timestamp(row, clock, false);
    insert_transaction_metadata(row, transaction, false);
    insert_trace_metadata(row, trace);
}
