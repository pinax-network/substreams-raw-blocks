// use std::collections::HashSet;

use serde_json::Value;
use substreams_antelope::pb::{ActionTrace, DbOp, PermissionLevel};

// i.e. https://docs.dfuse.eosnation.io/eosio/public-apis/reference/search/terms/
pub fn collect_action_keys(trace: &ActionTrace) -> Vec<String> {
    let action = trace.action.as_ref().expect("invalid action trace");

    let mut keys = Vec::with_capacity(action.authorization.len() * 2 + 5 + 3);
    let json_data: Value = match serde_json::from_str(&action.json_data) {
        Ok(data) => data,
        Err(_) => Value::Object(Default::default()),
    };

    // action: is the name of the action executed
    // code: matches the account called on the action. Not to be mixed up with receiver. This will match notifications sent from a contract to another account.
    // receiver: means the account with code that has executed the action. This is unambiguous.
    keys.extend(vec![
        format!("action:{}", action.name),
        format!("code:{}", action.account),
        format!("receiver:{}", trace.receiver),
    ]);

    // data.sub.fields: a good number of action-specific fields can be matched for equality.
    // Ex: data.from and data.to (present in transfer operations).
    // lists are flattened, and terms matched when the query is present in the list.
    // nested documents fields are separated by ., so data.user.id:hello will match the action data: {"data": {"user": {"id": "hello"}}}
    keys.extend(
        json_data
            .as_object()
            .expect("json_data must be an object")
            .iter()
            .filter_map(|(key, value)| match value {
                Value::String(value) => Some(format!("data.{}:{}", key, value)),
                Value::Number(value) => Some(format!("data.{}:{}", key, value)),
                Value::Bool(value) => Some(format!("data.{}:{}", key, value)),
                _ => None,
            }),
    );

    // auth: means the action was signed with the authority of a given account.
    // The field has two formats:
    // auth:account an account, irrespective of the permission which signed the transaction
    // auth:account@perm an account and specific permission that signed the transaction, in which this action was declared.
    keys.extend(
        action
            .authorization
            .iter()
            .flat_map(|PermissionLevel { actor, permission }| {
                vec![
                    format!("auth:{actor}@{permission}"),
                    format!("auth:{actor}"),
                ]
            }),
    );

    // input:true will match only the top-level actions (those present in the original transactions, and not as a side effect of contract execution).
    if trace.creator_action_ordinal == 0 {
        keys.push("input:true".to_string());
    }

    // notif:true will match only notifications, excluding input action or other inline actions.
    if action.account.ne(&trace.receiver) {
        keys.push("notif:true".to_string());
    }

    keys
}

// i.e. https://docs.dfuse.eosnation.io/eosio/public-apis/reference/search/terms/
pub fn collect_db_op_keys(db_op: &DbOp) -> Vec<String> {
    let mut keys = Vec::new();
    // let json_data: Value = match serde_json::from_str(&db_op.new_data_json) {
    //     Ok(data) => data,
    //     Err(_) => Value::Object(Default::default()),
    // };

    // db.table:accounts/swap.defi account:eosio.token
    keys.extend(vec![
        format!("db.table:{}", db_op.table_name),
        format!("db.table:{}/{}", db_op.table_name, db_op.scope),
    ]);

    // TO-DO
    // keys.extend(
    //     json_data
    //         .as_object()
    //         .expect("json_data must be an object")
    //         .iter()
    //         .filter_map(|(key, value)| match value {
    //             Value::String(value) => Some(format!("db.table:{}:{}", key, value)),
    //             Value::Number(value) => Some(format!("db.table:{}:{}", key, value)),
    //             Value::Bool(value) => Some(format!("db.table:{}:{}", key, value)),
    //             _ => None,
    //         }),
    // );

    keys
}