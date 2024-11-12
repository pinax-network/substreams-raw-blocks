use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_cosmos::{
    pb::{Event, EventAttribute, TxResults},
    Block,
};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::event_keys;

pub fn insert_tx_events(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TxResults, tx_hash: &str) {
    for (index, event) in transaction.events.iter().enumerate() {
        insert_event(tables, clock, tx_hash, index, event, "tx");
    }
}

pub fn insert_block_events(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    for (index, event) in block.events.iter().enumerate() {
        insert_event(tables, clock, &clock.id, index, event, "block");
    }
}

fn insert_event(tables: &mut DatabaseChanges, clock: &Clock, hash: &str, index: usize, event: &Event, table_prefix: &str) {
    let event_type = &event.r#type;
    let attributes_str = build_attributes_array_string(&event.attributes);

    let index_str = index.to_string();

    let keys = event_keys(hash, &index_str);

    let table_name = format!("{}_events", table_prefix);

    let row = tables
        .push_change_composite(table_name, keys, 0, table_change::Operation::Create)
        .change("index", ("", index_str.as_str()))
        .change("type", ("", event_type.as_str()))
        .change("attributes", ("", attributes_str.as_str()));

    insert_timestamp(row, clock, false, true);
}

// Builds a string in the format of an array of tuples (key, value)
fn build_attributes_array_string(attributes: &[EventAttribute]) -> String {
    let tuples: Vec<(&str, &str)> = attributes.iter().map(|attr| (attr.key.as_str(), attr.value.as_str())).collect();
    serde_json::to_string(&tuples).unwrap_or_default()
}
