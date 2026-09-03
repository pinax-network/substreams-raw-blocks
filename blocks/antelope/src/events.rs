use std::collections::HashMap;

use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::actions::collect_actions;
use crate::blocks::collect_block;
use crate::db_ops::collect_db_ops;
use crate::ops::{collect_dtrx_ops, collect_feature_ops, collect_perm_ops, collect_ram_correction_ops, collect_ram_ops, collect_table_ops};
use crate::pb::pinax::antelope::v2::Events;
use crate::transactions::collect_transaction;
use crate::utils::hex_to_bytes;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let parent_id = block.header.as_ref().map(|h| hex_to_bytes(&h.previous)).unwrap_or_default();
    let id = BlockIdentity::new(block.number as u64, hex_to_bytes(&block.id), (block.number as u64).saturating_sub(1), parent_id, &clock.timestamp);

    // Signatures live on the transaction receipts, traces carry the execution.
    let receipts_source = if block.filtering_applied { &block.filtered_transactions } else { &block.unfiltered_transactions };
    let receipts: HashMap<&str, _> = receipts_source.iter().map(|r| (r.id.as_str(), r)).collect();

    let mut events = Events::default();
    events.blocks.push(collect_block(&block, &id));

    for tx in block.transaction_traces() {
        events.transactions.push(collect_transaction(tx, receipts.get(tx.id.as_str()).copied(), &id));
        events.actions.extend(collect_actions(tx, &id));
        events.db_ops.extend(collect_db_ops(tx, &id));
        events.feature_ops.extend(collect_feature_ops(tx, &id));
        events.perm_ops.extend(collect_perm_ops(tx, &id));
        events.table_ops.extend(collect_table_ops(tx, &id));
        events.ram_ops.extend(collect_ram_ops(tx, &id));
        events.ram_correction_ops.extend(collect_ram_correction_ops(tx, &id));
        events.dtrx_ops.extend(collect_dtrx_ops(tx, &id));
    }

    Ok(events)
}
