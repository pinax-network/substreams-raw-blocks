use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;

use substreams_ethereum::pb::eth::v2::{AccountCreation, Block, TransactionTrace};

use crate::pb::pinax::evm::v1::AccountCreation as AccountCreationEvent;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L736
// DetailLevel: EXTENDED
pub fn collect_account_creations(block: &Block, timestamp: &BlockTimestamp) -> Vec<AccountCreationEvent> {
    let mut account_creations: Vec<AccountCreationEvent> = vec![];

    // Collect account creations from system calls
    for call in &block.system_calls {
        for account_creation in &call.account_creations {
            account_creations.push(parse_account_creation(account_creation, &TransactionTrace::default(), timestamp));
        }
    }

    // Collect account creations from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for account_creation in &call.account_creations {
                account_creations.push(parse_account_creation(account_creation, transaction, timestamp));
            }
        }
    }

    account_creations
}

pub fn parse_account_creation(account_creation: &AccountCreation, transaction: &TransactionTrace, timestamp: &BlockTimestamp) -> AccountCreationEvent {
    AccountCreationEvent {
        // block
        block_time: timestamp.time.to_string(),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: Some(bytes_to_hex(&transaction.hash)),

        // account creation
        account: bytes_to_hex(&account_creation.account),
        ordinal: account_creation.ordinal,
    }
}
