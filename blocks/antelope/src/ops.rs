// Smaller per-transaction operation tables: feature, permission, table, RAM,
// RAM correction and deferred-transaction operations.
use common::structs::BlockIdentity;
use serde_json::json;
use substreams_antelope::pb::{PermissionObject, TransactionTrace};

use crate::pb::pinax::antelope::v2::{DtrxOp, FeatureOp, PermOp, RamCorrectionOp, RamOp, TableOp};
use crate::utils::{dtrx_op_operation_text, perm_op_operation_text, permission_level_text, ram_op_action_text, ram_op_namespace_text, ram_op_operation_text, table_op_operation_text};

pub fn collect_feature_ops(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<FeatureOp> {
    tx.feature_ops
        .iter()
        .map(|op| {
            let feature = op.feature.clone().unwrap_or_default();
            FeatureOp {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                tx_hash: tx.id.clone(),
                tx_index: tx.index,
                action_index: op.action_index,
                kind: op.kind.clone(),
                feature_digest: op.feature_digest.clone(),
                description_digest: feature.description_digest.clone(),
                protocol_feature_type: feature.protocol_feature_type.clone(),
                dependencies: feature.dependencies.clone(),
            }
        })
        .collect()
}

fn permission_json(perm: &PermissionObject) -> String {
    let authority = perm.authority.clone().unwrap_or_default();
    json!({
        "id": perm.id,
        "parent_id": perm.parent_id,
        "owner": perm.owner,
        "name": perm.name,
        "threshold": authority.threshold,
        "keys": authority.keys.iter().map(|k| json!({"public_key": k.public_key, "weight": k.weight})).collect::<Vec<_>>(),
        "accounts": authority.accounts.iter().map(|a| json!({"permission": a.permission.as_ref().map(permission_level_text), "weight": a.weight})).collect::<Vec<_>>(),
        "waits": authority.waits.iter().map(|w| json!({"wait_sec": w.wait_sec, "weight": w.weight})).collect::<Vec<_>>(),
    })
    .to_string()
}

pub fn collect_perm_ops(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<PermOp> {
    tx.perm_ops
        .iter()
        .map(|op| {
            // REMOVE carries only old_perm; INSERT/UPDATE carry new_perm.
            let perm = op.new_perm.clone().or_else(|| op.old_perm.clone()).unwrap_or_default();
            let authority = perm.authority.clone().unwrap_or_default();
            PermOp {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                tx_hash: tx.id.clone(),
                tx_index: tx.index,
                action_index: op.action_index,
                operation: perm_op_operation_text(op.operation),
                id: perm.id,
                parent_id_perm: perm.parent_id,
                owner: perm.owner.clone(),
                name: perm.name.clone(),
                last_updated: perm.last_updated.clone(),
                threshold: authority.threshold,
                keys_public_key: authority.keys.iter().map(|k| k.public_key.clone()).collect(),
                keys_weight: authority.keys.iter().map(|k| k.weight).collect(),
                accounts: authority.accounts.iter().filter_map(|a| a.permission.as_ref().map(permission_level_text)).collect(),
                accounts_weight: authority.accounts.iter().map(|a| a.weight).collect(),
                wait_sec: authority.waits.iter().map(|w| w.wait_sec).collect(),
                wait_weight: authority.waits.iter().map(|w| w.weight).collect(),
                old_perm: op.old_perm.as_ref().filter(|_| op.new_perm.is_some()).map(permission_json),
            }
        })
        .collect()
}

pub fn collect_table_ops(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<TableOp> {
    tx.table_ops
        .iter()
        .enumerate()
        .map(|(index, op)| TableOp {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            tx_hash: tx.id.clone(),
            tx_index: tx.index,
            index: index as u32,
            action_index: op.action_index,
            operation: table_op_operation_text(op.operation),
            payer: op.payer.clone(),
            code: op.code.clone(),
            scope: op.scope.clone(),
            table_name: op.table_name.clone(),
        })
        .collect()
}

pub fn collect_ram_ops(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<RamOp> {
    tx.ram_ops
        .iter()
        .enumerate()
        .map(|(index, op)| RamOp {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            tx_hash: tx.id.clone(),
            tx_index: tx.index,
            index: index as u32,
            action_index: op.action_index,
            operation: ram_op_operation_text(op.operation),
            payer: op.payer.clone(),
            delta: op.delta,
            usage: op.usage,
            namespace: ram_op_namespace_text(op.namespace),
            action: ram_op_action_text(op.action),
            unique_key: op.unique_key.clone(),
        })
        .collect()
}

pub fn collect_ram_correction_ops(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<RamCorrectionOp> {
    tx.ram_correction_ops
        .iter()
        .enumerate()
        .map(|(index, op)| RamCorrectionOp {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            tx_hash: tx.id.clone(),
            tx_index: tx.index,
            index: index as u32,
            correction_id: op.correction_id.clone(),
            unique_key: op.unique_key.clone(),
            payer: op.payer.clone(),
            delta: op.delta,
        })
        .collect()
}

pub fn collect_dtrx_ops(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<DtrxOp> {
    tx.dtrx_ops
        .iter()
        .enumerate()
        .map(|(index, op)| DtrxOp {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            tx_hash: tx.id.clone(),
            tx_index: tx.index,
            index: index as u32,
            action_index: op.action_index,
            operation: dtrx_op_operation_text(op.operation),
            sender: op.sender.clone(),
            sender_id: op.sender_id.clone(),
            payer: op.payer.clone(),
            published_at: op.published_at.clone(),
            delay_until: op.delay_until.clone(),
            expiration_at: op.expiration_at.clone(),
            transaction_id: op.transaction_id.clone(),
        })
        .collect()
}
