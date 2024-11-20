use crate::pb::antelope::CreationTree as RawCreationTree;
use crate::transactions::is_transaction_success;
use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;
use substreams_antelope::Block;

pub fn collect_creation_trees(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawCreationTree> {
    let mut creation_trees: Vec<RawCreationTree> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for creation_flat_node in transaction.creation_tree.iter() {
            creation_trees.push(RawCreationTree {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: tx_hash.clone(),
                tx_success,
                creator_action_index: creation_flat_node.creator_action_index,
                execution_action_index: creation_flat_node.execution_action_index,
            });
        }
    }

    creation_trees
}

pub fn collect_tx_creation_trees(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<RawCreationTree> {
    let mut creation_trees = Vec::new();

    for creation_flat_node in transaction.creation_tree.iter() {
        creation_trees.push(RawCreationTree {
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
