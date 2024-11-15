use common::{structs::BlockTimestamp, utils::bytes_to_hex};
use substreams_cosmos::Block;

use crate::{pb::cosmos::rawblocks::Transaction as RawTransaction, utils::compute_tx_hash};

pub fn collect_transactions(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawTransaction> {
    let mut vec: Vec<RawTransaction> = vec![];

    for (i, transaction) in block.tx_results.iter().enumerate() {
        let tx_as_bytes = block.txs.get(i).unwrap();
        let tx_hash = compute_tx_hash(tx_as_bytes);
        let code = transaction.code;
        let data = bytes_to_hex(&transaction.data);

        vec.push(RawTransaction {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: bytes_to_hex(&block.hash),
            index: i as u32,
            hash: tx_hash,
            code,
            data,
            log: transaction.log.clone(),
            info: transaction.info.clone(),
            gas_wanted: transaction.gas_wanted,
            gas_used: transaction.gas_used,
            codespace: transaction.codespace.clone(),
        });
    }

    vec
}
