use common::blocks::insert_timestamp;
use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{Block, NonceChange};

use crate::keys::block_ordinal_keys;
use crate::pb::evm::NonceChange as RawNonceChange;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L726C9-L726C20
// DetailLevel: EXTENDED
pub fn insert_nonce_change(tables: &mut DatabaseChanges, clock: &Clock, nonce_change: &NonceChange) {
    let address = bytes_to_hex(&nonce_change.address);
    let old_value = nonce_change.old_value;
    let new_value = nonce_change.new_value;
    let ordinal = nonce_change.ordinal;

    let keys = block_ordinal_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("nonce_changes", keys, 0, table_change::Operation::Create)
        .change("address", ("", address.as_str()))
        .change("old_value", ("", old_value.to_string().as_str()))
        .change("new_value", ("", new_value.to_string().as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));

    insert_timestamp(row, clock, false, true);
}

pub fn collect_nonce_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawNonceChange> {
    let mut nonce_changes: Vec<RawNonceChange> = vec![];

    // Collect nonce changes from system calls
    for call in &block.system_calls {
        for nonce_change in &call.nonce_changes {
            nonce_changes.push(RawNonceChange {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                address: bytes_to_hex(&nonce_change.address),
                old_value: nonce_change.old_value,
                new_value: nonce_change.new_value,
                ordinal: nonce_change.ordinal,
            });
        }
    }

    // Collect nonce changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for nonce_change in &call.nonce_changes {
                nonce_changes.push(RawNonceChange {
                    block_time: Some(timestamp.time),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    address: bytes_to_hex(&nonce_change.address),
                    old_value: nonce_change.old_value,
                    new_value: nonce_change.new_value,
                    ordinal: nonce_change.ordinal,
                });
            }
        }
    }

    nonce_changes
}
