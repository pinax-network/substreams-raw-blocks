use common::structs::BlockIdentity;
use substreams_antelope::pb::Block;

use crate::pb::pinax::antelope::v2::Block as BlockRow;

// https://github.com/pinax-network/firehose-antelope/blob/develop/proto/sf/antelope/type/v1/type.proto
pub fn collect_block(block: &Block, id: &BlockIdentity) -> BlockRow {
    let header = block.header.clone().unwrap_or_default();
    let merkle = block.blockroot_merkle.clone().unwrap_or_default();
    let num_transactions = block.transaction_traces().count() as u32;
    let num_actions = if block.filtering_applied {
        block.filtered_executed_total_action_count
    } else {
        block.unfiltered_executed_total_action_count
    };

    BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        number: block.number,
        hash: block.id.clone(),
        producer: header.producer.clone(),
        confirmed: header.confirmed,
        schedule_version: header.schedule_version,

        parent_hash: header.previous.clone(),
        version: block.version,
        producer_signature: block.producer_signature.clone(),
        transaction_mroot: header.transaction_mroot.clone(),
        action_mroot: header.action_mroot.clone(),
        action_mroot_savanna: block.action_mroot_savanna.clone(),
        blockroot_merkle_active_nodes: merkle.active_nodes.clone(),
        blockroot_merkle_node_count: merkle.node_count,
        block_signing_key: block.block_signing_key.clone(),
        confirm_count: block.confirm_count.clone(),
        dpos_proposed_irreversible_blocknum: block.dpos_proposed_irreversible_blocknum,
        dpos_irreversible_blocknum: block.dpos_irreversible_blocknum,
        finality_lib: block.finality_lib,
        validated: block.validated,
        num_transactions,
        num_actions,
        filtering_applied: block.filtering_applied,
    }
}
