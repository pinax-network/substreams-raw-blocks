use common::{blocks::insert_timestamp, utils::bytes_to_hex_no_prefix};
use common::keys::traces_keys;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_antelope::pb::{ActionTrace, BlockHeader, TransactionTrace};

use crate::transactions::insert_transaction_metadata;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_trace(tables: &mut DatabaseChanges, clock: &Clock, trace: &ActionTrace, transaction: &TransactionTrace, block_header: &BlockHeader) {
    // receipt
	let receipt = trace.receipt.clone().unwrap_or_default();
    let abi_sequence = receipt.abi_sequence;
    let code_sequence = receipt.code_sequence;
    let digest = &receipt.digest;
    let global_sequence = receipt.global_sequence;
    let receipt_receiver = receipt.receiver;
    let recv_sequence = receipt.recv_sequence;

    // action
	let action = trace.action.clone().unwrap_or_default();
    let account = action.account;
    let name = action.name;
    let json_data = action.json_data;
    let raw_data = bytes_to_hex_no_prefix(&action.raw_data.to_vec());

    // trace
	let receiver = &trace.receiver;
	let context_free = trace.context_free;
	let elapsed = trace.elapsed;
	let console = &trace.console;
	let raw_return_value = bytes_to_hex_no_prefix(&trace.raw_return_value.to_vec());
	let json_return_value = &trace.json_return_value;
	let action_ordinal = trace.action_ordinal;
	let creator_action_ordinal = trace.creator_action_ordinal;
	let closest_unnotified_ancestor_action_ordinal = trace.closest_unnotified_ancestor_action_ordinal;
	let execution_index = trace.execution_index;

    // block roots
    let action_mroot = bytes_to_hex_no_prefix(&block_header.action_mroot.to_vec());

    let keys = traces_keys(&clock, &transaction.id, &transaction.index, &action_ordinal);
    let row = tables
        .push_change_composite("traces", keys, 0, table_change::Operation::Create)

        // receipt
        .change("abi_sequence", ("", abi_sequence.to_string().as_str()))
        .change("code_sequence", ("", code_sequence.to_string().as_str()))
        .change("digest", ("", digest.to_string().as_str()))
        .change("global_sequence", ("", global_sequence.to_string().as_str()))
        .change("receipt_receiver", ("", receipt_receiver.to_string().as_str()))
        .change("recv_sequence", ("", recv_sequence.to_string().as_str()))

        // action
        .change("account", ("", account.to_string().as_str()))
        .change("name", ("", name.to_string().as_str()))
        .change("json_data", ("", json_data.to_string().as_str()))
        .change("raw_data", ("", raw_data.to_string().as_str()))

        // trace
        .change("receiver", ("", receiver.to_string().as_str()))
        .change("context_free", ("", context_free.to_string().as_str()))
        .change("elapsed", ("", elapsed.to_string().as_str()))
        .change("console", ("", console.to_string().as_str()))
        .change("raw_return_value", ("", raw_return_value.to_string().as_str()))
        .change("json_return_value", ("", json_return_value.to_string().as_str()))
        .change("action_ordinal", ("", action_ordinal.to_string().as_str()))
        .change("creator_action_ordinal", ("", creator_action_ordinal.to_string().as_str()))
        .change("closest_unnotified_ancestor_action_ordinal", ("", closest_unnotified_ancestor_action_ordinal.to_string().as_str()))
        .change("execution_index", ("", execution_index.to_string().as_str()))

        // block roots
        .change("action_mroot", ("", action_mroot.as_str()))
        ;

    insert_transaction_metadata(row, transaction);

    // TO-DO
    // Need Array(String) support
    // https://github.com/pinax-network/substreams-sink-sql/issues/18
    // action.authorization.iter().for_each(|authorization| {
    //     let actor = authorization.actor;
    //     let permission = authorization.permission;
    // });

    // TO-DO
    // Need Array(String) support
    // https://github.com/pinax-network/substreams-sink-sql/issues/18
    // receipt.auth_sequence.iter().for_each(|auth_sequence| {
    //     let account_name = auth_sequence.account_name;
    //     let sequence = auth_sequence.sequence;
    // });

    // TO-DO
    // exception
	// let exception = trace.exception.unwrap_or_default();
	// let error_code = trace.error_code;

    // TO-DO
    // for account_ram_delta in transaction.account_ram_deltas.iter() {
    //     insert_account_ram_delta(tables, clock, trace, &account_ram_delta, &block);
    // }
    insert_timestamp(row, clock, false);
}