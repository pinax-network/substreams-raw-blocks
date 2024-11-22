use common::structs::BlockTimestamp;
use substreams_bitcoin::pb::btc::v1::Transaction;

use crate::pb::bitcoin::Input;

pub fn collect_transaction_inputs(transaction: &Transaction, timestamp: &BlockTimestamp) -> Vec<Input> {
    let mut inputs = Vec::new();

    for (index, input) in transaction.vin.iter().enumerate() {
        let script_sig = input.script_sig.as_ref();

        inputs.push(Input {
            block_time: Some(timestamp.time),
            block_date: timestamp.date.clone(),
            block_height: timestamp.number as u32,
            block_hash: timestamp.hash.clone(),
            tx_id: transaction.txid.clone(),
            index: index as u32,
            spent_block_height: 0, // TODO: Need to look up from previous tx
            spent_tx_id: input.txid.clone(),
            spent_output_number: input.vout as u64,
            value: 0.0,             // TODO: Need to look up from previous tx
            address: String::new(), // TODO: Need to look up from previous tx
            r#type: String::new(),  // TODO: Need to look up from previous tx
            coinbase: input.coinbase.clone(),
            is_coinbase: !input.coinbase.is_empty(),
            script_asm: script_sig.map(|s| s.asm.clone()).unwrap_or_default(),
            script_hex: script_sig.map(|s| s.hex.clone()).unwrap_or_default(),
            script_desc: String::new(), // TODO: Need to investigate how Dune generates this
            script_signature_asm: script_sig.map(|s| s.asm.clone()).unwrap_or_default(),
            script_signature_hex: script_sig.map(|s| s.hex.clone()).unwrap_or_default(),
            sequence: input.sequence as i64,
            witness_data: input.txinwitness.clone(),
        });
    }

    inputs
}
