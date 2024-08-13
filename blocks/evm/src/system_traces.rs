use common::{
    blocks::insert_timestamp,
    keys::system_traces_keys,
};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::Call;

use crate::traces::insert_trace_values;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L546
// DetailLevel: EXTENDED
pub fn insert_system_trace(tables: &mut DatabaseChanges, clock: &Clock, system_call: &Call) {
    let keys = system_traces_keys(&clock, &system_call.index);
    let row = tables.push_change_composite("system_traces", keys, 0, table_change::Operation::Create);
    insert_trace_values(row, system_call);
    insert_timestamp(row, clock, false);

    // TO-DO: add additional tables? (e.g. logs, balance_changes, storage_changes, code_changes, account_creations, nonce_changes, gas_changes)
    // would require to drop `transaction` from those tables
}