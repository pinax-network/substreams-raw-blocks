use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{CreationFlatNode, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::creation_flat_nodes_keys, transactions::insert_transaction_metadata};

pub fn insert_creation_flat_node(tables: &mut DatabaseChanges, clock: &Clock, creation_flat_node: &CreationFlatNode, transaction: &TransactionTrace) {
    let creator_action_index = &creation_flat_node.creator_action_index;
    let execution_action_index = &creation_flat_node.execution_action_index;

    let keys = creation_flat_nodes_keys(clock, &transaction.id, creator_action_index, execution_action_index);
    let row = tables
        .push_change_composite("creation_flat_nodes", keys, 0, table_change::Operation::Create)
        .change("creator_action_index", ("", creator_action_index.to_string().as_str()))
        .change("execution_action_index", ("", execution_action_index.to_string().as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}
