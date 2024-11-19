use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::evm::CodeChange;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L744
// DetailLevel: EXTENDED
pub fn collect_code_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<CodeChange> {
    let mut code_changes: Vec<CodeChange> = vec![];

    // Collect code changes from system calls
    for call in &block.system_calls {
        for code_change in &call.code_changes {
            code_changes.push(CodeChange {
                // block
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),

                // transaction
                tx_hash: Some(String::new()),

                // code changes
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
                code_changes.push(CodeChange {
                    // block
                    block_time: Some(timestamp.time),
                    block_number: timestamp.number,
                    block_hash: timestamp.hash.clone(),
                    block_date: timestamp.date.clone(),

                    // transaction
                    tx_hash: Some(bytes_to_hex(&transaction.hash)),

                    // code changes
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
