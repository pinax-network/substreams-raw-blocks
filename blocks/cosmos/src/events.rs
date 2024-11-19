use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::Block;

use crate::{
    pb::cosmos::{BlockEvent as RawBlockEvent, TransactionEvent as RawTransactionEvent},
    utils::{build_attributes_array_string, compute_tx_hash},
};

pub fn collect_tx_events(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawTransactionEvent> {
    let mut vec: Vec<RawTransactionEvent> = vec![];


    for (tx_index, tx) in block.tx_results.iter().enumerate() {
        for (event_index, event) in tx.events.iter().enumerate() {
            let tx_as_bytes = block.txs.get(tx_index).unwrap();
            let tx_hash = compute_tx_hash(tx_as_bytes);

            vec.push(RawTransactionEvent {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_date: timestamp.date.clone(),
                block_hash: Hex::encode(&block.hash),
                index: event_index as u32,
                tx_hash: Hex::encode(&tx_hash),
                r#type: event.r#type.clone(),
                attributes: build_attributes_array_string(&event.attributes),
            });
        }
    }
    vec
}

pub fn collect_block_events(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawBlockEvent> {
    let mut vec: Vec<RawBlockEvent> = vec![];

    for (index, event) in block.events.iter().enumerate() {
        vec.push(RawBlockEvent {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: Hex::encode(&block.hash),
            index: index as u32,
            r#type: event.r#type.clone(),
            attributes: build_attributes_array_string(&event.attributes),
        });
    }

    vec
}
