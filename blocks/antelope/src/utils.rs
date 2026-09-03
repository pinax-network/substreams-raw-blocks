use serde_json::json;
use substreams_antelope::pb::{d_trx_op, db_op, perm_op, ram_op, table_op};
use substreams_antelope::pb::{AccountRamDelta, AuthSequence, Exception, PermissionLevel, TransactionStatus};

// Enum names follow firehose-parquet: the Firehose enum name without its prefix.
fn strip(name: &str, prefix: &str) -> String {
    name.strip_prefix(prefix).unwrap_or(name).to_string()
}

pub fn transaction_status_text(v: i32) -> String {
    TransactionStatus::try_from(v)
        .map(|e| strip(e.as_str_name(), "TRANSACTIONSTATUS_"))
        .unwrap_or_else(|_| "UNKNOWN".into())
}
pub fn db_op_operation_text(v: i32) -> String {
    db_op::Operation::try_from(v).map(|e| strip(e.as_str_name(), "OPERATION_")).unwrap_or_else(|_| "UNKNOWN".into())
}
pub fn perm_op_operation_text(v: i32) -> String {
    perm_op::Operation::try_from(v).map(|e| strip(e.as_str_name(), "OPERATION_")).unwrap_or_else(|_| "UNKNOWN".into())
}
pub fn table_op_operation_text(v: i32) -> String {
    table_op::Operation::try_from(v).map(|e| strip(e.as_str_name(), "OPERATION_")).unwrap_or_else(|_| "UNKNOWN".into())
}
pub fn ram_op_operation_text(v: i32) -> String {
    ram_op::Operation::try_from(v).map(|e| strip(e.as_str_name(), "OPERATION_")).unwrap_or_else(|_| "UNKNOWN".into())
}
pub fn ram_op_namespace_text(v: i32) -> String {
    ram_op::Namespace::try_from(v).map(|e| strip(e.as_str_name(), "NAMESPACE_")).unwrap_or_else(|_| "UNKNOWN".into())
}
pub fn ram_op_action_text(v: i32) -> String {
    ram_op::Action::try_from(v).map(|e| strip(e.as_str_name(), "ACTION_")).unwrap_or_else(|_| "UNKNOWN".into())
}
pub fn dtrx_op_operation_text(v: i32) -> String {
    d_trx_op::Operation::try_from(v).map(|e| strip(e.as_str_name(), "OPERATION_")).unwrap_or_else(|_| "UNKNOWN".into())
}

// Antelope ids are hex strings in Firehose; decode for the canonical bytes columns.
pub fn hex_to_bytes(value: &str) -> Vec<u8> {
    hex::decode(value.strip_prefix("0x").unwrap_or(value)).unwrap_or_else(|_| value.as_bytes().to_vec())
}

pub fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
}

pub fn non_empty_bytes(value: &[u8]) -> Option<Vec<u8>> {
    (!value.is_empty()).then(|| value.to_vec())
}

// "actor@permission,actor@permission" (firehose-parquet format)
pub fn format_authorization(auth: &[PermissionLevel]) -> Option<String> {
    let parts: Vec<String> = auth.iter().map(permission_level_text).collect();
    (!parts.is_empty()).then(|| parts.join(","))
}

pub fn permission_level_text(p: &PermissionLevel) -> String {
    format!("{}@{}", p.actor, p.permission)
}

pub fn auth_sequence_json(auth_sequence: &[AuthSequence]) -> Option<String> {
    if auth_sequence.is_empty() {
        return None;
    }
    Some(json!(auth_sequence.iter().map(|a| json!({"account_name": a.account_name, "sequence": a.sequence})).collect::<Vec<_>>()).to_string())
}

pub fn ram_deltas_json(deltas: &[AccountRamDelta]) -> Option<String> {
    if deltas.is_empty() {
        return None;
    }
    Some(json!(deltas.iter().map(|d| json!({"account": d.account, "delta": d.delta})).collect::<Vec<_>>()).to_string())
}

pub fn exception_json(exception: &Option<Exception>) -> Option<String> {
    exception.as_ref().map(|e| {
        json!({
            "code": e.code,
            "name": e.name,
            "message": e.message,
            "stack": e.stack.iter().map(|m| json!({
                "format": m.format,
                "data": String::from_utf8_lossy(&m.data),
                "context": m.context.as_ref().map(|c| json!({"level": c.level, "file": c.file, "line": c.line, "method": c.method, "hostname": c.hostname, "thread_name": c.thread_name})),
            })).collect::<Vec<_>>(),
        })
        .to_string()
    })
}
