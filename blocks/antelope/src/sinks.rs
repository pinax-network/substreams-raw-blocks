use common::utils::build_timestamp;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::{
    account_ram_deltas::collect_account_ram_deltas, actions::collect_actions, auth_sequences::collect_auth_sequences, authority::collect_authority_vectors, authorizations::collect_authorizations,
    blocks::collect_block, creation_tree::collect_creation_trees, db_ops::collect_db_ops, feature_ops::collect_feature_ops, pb::antelope::Events, perm_ops::collect_perm_ops, ram_ops::collect_ram_ops,
    table_ops::collect_table_ops, transactions::collect_transactions,
};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);
    let authority_vectors = collect_authority_vectors(&block, &timestamp);

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: collect_transactions(&block, &timestamp),
        actions: collect_actions(&block, &timestamp),
        db_ops: collect_db_ops(&block, &timestamp),
        feature_ops: collect_feature_ops(&block, &timestamp),
        perm_ops: collect_perm_ops(&block, &timestamp),
        table_ops: collect_table_ops(&block, &timestamp),
        accounts: authority_vectors.accounts,
        keys: authority_vectors.keys,
        waits: authority_vectors.waits,
        ram_ops: collect_ram_ops(&block, &timestamp),
        authorizations: collect_authorizations(&block, &timestamp),
        auth_sequences: collect_auth_sequences(&block, &timestamp),
        account_ram_deltas: collect_account_ram_deltas(&block, &timestamp),
        creation_trees: collect_creation_trees(&block, &timestamp),
    })
}
