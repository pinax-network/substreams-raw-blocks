use common::utils::build_timestamp;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::account_ram_deltas::collect_tx_account_ram_deltas;
use crate::actions::collect_tx_actions;
use crate::auth_sequences::collect_tx_auth_sequences;
use crate::authority::collect_tx_authority_vectors;
use crate::authorizations::collect_tx_authorizations;
use crate::blocks::collect_block;
use crate::creation_tree::collect_tx_creation_trees;
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

    let mut events = Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: Vec::new(),
        actions: Vec::new(),
        db_ops: Vec::new(),
        feature_ops: Vec::new(),
        perm_ops: Vec::new(),
        table_ops: Vec::new(),
        accounts: Vec::new(),
        keys: Vec::new(),
        waits: Vec::new(),
        ram_ops: Vec::new(),
        authorizations: Vec::new(),
        auth_sequences: Vec::new(),
        account_ram_deltas: Vec::new(),
        creation_trees: Vec::new(),
    };

    for transaction in block.transaction_traces() {
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);
        events.transactions.push(collect_transaction(&block, transaction, &timestamp, tx_success));
        events.actions.extend(collect_tx_actions(&block, transaction, &timestamp, tx_success));
        events.db_ops.extend(collect_tx_db_ops(transaction, &timestamp, tx_success));
        events.feature_ops.extend(collect_tx_feature_ops(transaction, &timestamp, tx_success));
        events.perm_ops.extend(collect_tx_perm_ops(transaction, &timestamp, tx_success));
        events.table_ops.extend(collect_tx_table_ops(transaction, &timestamp, tx_success));
        let authority_vectors = collect_tx_authority_vectors(transaction, &timestamp, tx_success);
        events.accounts.extend(authority_vectors.accounts);
        events.keys.extend(authority_vectors.keys);
        events.waits.extend(authority_vectors.waits);
        events.ram_ops.extend(collect_tx_ram_ops(transaction, &timestamp, tx_success));
        events.authorizations.extend(collect_tx_authorizations(transaction, &timestamp, tx_success));
        events.auth_sequences.extend(collect_tx_auth_sequences(transaction, &timestamp, tx_success));
        events.account_ram_deltas.extend(collect_tx_account_ram_deltas(transaction, &timestamp, tx_success));
        events.creation_trees.extend(collect_tx_creation_trees(transaction, &timestamp, tx_success));
    }

    Ok(events)
}
