use common::blocks::insert_timestamp;
use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{AccountCreation, Block};

use crate::keys::block_ordinal_keys;
use crate::pb::evm::AccountCreation as RawAccountCreation;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L736
// DetailLevel: EXTENDED
pub fn insert_account_creation(tables: &mut DatabaseChanges, clock: &Clock, account_creation: &AccountCreation) {
    let account = bytes_to_hex(&account_creation.account);
    let ordinal = account_creation.ordinal;

    let keys = block_ordinal_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("account_creations", keys, 0, table_change::Operation::Create)
        .change("account", ("", account.as_str()))
        .change("ordinal", ("", ordinal.to_string().as_str()));

    insert_timestamp(row, clock, false, true);
}

pub fn collect_account_creations(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawAccountCreation> {
    let mut account_creations: Vec<RawAccountCreation> = vec![];

    // Collect account creations from system calls
    for call in &block.system_calls {
        for account_creation in &call.account_creations {
            account_creations.push(RawAccountCreation {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                account: bytes_to_hex(&account_creation.account),
                ordinal: account_creation.ordinal,
            });
        }
    }

    // Collect account creations from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for account_creation in &call.account_creations {
                account_creations.push(RawAccountCreation {
                    block_time: Some(timestamp.time),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),
                    account: bytes_to_hex(&account_creation.account),
                    ordinal: account_creation.ordinal,
                });
            }
        }
    }

    account_creations
}
