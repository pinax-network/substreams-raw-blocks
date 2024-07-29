use common::sinks::insert_timestamp;
use common::utils::bytes_to_hex;
use common::{keys::balance_changes_keys, utils::optional_bigint_to_string};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::Block;

use crate::traces::insert_traces;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L658
pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    // transactions
    for transaction in block.transaction_traces.iter() {
        insert_traces(tables, clock, transaction);
        // traces
        for trace in transaction.calls.iter() {}
        // let address = bytes_to_hex(balance_change.address);
        // let new_value = bytes_to_hex(balance_change.new_value.unwrap_or_default().bytes);
        // let old_value = bytes_to_hex(balance_change.old_value.unwrap_or_default().bytes);
        // let ordinal = balance_change.ordinal.to_string();
        // let reason = balance_change.reason.to_string();
        // let keys = balance_changes_keys(&clock, &ordinal);
        // let row = tables
        //     .push_change_composite("balance_changes", keys, 0, table_change::Operation::Create)
        //     .change("address", ("", address.as_str()))
        //     .change("new_value", ("", new_value.as_str()))
        //     .change("old_value", ("", old_value.as_str()))
        //     .change("ordinal", ("", ordinal.as_str()))
        //     .change("reason", ("", reason.as_str()));

        insert_timestamp(row, clock, false);
    }
}
