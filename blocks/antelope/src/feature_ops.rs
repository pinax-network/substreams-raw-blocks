use common::structs::BlockTimestamp;
use substreams_antelope::pb::TransactionTrace;

use crate::pb::pinax::antelope::v1::FeatureOp;

pub fn collect_tx_feature_ops(transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<FeatureOp> {
    let mut feature_ops = Vec::new();

    for feature_op in transaction.feature_ops.iter() {
        let feature = feature_op.feature.as_ref().expect("feature is required");

        feature_ops.push(FeatureOp {
            // block
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),

            // transaction
            tx_hash: transaction.id.clone(),
            tx_success,

            // feature operation
            feature_digest: feature_op.feature_digest.clone(),
            kind: feature_op.kind.clone(),
            action_index: feature_op.action_index,
            description_digest: feature.description_digest.clone(),
            protocol_feature_type: feature.protocol_feature_type.clone(),
        });
    }

    feature_ops
}
