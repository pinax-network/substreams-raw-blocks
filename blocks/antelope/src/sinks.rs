use common::utils::build_timestamp;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::{actions::collect_actions, blocks::collect_block, db_ops::collect_db_ops, pb::antelope::Events, transactions::collect_transactions};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: collect_transactions(&block, &timestamp),
        actions: collect_actions(&block, &timestamp),
        db_ops: collect_db_ops(&block, &timestamp),
        feature_ops: vec![],
        perm_ops: vec![],
        table_ops: vec![],
        accounts: vec![],
        keys: vec![],
        waits: vec![],
        ram_ops: vec![],
        authorizations: vec![],
        auth_sequences: vec![],
        account_ram_deltas: vec![],
        creation_trees: vec![],
    })
}
