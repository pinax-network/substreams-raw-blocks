use common::{blocks::insert_timestamp, structs::BlockTimestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_cosmos::{
    pb::{Event, TxResults},
    Block,
};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{
    keys::event_keys,
    pb::cosmos::rawblocks::{BlockEvent as RawBlockEvent, TransactionEvent as RawTransactionEvent},
    utils::build_attributes_array_string,
};

pub fn collect_tx_events(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawTransactionEvent> {
    let mut vec: Vec<RawTransactionEvent> = vec![];

    for (index, event) in block.events.iter().enumerate() {
        vec.push(RawTransactionEvent {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: bytes_to_hex(&block.hash),
            index: index as u32,
            tx_hash: bytes_to_hex(&block.hash),
            r#type: event.r#type.clone(),
            attributes: build_attributes_array_string(&event.attributes),
        });
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
            block_hash: bytes_to_hex(&block.hash),
            index: index as u32,
            r#type: event.r#type.clone(),
            attributes: build_attributes_array_string(&event.attributes),
        });
    }

    vec
}
