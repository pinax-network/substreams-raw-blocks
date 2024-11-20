use crate::pb::antelope::CreationTree;
use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;

pub fn collect_tx_creation_trees(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<CreationTree> {
    let mut creation_trees = Vec::new();

    for creation_flat_node in transaction.creation_tree.iter() {
        creation_trees.push(CreationTree {
            block_time: Some(timestamp.time.clone()),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),
            tx_hash: transaction.id.clone(),
            tx_success,
            creator_action_index: creation_flat_node.creator_action_index,
            execution_action_index: creation_flat_node.execution_action_index,
        });
    }

    creation_trees
}
