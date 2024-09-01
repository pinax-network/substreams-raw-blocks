use substreams::{pb::substreams::Clock, Hex};
use substreams_antelope::pb::Block;
use substreams_entity_change::tables::{Row, Tables};

use crate::size::insert_size;
use crate::transactions::insert_transaction;

use crate::utils::block_time_to_date;

pub fn insert_timestamp(row: &mut Row, clock: &Clock) {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(timestamp.to_string().as_str());
    let seconds = timestamp.seconds;
    // let nanos = timestamp.nanos;
    // let milliseconds = seconds * 1000 + i64::from(nanos) / 1_000_000;
    let block_time = seconds;
    let block_number = clock.number.to_string();
    let block_hash = &clock.id;

    row.set("date", block_date)
        .set_bigint("time", &block_time.to_string())
        .set_bigint("number", &block_number.to_string())
        .set("hash", block_hash);
}

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn insert_blocks(tables: &mut Tables, clock: &Clock, block: &Block) {
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
    let blockroot_merkle_active_nodes = blockroot_merkle.active_nodes.iter().map(|row| Hex::encode(row)).collect::<Vec<String>>();
    let blockroot_merkle_node_count = blockroot_merkle.node_count;

    // block roots
    let transaction_mroot = Hex::encode(&header.transaction_mroot.to_vec());
    let action_mroot = Hex::encode(&header.action_mroot.to_vec());

    // TO-DO
    // to be used during Legacy to Savanna transition where action_mroot needs to be converted from Legacy merkle to Savanna merkle
    // let action_mroot_savanna = block.action_mroot_savanna;

    // blocks
    let row = tables.create_row("Block", &clock.id)
        // header
        .set("previous", previous)
        .set("producer", producer)
        .set_bigint("confirmed", &confirmed.to_string())
        .set("scheduleVersion", schedule_version.to_string())

        // block
        .set_bigint("version", &version.to_string())
        .set("producerSignature", producer_signature)
        .set_bigint("dposProposedIrreversibleBlocknum", &dpos_proposed_irreversible_blocknum.to_string())
        .set_bigint("dposIrreversibleBlocknum", &dpos_irreversible_blocknum.to_string())

        // block roots
        .set("transactionMroot", transaction_mroot)
        .set("actionMroot", action_mroot)
        .set("blockrootMerkleActiveNodes", blockroot_merkle_active_nodes)
        .set_bigint("blockrootMerkleNodeCount", &blockroot_merkle_node_count.to_string());

    // transaction status counts
    insert_size(row, block);
    insert_timestamp(row, clock);

    // TABLE::transactions
    for transaction in block.transaction_traces() {
        insert_transaction(tables, clock, &transaction, &header);
    }
}
