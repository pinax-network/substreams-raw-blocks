use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{FeatureOp, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::feature_ops_keys;
use crate::transactions::insert_transaction_metadata;

pub fn insert_feature_op(tables: &mut DatabaseChanges, clock: &Clock, feature_op: &FeatureOp, transaction: &TransactionTrace) {
    let feature_digest = &feature_op.feature_digest;
    let kind = &feature_op.kind;
    let action_index = &feature_op.action_index;
    let description_digest = &feature_op.feature.as_ref().unwrap().description_digest;
    let protocol_feature_type = &feature_op.feature.as_ref().unwrap().protocol_feature_type;

    let keys = feature_ops_keys(clock, &transaction.id, &feature_op.kind, &feature_op.feature_digest);
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
