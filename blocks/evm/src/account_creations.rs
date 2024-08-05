use common::blocks::insert_timestamp;
use common::keys::balance_changes_keys;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{AccountCreation, Call, TransactionTrace};

use crate::traces::insert_trace_metadata;
use crate::transactions::insert_transaction_metadata;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L736
pub fn insert_account_creation(tables: &mut DatabaseChanges, clock: &Clock, account_creation: &AccountCreation, transaction: &TransactionTrace, trace: &Call) {
    let account = bytes_to_hex(account_creation.account.clone());
    let ordinal = account_creation.ordinal;

    let keys = balance_changes_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("account_creations", keys, 0, table_change::Operation::Create)
        .change("account", ("", account.as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));

    insert_timestamp(row, clock, false);
    insert_transaction_metadata(row, transaction);
    insert_trace_metadata(row, trace);
}
