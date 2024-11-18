use common::{blocks::insert_timestamp, structs::BlockTimestamp};
use substreams::pb::substreams::Clock;
use substreams_antelope::{
    pb::{FeatureOp, TransactionTrace},
    Block,
};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{
    keys::feature_ops_keys,
    pb::antelope::FeatureOp as RawFeatureOp,
    transactions::{insert_transaction_metadata, is_transaction_success},
};

pub fn insert_feature_op(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace, feature_op: &FeatureOp) {
    let feature_digest = &feature_op.feature_digest.as_str();
    let kind = feature_op.kind.as_str();
    let action_index = feature_op.action_index;
    let feature = feature_op.feature.as_ref().expect("feature is required");
    let description_digest = feature.description_digest.as_str();
    let protocol_feature_type = feature.protocol_feature_type.as_str();
    // TO-DO: feature.dependencies
    // TO-DO: feature.specification
    // TO-DO: feature.subjective_restrictions

    let keys = feature_ops_keys(feature_digest);
    let row = tables
        .push_change_composite("feature_ops", keys, 0, table_change::Operation::Create)
        .change("kind", ("", kind.to_string().as_str()))
        .change("action_index", ("", action_index.to_string().as_str()))
        .change("feature_digest", ("", feature_digest.to_string().as_str()))
        .change("description_digest", ("", description_digest.to_string().as_str()))
        .change("protocol_feature_type", ("", protocol_feature_type.to_string().as_str()));

    insert_transaction_metadata(row, transaction);
    insert_timestamp(row, clock, false, false);
}

pub fn collect_feature_ops(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawFeatureOp> {
    let mut feature_ops: Vec<RawFeatureOp> = vec![];

    for transaction in block.transaction_traces() {
        let tx_hash = &transaction.id;
        let tx_success = is_transaction_success(transaction.receipt.clone().unwrap_or_default().status);

        for feature_op in transaction.feature_ops.iter() {
            let feature = feature_op.feature.as_ref().expect("feature is required");

            feature_ops.push(RawFeatureOp {
                block_time: Some(timestamp.time.clone()),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: tx_hash.clone(),
                tx_success,
                feature_digest: feature_op.feature_digest.clone(),
                kind: feature_op.kind.clone(),
                action_index: feature_op.action_index,
                description_digest: feature.description_digest.clone(),
                protocol_feature_type: feature.protocol_feature_type.clone(),
            });
        }
    }

    feature_ops
}
