use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams_ethereum::pb::eth::v2::{Block, CodeChange as CodeChangeSource, TransactionTrace};

use crate::pb::pinax::evm::v1::CodeChange;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L744
// DetailLevel: EXTENDED
pub fn collect_code_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<CodeChange> {
    let mut code_changes: Vec<CodeChange> = vec![];

    // Collect code changes from system calls
    for call in &block.system_calls {
        for code_change in &call.code_changes {
            code_changes.push(parse_creation_trace(code_change, &TransactionTrace::default(), timestamp));
        }
    }

    // Collect code changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for code_change in &call.code_changes {
                code_changes.push(parse_creation_trace(code_change, transaction, timestamp));
            }
        }
    }

    code_changes
}

pub fn parse_creation_trace(code_change: &CodeChangeSource, transaction: &TransactionTrace, timestamp: &BlockTimestamp) -> CodeChange {
    let factory = if transaction.to == code_change.address { "".to_string() } else { bytes_to_hex(&transaction.to) };
    CodeChange {
        // block
        block_time: timestamp.time.to_string(),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: bytes_to_hex(&transaction.hash),

        // creation trace
        from: bytes_to_hex(&transaction.from),
        address: bytes_to_hex(&code_change.address),
        factory: factory.to_string(),
        code: bytes_to_hex(&code_change.new_code),
    }
}
