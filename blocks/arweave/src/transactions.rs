use common::structs::BlockTimestamp;
use substreams::Hex;

use crate::pb::{
    pinax::arweave::v1::{Transaction as TransactionOutput, TransactionTag},
    sf::arweave::r#type::v1::Transaction,
};

pub fn collect_transaction(transaction: &Transaction, timestamp: &BlockTimestamp, index: usize) -> (TransactionOutput, Vec<TransactionTag>) {
    let tx_id = Hex::encode(&transaction.id);

    let tx = TransactionOutput {
        block_time: timestamp.time.to_string(),
        block_height: timestamp.number,
        block_date: timestamp.date.clone(),
        block_indep_hash: timestamp.hash.clone(),
        format: transaction.format,
        id: tx_id.clone(),
        index: index as u32,
        last_tx: Hex::encode(&transaction.last_tx),
        owner: Hex::encode(&transaction.owner),
        target: Hex::encode(&transaction.target),
        quantity: transaction.quantity.as_ref().unwrap().bytes.clone(),
        data: transaction.data.clone(),
        data_size: transaction.data_size.as_ref().unwrap().bytes.clone(),
        data_root: Hex::encode(&transaction.data_root),
        signature: Hex::encode(&transaction.signature),
        reward: transaction.reward.as_ref().unwrap().bytes.clone(),
    };

    let mut tags = Vec::new();

    for (index, tag) in transaction.tags.iter().enumerate() {
        tags.push(TransactionTag {
            block_time: timestamp.time.to_string(),
            block_height: timestamp.number,
            block_date: timestamp.date.clone(),
            block_indep_hash: timestamp.hash.clone(),
            tx_id: tx_id.clone(),
            index: index.to_string(),
            name: String::from_utf8(tag.name.clone()).unwrap(),
            value: String::from_utf8(tag.value.clone()).unwrap(),
        });
    }

    // TODO: merge tags into Transaction when supported by substreams-sink-files
    (tx, tags)
}
