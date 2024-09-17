use common::blocks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_antelope::pb::{BlockHeader, TransactionTrace};
use substreams_database_change::pb::database::TableChange;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::feature_ops::insert_feature_op;
use crate::keys::transactions_keys;
use crate::ram_ops::insert_ram_op;
use crate::perm_ops::insert_perm_op;

use super::actions::insert_action;
use super::db_ops::insert_db_op;

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

    // TABLE::transactions
    let keys = transactions_keys(clock, hash);
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
        .change("transaction_mroot", ("", transaction_mroot.as_str()));
    insert_timestamp(row, clock, false, false);

    // TABLE::actions
    for trace in transaction.action_traces.iter() {
        insert_action(tables, clock, trace, transaction, block_header);
    }

    // TABLE::db_ops
    let mut db_op_index = 0;
    for db_op in transaction.db_ops.iter() {
        insert_db_op(tables, clock, db_op, transaction, db_op_index);
        db_op_index += 1;
    }

    // TO-DO
    // Trace of a failed deferred transaction, if any.
    // match transaction.failed_dtrx_trace {
    //     Some(failed_dtrx_trace) => {
    //         insert_transaction(tables, clock, &failed_dtrx_trace, &block);
    //     }
    //     None => {}
    // }

    // TO-DO
    // List of deferred transactions operations this transaction entailed
    // for db_op in transaction.dtrx_ops.iter() {
    //     insert_db_op(tables, clock, db_op, &transaction, &block);
    // }

    // TO-DO
    // List of feature switching operations (changes to feature switches in nodeos) this transaction entailed
    for feature_op in transaction.feature_ops.iter() {
        insert_feature_op(tables, clock, feature_op, transaction);
    }

    // TO-DO
    // List of permission changes operations
    for perm_op in transaction.perm_ops.iter() {
        insert_perm_op(tables, clock, transaction, perm_op);
    }

    // TO-DO
    // List of RAM consumption/redemption
    for ram_op in transaction.ram_ops.iter() {
        insert_ram_op(tables, clock, ram_op, transaction);
    }

    // TO-DO
    // List of RAM correction operations (happens only once upon feature activation)
    // for ram_correction_op in transaction.ram_correction_ops.iter() {
    //     insert_ram_correction_op(tables, clock, ram_correction_op, &block);
    // }

    // TO-DO
    // List of changes to rate limiting values
    // for rlimit_op in transaction.rlimit_ops.iter() {
    //     insert_rlimit_op(tables, clock, rlimit_op, &block);
    // }

    // TO-DO
    // List of table creations/deletions
    // for table_op in transaction.table_ops.iter() {
    //     insert_table_op(tables, clock, table_op, &block);
    // }

    // TO-DO
    // Tree of creation, rather than execution
    // for creation_tree in transaction.creation_tree.iter() {
    //     insert_creation_tree(tables, clock, creation_tree, &block);
    // }

    // TO-DO??
    // Exception leading to the failed dtrx trace.
    // let exception = transaction.exception;
    // let error_code = transaction.error_code;
    // match exception {
    //     Some(exception) => {
    //         let exception_code = exception.code;
    //         let exception_name = exception.name;
    //         let exception_message = exception.message;
    //     }
    //     None => {}
    // }
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
