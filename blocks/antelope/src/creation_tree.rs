use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{CreationFlatNode, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::pb::antelope::CreationTree as RawCreationTree;
use crate::transactions::is_transaction_success;
use crate::{keys::creation_tree_keys, transactions::insert_transaction_metadata};
use common::structs::BlockTimestamp;
use substreams_antelope::Block;

pub fn insert_creation_tree(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace, creation_flat_node: &CreationFlatNode) {
    let creator_action_index = creation_flat_node.creator_action_index;
    let execution_action_index = creation_flat_node.execution_action_index;
    let keys = creation_tree_keys(&transaction.id, &creator_action_index, &execution_action_index);
    let row = tables
        .push_change_composite("creation_tree", keys, 0, table_change::Operation::Create)
        .change("creator_action_index", ("", creation_flat_node.creator_action_index.to_string().as_str()))
        .change("execution_action_index", ("", creation_flat_node.execution_action_index.to_string().as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}

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
