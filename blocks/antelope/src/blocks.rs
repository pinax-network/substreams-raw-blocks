use common::blocks::insert_timestamp;
use common::utils::bytes_to_hex_no_prefix;
use common::keys::blocks_keys;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges, TableChange};
use substreams_antelope::pb::Block;

use crate::size::insert_size;
use crate::transactions::insert_transaction;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn insert_blocks(params: String, tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
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
    // let blockroot_merkle_active_nodes = blockroot_merkle.active_nodes.iter().map(|row| bytes_to_hex_no_prefix(row).to_string()).collect::<Vec<String>>();
    let blockroot_merkle_node_count = blockroot_merkle.node_count;

    // block roots
    let transaction_mroot = bytes_to_hex_no_prefix(&header.transaction_mroot.to_vec());
    let action_mroot = bytes_to_hex_no_prefix(&header.action_mroot.to_vec());

    // TO-DO
    // to be used during Legacy to Savanna transition where action_mroot needs to be converted from Legacy merkle to Savanna merkle
    // let action_mroot_savanna = block.action_mroot_savanna;

    // blocks
    let keys = blocks_keys(&clock, true);
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
    insert_transaction_counters(row, block);
    insert_timestamp(row, clock, true, false);

    // skip the rest if blocks is the only requested table
    // designed for high throughput to calculate total block size of the entire chain
    if params == "blocks" { return; }

    // TABLE::transactions
    for transaction in block.transaction_traces() {
        insert_transaction(tables, clock, &transaction, &header);
    }
}

pub fn insert_transaction_counters(row: &mut TableChange, block: &Block) {
    // counters
    let mut total_transactions = 0;
    let mut successful_transactions = 0;
    let mut failed_transactions = 0;
    let mut total_actions = 0;
    let mut total_db_ops = 0;

    for transaction in block.transaction_traces() {
        let status = transaction.receipt.clone().unwrap_or_default().status;
        if status == 1 {
            successful_transactions += 1;
        } else {
            failed_transactions += 1;
        }
        total_transactions += 1;
        total_actions += transaction.action_traces.len();
        total_db_ops += transaction.db_ops.len();
    }

    row.change("total_transactions", ("", total_transactions.to_string().as_str()))
        .change("successful_transactions", ("", successful_transactions.to_string().as_str()))
        .change("failed_transactions", ("", failed_transactions.to_string().as_str()))
        .change("total_actions", ("", total_actions.to_string().as_str()))
        .change("total_db_ops", ("", total_db_ops.to_string().as_str()))
    ;
}
