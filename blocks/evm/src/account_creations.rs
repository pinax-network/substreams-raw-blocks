use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;

use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::evm::AccountCreation;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L736
// DetailLevel: EXTENDED
pub fn collect_account_creations(block: &Block, timestamp: &BlockTimestamp) -> Vec<AccountCreation> {
    let mut account_creations: Vec<AccountCreation> = vec![];

    // Collect account creations from system calls
    for call in &block.system_calls {
        for account_creation in &call.account_creations {
            account_creations.push(AccountCreation {
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
                account_creations.push(AccountCreation {
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
