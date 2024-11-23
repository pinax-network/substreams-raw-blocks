use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

use crate::pb::pinax::evm::v1::CreationTrace;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L744
// DetailLevel: EXTENDED
pub fn collect_creation_traces(block: &Block, timestamp: &BlockTimestamp) -> Vec<CreationTrace> {
    let mut creation_traces: Vec<CreationTrace> = vec![];

    // Collect code changes from system calls
    for trace in block.transaction_traces.iter() {
        for call in trace.calls.iter() {
            if call.call_type() == CallType::Create {
                for code in call.code_changes.iter() {
                    let factory = if trace.to == code.address { "".to_string() } else { bytes_to_hex(&trace.to) };

                    creation_traces.push(CreationTrace {
                        // block
                        block_time: Some(timestamp.time),
                        block_number: timestamp.number,
                        block_hash: timestamp.hash.clone(),
                        block_date: timestamp.date.clone(),

                        // transaction
                        tx_hash: bytes_to_hex(&trace.hash),

                        // creation trace
                        from: bytes_to_hex(&trace.from),
                        address: bytes_to_hex(&code.address),
                        factory: factory.to_string(),
                        code: bytes_to_hex(&code.new_code),
                    });
                }
            }
        }
    }

    creation_traces
}
