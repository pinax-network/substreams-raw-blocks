use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::{FeatureOp, TransactionTrace};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::feature_ops_keys;
use crate::transactions::insert_transaction_metadata;

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
