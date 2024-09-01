use substreams::{pb::substreams::Clock, Hex};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_antelope::pb::Block;

use crate::{keys::blocks_keys, size::insert_size};
use substreams_database_change::pb::database::TableChange;

use crate::utils::block_time_to_date;

use super::transactions::insert_transaction_clickhouse;

pub fn insert_timestamp_clickhouse(row: &mut TableChange, clock: &Clock) {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(timestamp.to_string().as_str());
    let seconds = timestamp.seconds;
    let nanos = timestamp.nanos;
    let milliseconds = seconds * 1000 + i64::from(nanos) / 1_000_000;
    let block_time = milliseconds.to_string();
    let block_number = clock.number.to_string();
    let block_hash = &clock.id;

    row.change("block_date".to_string(), ("", block_date.as_str()))
        .change("block_time".to_string(), ("", block_time.as_str()))
        .change("block_number".to_string(), ("", block_number.as_str()))
        .change("block_hash".to_string(), ("", block_hash.as_str()));
}

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn insert_blocks_clickhouse(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    // header
    let header = block.header.clone().unwrap_or_default();
    let previous = &header.previous;
    let producer = &header.producer;
    let confirmed = &header.confirmed;
    let schedule_version = &header.schedule_version;

    // block
    let version = block.version;
    let producer_signature = &block.producer_signature;
    let dpos_proposed_irreversible_blocknum = block.dpos_proposed_irreversible_blocknum;
    let dpos_irreversible_blocknum = block.dpos_irreversible_blocknum;

    // blockroot_merkle
    let blockroot_merkle = block.blockroot_merkle.clone().unwrap_or_default();

    // TO-DO
    // Array(String) type is not supported by `substreams-sink-sql`
    // https://github.com/pinax-network/substreams-sink-sql/issues/18
    // let blockroot_merkle_active_nodes = blockroot_merkle.active_nodes.iter().map(|row| bytes_to_hex(row).to_string()).collect::<Vec<String>>();
    let blockroot_merkle_node_count = blockroot_merkle.node_count;

    // block roots
    let transaction_mroot = Hex::encode(&header.transaction_mroot.to_vec());
    let action_mroot = Hex::encode(&header.action_mroot.to_vec());

    // TO-DO
    // to be used during Legacy to Savanna transition where action_mroot needs to be converted from Legacy merkle to Savanna merkle
    // let action_mroot_savanna = block.action_mroot_savanna;

    // blocks
    let keys = blocks_keys(&clock);
    let row = tables
        .push_change_composite("blocks", keys, 0, table_change::Operation::Create)
        // header
        .change("previous", ("", previous.as_str()))
        .change("producer", ("", producer.to_string().as_str()))
        .change("confirmed", ("", confirmed.to_string().as_str()))
        .change("schedule_version", ("", schedule_version.to_string().as_str()))

        // block
        .change("version", ("", version.to_string().as_str()))
        .change("producer_signature", ("", producer_signature.to_string().as_str()))
        .change("dpos_proposed_irreversible_blocknum", ("", dpos_proposed_irreversible_blocknum.to_string().as_str()))
        .change("dpos_irreversible_blocknum", ("", dpos_irreversible_blocknum.to_string().as_str()))

        // block roots
        .change("transaction_mroot", ("", transaction_mroot.to_string().as_str()))
        .change("action_mroot", ("", action_mroot.to_string().as_str()))
        // .change("blockroot_merkle_active_nodes", ("", format!("['{}']", blockroot_merkle_active_nodes.join("','") ).as_str()))
        .change("blockroot_merkle_node_count", ("", blockroot_merkle_node_count.to_string().as_str()))
        ;

    // transaction status counts
    insert_size(row, block);
    insert_timestamp_clickhouse(row, clock);

    // TABLE::transactions
    for transaction in block.transaction_traces() {
        insert_transaction_clickhouse(tables, clock, &transaction, &header);
    }
}