use common::structs::BlockTimestamp;
use substreams_bitcoin::pb::btc::v1::Transaction;

use crate::pb::pinax::bitcoin::v1::Output;

pub fn collect_transaction_outputs(transaction: &Transaction, timestamp: &BlockTimestamp) -> Vec<Output> {
    let mut outputs = Vec::new();

    for (index, output) in transaction.vout.iter().enumerate() {
        let script_pub_key = output.script_pub_key.as_ref().unwrap();

        outputs.push(Output {
            block_time: Some(timestamp.time),
            block_date: timestamp.date.clone(),
            block_height: timestamp.number as u32,
            block_hash: timestamp.hash.clone(),
            tx_id: transaction.txid.clone(),
            index: index as u32,
            value: output.value,
            address: script_pub_key.address.clone(),
            r#type: script_pub_key.r#type.clone(),
            script_asm: script_pub_key.asm.clone(),
            script_hex: script_pub_key.hex.clone(),
        });
    }

    outputs
}
