use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_database_change::pb::database::TableChange;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_antelope::pb::{BlockHeader, TransactionTrace};

use crate::keys::transactions_keys;

use super::actions::insert_action;
use super::blocks::insert_timestamp;
use super::receivers::insert_receiver;

pub fn transaction_status_to_string(status: i32) -> String {
    match status {
        0 => "None".to_string(),
        1 => "Executed".to_string(),
        2 => "Softfail".to_string(),
        3 => "Hardfail".to_string(),
        4 => "Delayed".to_string(),
        5 => "Expired".to_string(),
        6 => "Unknown".to_string(),
        7 => "Canceled".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn is_transaction_success(status: i32) -> bool {
    status == 1
}

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_transaction(tables: &mut DatabaseChanges, clock: &Clock, transaction: &TransactionTrace, block_header: &BlockHeader) {
    let hash = &transaction.id;
    let index = transaction.index;
    let elapsed = transaction.elapsed;
    let net_usage = transaction.net_usage;
    let scheduled = transaction.scheduled;

    // header
    let header = transaction.receipt.clone().unwrap_or_default();
    let cpu_usage_micro_seconds = header.cpu_usage_micro_seconds;
    let net_usage_words = header.net_usage_words;
    let status = transaction_status_to_string(header.status);
    let status_code = header.status;
    let success = is_transaction_success(header.status);

    // block roots
    let transaction_mroot = Hex::encode(&block_header.transaction_mroot.to_vec());

    let keys = transactions_keys(&clock, &hash);
    let row = tables
        .push_change_composite("transactions", keys, 0, table_change::Operation::Create)
        .change("index", ("", index.to_string().as_str()))
        .change("hash", ("", hash.as_str()))
        .change("elapsed", ("", elapsed.to_string().as_str()))
        .change("net_usage", ("", net_usage.to_string().as_str()))
        .change("scheduled", ("", scheduled.to_string().as_str()))

        // header
        .change("cpu_usage_micro_seconds", ("", cpu_usage_micro_seconds.to_string().as_str()))
        .change("net_usage_words", ("", net_usage_words.to_string().as_str()))
        .change("status", ("", status.as_str()))
        .change("status_code", ("", status_code.to_string().as_str()))
        .change("success", ("", success.to_string().as_str()))

        // block roots
        .change("transaction_mroot", ("", transaction_mroot.as_str()))
        ;

    insert_timestamp(row, clock);

    // Traces of each action within the transaction, including all notified and nested actions.
    for trace in transaction.action_traces.iter() {
        insert_receiver(tables, clock, trace, transaction);
        insert_action(tables, clock, trace, transaction, block_header);
    }
}

pub fn insert_transaction_metadata(row: &mut TableChange, transaction: &TransactionTrace) {
    let tx_hash = &transaction.id;
    let tx_index = transaction.index;
    let header = transaction.receipt.clone().unwrap_or_default();
    let tx_status = transaction_status_to_string(header.status);
    let tx_status_code = header.status;
    let tx_success = is_transaction_success(header.status);

    row.change("tx_hash", ("", tx_hash.as_str()))
        .change("tx_index", ("", tx_index.to_string().as_str()))
        .change("tx_status", ("", tx_status.as_str()))
        .change("tx_status_code", ("", tx_status_code.to_string().as_str()))
        .change("tx_success", ("", tx_success.to_string().as_str()));
}