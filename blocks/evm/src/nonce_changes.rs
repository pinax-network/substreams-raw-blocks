use common::blocks::insert_timestamp;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::NonceChange;

use crate::keys::block_ordinal_keys;

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
