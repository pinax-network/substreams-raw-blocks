use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::pb::TxResults;

use crate::pb::pinax::cosmos::v1::Transaction;

pub fn collect_transaction(tx_result: &TxResults, tx_hash: &str, timestamp: &BlockTimestamp, index: usize) -> Transaction {
    Transaction {
        block_time: Some(timestamp.time),
        block_number: timestamp.number,
        block_date: timestamp.date.clone(),
        block_hash: timestamp.hash.clone(),
        index: index as u32,
        hash: tx_hash.to_string(),
        code: tx_result.code,
        data: Hex::encode(&tx_result.data),
        log: tx_result.log.clone(),
        info: tx_result.info.clone(),
        gas_wanted: tx_result.gas_wanted,
        gas_used: tx_result.gas_used,
        codespace: tx_result.codespace.clone(),
    }
}
