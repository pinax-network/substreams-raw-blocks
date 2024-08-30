use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_antelope::pb::{ActionTrace, PermissionLevel, TransactionTrace};

use crate::keys::authorizations_keys;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L616
pub fn insert_authorization(tables: &mut DatabaseChanges, action: &ActionTrace, transaction: &TransactionTrace, authorization: &PermissionLevel) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_ordinal = &action.action_ordinal;

    // authorization
    let actor = &authorization.actor;
    let permission = &authorization.permission;

    let keys = authorizations_keys(&tx_hash, &action_ordinal, actor, permission);
    tables
        .push_change_composite("authorizations", keys, 0, table_change::Operation::Create)

        // transaction
        .change("tx_hash", ("", tx_hash.to_string().as_str()))

        // action
        .change("action_ordinal", ("", action_ordinal.to_string().as_str()))
        .change("actor", ("", actor.to_string().as_str()))
        .change("permission", ("", permission.to_string().as_str()));
}