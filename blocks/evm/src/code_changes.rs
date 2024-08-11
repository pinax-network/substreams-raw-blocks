use common::blocks::insert_timestamp;
use common::keys::block_ordinal_keys;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{Call, CodeChange, TransactionTrace};

use crate::traces::insert_trace_metadata;
use crate::transactions::insert_transaction_metadata;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L744
// DetailLevel: EXTENDED
pub fn insert_code_change(tables: &mut DatabaseChanges, clock: &Clock, code_change: &CodeChange, transaction: &TransactionTrace, trace: &Call) {
    let address = bytes_to_hex(code_change.address.clone());
    let old_hash = bytes_to_hex(code_change.old_hash.clone());
    let old_code = bytes_to_hex(code_change.old_code.clone());
    let new_hash = bytes_to_hex(code_change.new_hash.clone());
    let new_code = bytes_to_hex(code_change.new_code.clone());
    let ordinal = code_change.ordinal;

    let keys = block_ordinal_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("code_changes", keys, 0, table_change::Operation::Create)
        .change("address", ("", address.as_str()))
        .change("old_hash", ("", old_hash.as_str()))
        .change("old_code", ("", old_code.as_str()))
        .change("new_hash", ("", new_hash.as_str()))
        .change("new_code", ("", new_code.as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));

    insert_timestamp(row, clock, false);
    insert_transaction_metadata(row, transaction, false);
    insert_trace_metadata(row, trace);
}
