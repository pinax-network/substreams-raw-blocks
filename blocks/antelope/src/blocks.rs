use common::blocks::{insert_timestamp, insert_transaction_counts};
use common::utils::bytes_to_hex;
use common::keys::blocks_keys;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_antelope::pb::Block;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn insert_blocks(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    // header
    let header = block.header.clone().unwrap_or_default();
    let previous = format!("0x{}", header.previous);
    let producer = header.producer;
    let confirmed = header.confirmed;
    let schedule_version = header.schedule_version;

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
    let transaction_mroot = bytes_to_hex(&header.transaction_mroot.to_vec());
    let action_mroot = bytes_to_hex(&header.action_mroot.to_vec());

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
    let all_transaction_status: Vec<i32> = block.transaction_traces().map(|transaction| transaction.receipt.clone().unwrap().status).collect();
    insert_transaction_counts(row, all_transaction_status);

    insert_timestamp(row, clock, true);
}
