use substreams::{pb::substreams::Clock, Hex};
use substreams_antelope::pb::{ActionTrace, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::keys::action_key;

use super::authorization::insert_authorization;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_action(tables: &mut Tables, clock: &Clock, trace: &ActionTrace, transaction: &TransactionTrace) {
    // action
	let action_ordinal = trace.action_ordinal;
	let action = trace.action.clone().unwrap_or_default();
    let account = action.account;

    // skip if action is an inline notification
    // notifications are not included in the actions table
    // `receivers` table is used to store notifications
    if trace.receiver != account { return; }

    // action data
    let name = action.name;
    let json_data = action.json_data;
    let raw_data = Hex::encode(&action.raw_data.to_vec());

    // transaction
    let tx_hash = &transaction.id;

    // TABLE::Action
    let key = action_key(tx_hash, &action_ordinal);
    tables
        .create_row("Action", key)
        // pointers
        .set("transaction", tx_hash)
        .set("block", &clock.id)

        // action
        .set_bigint("ordinal", &action_ordinal.to_string())
        .set("account", account.to_string())
        .set("name", name.to_string())
        .set("jsonData", json_data.to_string())
        .set("rawData", raw_data.to_string())
        ;

    // TABLE::authorizations
    for authorization in action.authorization.iter() {
        insert_authorization(tables, trace, transaction, authorization);
    };
}