use common::utils::build_timestamp;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::actions::collect_tx_actions;
use crate::blocks::collect_block;
use crate::db_ops::collect_tx_db_ops;
use crate::feature_ops::collect_tx_feature_ops;
use crate::pb::antelope::Events;
use crate::perm_ops::collect_tx_perm_ops;
use crate::ram_ops::collect_tx_ram_ops;
use crate::table_ops::collect_tx_table_ops;
use crate::transactions::{collect_transaction, is_transaction_success};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);
    let mut events = Events::default();
    events.blocks.push(collect_block(&block, &timestamp));

    for transaction in block.transaction_traces() {
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);
        events.transactions.push(collect_transaction(&block, transaction, &timestamp, tx_success));
        events.actions.extend(collect_tx_actions(&block, transaction, &timestamp, tx_success));
        events.db_ops.extend(collect_tx_db_ops(transaction, &timestamp, tx_success));
        events.feature_ops.extend(collect_tx_feature_ops(transaction, &timestamp, tx_success));
        events.perm_ops.extend(collect_tx_perm_ops(transaction, &timestamp, tx_success));
        events.table_ops.extend(collect_tx_table_ops(transaction, &timestamp, tx_success));
        events.ram_ops.extend(collect_tx_ram_ops(transaction, &timestamp, tx_success));
    }

    Ok(events)
}
