use common::structs::BlockTimestamp;
use substreams_bitcoin::pb::btc::v1::Transaction;

use crate::pb::pinax::bitcoin::Transaction as OutputTransaction;

pub fn collect_transaction(transaction: &Transaction, timestamp: &BlockTimestamp, index: u32) -> OutputTransaction {
    OutputTransaction {
        block_time: Some(timestamp.time),
        block_date: timestamp.date.clone(),
        block_height: timestamp.number as u32,
        block_hash: timestamp.hash.clone(),
        index: index as u32,
        id: transaction.txid.clone(),
        lock_time: transaction.locktime,
        size: transaction.size,
        virtual_size: transaction.vsize,
        coinbase: transaction.vin.first().unwrap().coinbase.clone(),
        is_coinbase: index == 0,
        version: transaction.version as i64,
        input_count: transaction.vin.len() as i32,
        output_count: transaction.vout.len() as i32,
        input_tx_ids: transaction.vin.iter().map(|vin| vin.txid.clone()).collect(),
        input_output_indices: transaction.vin.iter().map(|vin| vin.vout as u32).collect(),
        output_values: transaction.vout.iter().map(|vout| vout.value).collect(),
        hex: transaction.hex.clone(),
    }
}
