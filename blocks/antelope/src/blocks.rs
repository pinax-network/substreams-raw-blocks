use crate::{pb::antelope::Block as EventsBlock, size::compute_block_size};
use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_antelope::Block;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn collect_block(block: &Block, timestamp: &BlockTimestamp) -> EventsBlock {
    let header = block.header.as_ref().expect("missing block header");
    let blockroot_merkle = block.blockroot_merkle.clone().unwrap_or_default();
    let size = compute_block_size(block);

    EventsBlock {
        // clock
        time: Some(timestamp.time),
        number: timestamp.number,
        date: timestamp.date.clone(),
        hash: timestamp.hash.clone(),

        // block
        parent_hash: header.previous.clone(),
        producer: header.producer.clone(),
        confirmed: header.confirmed,
        schedule_version: header.schedule_version,
        version: block.version,
        producer_signature: block.producer_signature.clone(),
        dpos_proposed_irreversible_blocknum: block.dpos_proposed_irreversible_blocknum,
        dpos_irreversible_blocknum: block.dpos_irreversible_blocknum,
        transaction_mroot: Hex::encode(&header.transaction_mroot.to_vec()),
        action_mroot: Hex::encode(&header.action_mroot.to_vec()),
        blockroot_merkle_active_nodes: blockroot_merkle.active_nodes.iter().map(|row| Hex::encode(row)).collect::<Vec<String>>(),
        blockroot_merkle_node_count: blockroot_merkle.node_count,

        // counters
        size: size.size,
        total_transactions: size.total_transactions,
        successful_transactions: size.successful_transactions,
        failed_transactions: size.failed_transactions,
        total_actions: size.total_actions,
        total_db_ops: size.total_db_ops,
    }
}
