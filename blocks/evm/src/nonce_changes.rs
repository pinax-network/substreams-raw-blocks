use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams_ethereum::pb::eth::v2::{Block, NonceChange, TransactionTrace};

use crate::pb::evm::NonceChange as NonceChangeEvent;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L726C9-L726C20
// DetailLevel: EXTENDED
pub fn collect_nonce_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<NonceChangeEvent> {
    let mut nonce_changes: Vec<NonceChangeEvent> = vec![];

    // Collect nonce changes from system calls
    for call in &block.system_calls {
        for nonce_change in &call.nonce_changes {
            nonce_changes.push(parse_nonce_change(nonce_change, &TransactionTrace::default(), timestamp));
        }
    }

    // Collect nonce changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for nonce_change in &call.nonce_changes {
                nonce_changes.push(parse_nonce_change(nonce_change, transaction, timestamp));
            }
        }
    }

    nonce_changes
}

pub fn parse_nonce_change(nonce_change: &NonceChange, transaction: &TransactionTrace, timestamp: &BlockTimestamp) -> NonceChangeEvent {
    NonceChangeEvent {
        // block
        block_time: Some(timestamp.time),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: Some(bytes_to_hex(&transaction.hash)),

        // nonce changes
        address: bytes_to_hex(&nonce_change.address),
        old_value: nonce_change.old_value,
        new_value: nonce_change.new_value,
        ordinal: nonce_change.ordinal,
    }
}
