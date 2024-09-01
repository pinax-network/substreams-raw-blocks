use std::collections::HashMap;

use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_antelope::pb::{BlockHeader, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::actions::insert_action;
use crate::blocks::insert_timestamp;
use crate::receivers::insert_receiver;

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
pub fn insert_transaction(tables: &mut Tables, clock: &Clock, transaction: &TransactionTrace, block_header: &BlockHeader) {
    let hash = &transaction.id;
    let index = transaction.index;
    let elapsed = transaction.elapsed;
    let net_usage = transaction.net_usage;

    // header
    let header = transaction.receipt.clone().unwrap_or_default();
    let cpu_usage_micro_seconds = header.cpu_usage_micro_seconds;
    let net_usage_words = header.net_usage_words;
    let status = transaction_status_to_string(header.status);
    let status_code = header.status;
    let success = is_transaction_success(header.status);

    // block roots
    let transaction_mroot = Hex::encode(&block_header.transaction_mroot.to_vec());
    let action_mroot = Hex::encode(&block_header.action_mroot.to_vec());

    let row = tables
        .create_row("Transaction", hash)

        .set("block", &clock.id) // pointer to Block
        .set_bigint("index", &index.to_string())
        .set("hash", hash)
        .set_bigint("elapsed", &elapsed.to_string())
        .set_bigint("netUsage", &net_usage.to_string())

        // header
        .set_bigint("cpuUsageMicroSeconds", &cpu_usage_micro_seconds.to_string())
        .set_bigint("netUsageWords", &net_usage_words.to_string())
        .set("status", status)
        .set_bigint("statusCode", &status_code.to_string())
        .set("success", success.to_string())

        // block roots
        .set("transactionMroot", transaction_mroot)
        .set("actionMroot", action_mroot)
        ;

    insert_timestamp(row, clock);

    // first find all receivers in action traces by contract
    let mut previous_receivers = HashMap::new();
    for trace in transaction.action_traces.iter() {
        let action = trace.clone().action.unwrap_or_default();
        let account = action.account;
        let receiver = trace.clone().receiver;
        let mut previous = previous_receivers.get(&account).unwrap_or(&vec![]).clone();
        previous.push(receiver);
        previous_receivers.insert(account, previous);
    }

    // Traces of each action within the transaction, excluding all notifications
    for trace in transaction.action_traces.iter() {
        let action = trace.clone().action.unwrap_or_default();
        let account = action.account;

        // skip if action is an inline notification
        // notifications are not included in the actions table
        // `receivers` table is used to store notifications
        if trace.receiver != account { return; }
        insert_action(tables, trace, transaction);

        // TABLE::receivers
        let receivers = previous_receivers.get(&account).unwrap_or(&vec![]).clone();
        for receiver in receivers.iter() {
            insert_receiver(tables, trace, transaction, receiver);
        }
    }
}
