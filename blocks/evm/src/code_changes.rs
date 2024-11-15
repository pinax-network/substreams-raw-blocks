use common::blocks::insert_timestamp;
use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges, TableChange};
use substreams_ethereum::pb::eth::v2::{Block, CodeChange};

use crate::keys::block_ordinal_keys;
use crate::pb::evm::CodeChange as RawCodeChange;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L744
// DetailLevel: EXTENDED
pub fn insert_code_change_rows(row: &mut TableChange, code_change: &CodeChange) {
    let address = bytes_to_hex(&code_change.address);
    let old_hash = bytes_to_hex(&code_change.old_hash);
    let old_code = bytes_to_hex(&code_change.old_code);
    let new_hash = bytes_to_hex(&code_change.new_hash);
    let new_code = bytes_to_hex(&code_change.new_code);
    let ordinal = code_change.ordinal;

    row.change("address", ("", address.as_str()))
        .change("old_hash", ("", old_hash.as_str()))
        .change("old_code", ("", old_code.as_str()))
        .change("new_hash", ("", new_hash.as_str()))
        .change("new_code", ("", new_code.as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));
}

pub fn insert_code_change(tables: &mut DatabaseChanges, clock: &Clock, code_change: &CodeChange) {
    let ordinal: u64 = code_change.ordinal;
    let keys = block_ordinal_keys(&clock, &ordinal);
    let row = tables.push_change_composite("code_changes", keys, 0, table_change::Operation::Create);

    insert_code_change_rows(row, code_change);
    insert_timestamp(row, clock, false, true);
}

pub fn collect_code_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawCodeChange> {
    let mut code_changes: Vec<RawCodeChange> = vec![];

    // Collect code changes from system calls
    for call in &block.system_calls {
        for code_change in &call.code_changes {
            code_changes.push(RawCodeChange {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                address: bytes_to_hex(&code_change.address),
                old_hash: bytes_to_hex(&code_change.old_hash),
                old_code: bytes_to_hex(&code_change.old_code),
                new_hash: bytes_to_hex(&code_change.new_hash),
                new_code: bytes_to_hex(&code_change.new_code),
                ordinal: code_change.ordinal,
            });
        }
    }

    // Collect code changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for code_change in &call.code_changes {
                code_changes.push(RawCodeChange {
                    block_time: Some(timestamp.time),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    address: bytes_to_hex(&code_change.address),
                    old_hash: bytes_to_hex(&code_change.old_hash),
                    old_code: bytes_to_hex(&code_change.old_code),
                    new_hash: bytes_to_hex(&code_change.new_hash),
                    new_code: bytes_to_hex(&code_change.new_code),
                    ordinal: code_change.ordinal,
                });
            }
        }
    }

    code_changes
}
