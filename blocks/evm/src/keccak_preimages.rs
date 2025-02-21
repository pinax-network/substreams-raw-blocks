use std::collections::HashMap;

use common::structs::BlockTimestamp;
use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::pinax::evm::v1::KeccakPreimage;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L647
// DetailLevel: EXTENDED
pub fn collect_keccak_preimages(block: &Block, timestamp: &BlockTimestamp) -> Vec<KeccakPreimage> {
    let mut keccak_preimages_map = HashMap::new();
    let mut keccak_preimages = Vec::new();

    // Collect storage changes from system calls
    for call in &block.system_calls {
        for (hash, preimage) in &call.keccak_preimages {
            keccak_preimages_map.insert(hash, preimage);
        }
    }

    // Collect storage changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for (hash, preimage) in &call.keccak_preimages {
                keccak_preimages_map.insert(hash, preimage);
            }
        }
    }

    for (hash, preimage) in keccak_preimages_map {
        keccak_preimages.push(KeccakPreimage {
            // block
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),

            // keccak preimage
            hash: hash.to_string(),
            preimage: preimage.to_string(),
        });
    }
    keccak_preimages
}
