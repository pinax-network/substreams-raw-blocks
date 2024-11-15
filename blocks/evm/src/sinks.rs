use common::utils::build_timestamp;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;

use substreams_ethereum::pb::eth::v2::Block;

use crate::balance_changes::collect_balance_changes;
use crate::blocks::{block_detail_to_string, collect_block};
use crate::logs::collect_logs;
use crate::pb::evm::Events;
use crate::traces::collect_traces;
use crate::transactions::collect_transactions;

// #[substreams::handlers::map]
// pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
//     let mut tables: DatabaseChanges = DatabaseChanges::default();

//     // TABLE::blocks
//     insert_blocks(&mut tables, &clock, &block);

//     Ok(tables)
// }

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);
    let detail_level = block_detail_to_string(block.detail_level);

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: collect_transactions(&block, &timestamp),
        logs: collect_logs(&block, &timestamp, &detail_level),
        traces: collect_traces(&block, &timestamp, &detail_level),
        balance_changes: collect_balance_changes(&block, &timestamp),
        storage_changes: vec![],
        code_changes: vec![],
        account_creations: vec![],
        nonce_changes: vec![],
        gas_changes: vec![],
    })
}

// // TO-DO: Implement the `graph_out` function using EntityChanges
// #[substreams::handlers::map]
// pub fn graph_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
//     let mut tables: DatabaseChanges = DatabaseChanges::default();
//     insert_blocks(&mut tables, &clock, &block);
//     // TO-DO: Convert DatabaseChanges to EntityChanges
//     Ok(tables)
// }
