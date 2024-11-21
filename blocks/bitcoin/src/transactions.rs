use std::{collections::HashMap, time};

use common::structs::BlockTimestamp;
use substreams_bitcoin::pb::btc::v1::{Transaction, Vout};

use crate::pb::bitcoin::Transaction as OutputTransaction;

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

// fn calculate_transaction_values(tx: &Transaction, utxo_map: &HashMap<String, Vec<Vout>>) -> (f64, f64) {
//     // Calculate input value
//     let mut input_value = 0.0;
//     for vin in &tx.vin {
//         if let Some(previous_outputs) = utxo_map.get(&vin.txid) {
//             if let Some(output) = previous_outputs.get(vin.vout as usize) {
//                 input_value += output.value;
//             }
//         }
//     }

//     let input_value = tx.vin.iter().map(|vin| vin.)
//     // Calculate output value
//     let output_value: f64 = tx.vout.iter().map(|vout| vout.value).sum();

//     (input_value, output_value)
// }
