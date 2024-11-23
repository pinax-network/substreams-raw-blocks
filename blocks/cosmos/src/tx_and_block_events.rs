use common::structs::BlockTimestamp;
use substreams_cosmos::{pb::TxResults, Block};

use crate::{
    pb::pinax::cosmos::v1::{BlockEvent, TransactionEvent},
    utils::build_attributes_array,
};

pub fn collect_block_events(block: &Block, timestamp: &BlockTimestamp) -> Vec<BlockEvent> {
    let mut vec: Vec<BlockEvent> = vec![];

    for (index, event) in block.events.iter().enumerate() {
        vec.push(BlockEvent {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u32,
            r#type: event.r#type.clone(),
            attributes: build_attributes_array(&event.attributes),
        });
    }

    vec
}

pub fn collect_transaction_events(tx_result: &TxResults, tx_hash: &str, timestamp: &BlockTimestamp) -> Vec<TransactionEvent> {
    let mut vec: Vec<TransactionEvent> = vec![];

    for (event_index, event) in tx_result.events.iter().enumerate() {
        vec.push(TransactionEvent {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: event_index as u32,
            tx_hash: tx_hash.to_string(),
            r#type: event.r#type.clone(),
            attributes: build_attributes_array(&event.attributes),
        });
    }

    vec
}
