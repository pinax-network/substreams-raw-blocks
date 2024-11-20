use common::structs::BlockTimestamp;
use substreams_antelope::Block;

use crate::{
    account_ram_deltas::collect_tx_account_ram_deltas,
    actions::collect_tx_actions,
    auth_sequences::collect_tx_auth_sequences,
    authority::collect_tx_authority_vectors,
    authorizations::collect_tx_authorizations,
    blocks::collect_block,
    creation_tree::collect_tx_creation_trees,
    db_ops::collect_tx_db_ops,
    feature_ops::collect_tx_feature_ops,
    pb::antelope::Events as RawEvents,
    perm_ops::collect_tx_perm_ops,
    ram_ops::collect_tx_ram_ops,
    table_ops::collect_tx_table_ops,
    transactions::{collect_transaction, is_transaction_success},
};

pub fn collect_events(block: &Block, timestamp: &BlockTimestamp) -> RawEvents {
    let mut events = RawEvents {
        blocks: vec![collect_block(block, timestamp)],
        transactions: vec![],
        actions: vec![],
        db_ops: vec![],
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
    };

    for transaction in block.transaction_traces() {
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);
        events.transactions.push(collect_transaction(block, transaction, timestamp, tx_success));
        events.actions.extend(collect_tx_actions(block, transaction, timestamp, tx_success));
        events.db_ops.extend(collect_tx_db_ops(transaction, timestamp, tx_success));
        events.feature_ops.extend(collect_tx_feature_ops(transaction, timestamp, tx_success));
        events.perm_ops.extend(collect_tx_perm_ops(transaction, timestamp, tx_success));
        events.table_ops.extend(collect_tx_table_ops(transaction, timestamp, tx_success));
        let authority_vectors = collect_tx_authority_vectors(transaction, timestamp, tx_success);
        events.accounts.extend(authority_vectors.accounts);
        events.keys.extend(authority_vectors.keys);
        events.waits.extend(authority_vectors.waits);
        events.ram_ops.extend(collect_tx_ram_ops(transaction, timestamp, tx_success));
        events.authorizations.extend(collect_tx_authorizations(transaction, timestamp, tx_success));
        events.auth_sequences.extend(collect_tx_auth_sequences(transaction, timestamp, tx_success));
        events.account_ram_deltas.extend(collect_tx_account_ram_deltas(transaction, timestamp, tx_success));
        events.creation_trees.extend(collect_tx_creation_trees(transaction, timestamp, tx_success));
    }

    events
}
