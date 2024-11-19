use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::{pb::TxResults, Block};

use crate::{
    pb::cosmos::{BlockEvent as RawBlockEvent, TransactionEvent as RawTransactionEvent},
    utils::build_attributes_array_string,
};

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

pub fn collect_transaction_events(tx_result: &TxResults, tx_hash: &str, timestamp: &BlockTimestamp) -> Vec<RawTransactionEvent> {
    let mut vec: Vec<RawTransactionEvent> = vec![];

    for (event_index, event) in tx_result.events.iter().enumerate() {
        vec.push(RawTransactionEvent {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: event_index as u32,
            tx_hash: tx_hash.to_string(),
            r#type: event.r#type.clone(),
            attributes: build_attributes_array_string(&event.attributes),
        });
    }

    vec
}
