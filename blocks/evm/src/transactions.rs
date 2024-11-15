use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use common::utils::optional_bigint_to_string;
use substreams_database_change::pb::database::TableChange;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::eth::v2::TransactionTrace;

use crate::pb::evm::Transaction as RawTransaction;

pub fn transaction_type_to_string(r#type: i32) -> String {
    match r#type {
        0 => "Legacy".to_string(),
        1 => "AccessList".to_string(),
        2 => "DynamicFee".to_string(),
        3 => "Blob".to_string(),
        100 => "ArbitrumDeposit".to_string(),
        101 => "ArbitrumUnsigned".to_string(),
        102 => "ArbitrumContract".to_string(),
        104 => "ArbitrumRetry".to_string(),
        105 => "ArbitrumSubmitRetryable".to_string(),
        106 => "ArbitrumInternal".to_string(),
        120 => "ArbitrumLegacy".to_string(),
        126 => "OptimismDeposit".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn transaction_status_to_string(status: i32) -> String {
    match status {
        0 => "Unknown".to_string(),
        1 => "Succeeded".to_string(),
        2 => "Failed".to_string(),
        3 => "Reverted".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn is_transaction_success(status: i32) -> bool {
    status == 1
}

pub fn collect_transactions(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawTransaction> {
    let block_header = block.header.as_ref().unwrap();

    block
        .transaction_traces
        .iter()
        .map(|transaction| {
            let receipt = transaction.receipt.clone().unwrap();
            RawTransaction {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                transactions_root: bytes_to_hex(&block_header.transactions_root),
                receipts_root: bytes_to_hex(&block_header.receipt_root),
                index: transaction.index,
                hash: bytes_to_hex(&transaction.hash),
                from: bytes_to_hex(&transaction.from),
                to: bytes_to_hex(&transaction.to),
                nonce: transaction.nonce,
                status: transaction_status_to_string(transaction.status),
                status_code: transaction.status as u32,
                success: is_transaction_success(transaction.status),
                gas_price: optional_bigint_to_string(&transaction.gas_price, "0"),
                gas_limit: transaction.gas_limit,
                value: optional_bigint_to_string(&transaction.value, "0"),
                data: bytes_to_hex(&transaction.input),
                v: bytes_to_hex(&transaction.v),
                r: bytes_to_hex(&transaction.r),
                s: bytes_to_hex(&transaction.s),
                gas_used: transaction.gas_used,
                r#type: transaction_type_to_string(transaction.r#type),
                type_code: transaction.r#type as u32,
                max_fee_per_gas: optional_bigint_to_string(&transaction.max_fee_per_gas, "0"),
                max_priority_fee_per_gas: optional_bigint_to_string(&transaction.max_priority_fee_per_gas, "0"),
                begin_ordinal: transaction.begin_ordinal,
                end_ordinal: transaction.end_ordinal,
                blob_gas_price: optional_bigint_to_string(&receipt.blob_gas_price, "0"),
                blob_gas_used: receipt.blob_gas_used(),
                cumulative_gas_used: receipt.cumulative_gas_used,
                logs_bloom: bytes_to_hex(&receipt.logs_bloom),
                state_root: bytes_to_hex(&receipt.state_root),
            }
        })
        .collect()
}

pub fn insert_transaction_metadata(row: &mut TableChange, transaction: &TransactionTrace, is_transaction: bool) {
    let tx_hash = bytes_to_hex(&transaction.hash);
    let tx_index = transaction.index;
    let from = bytes_to_hex(&transaction.from); // does trace contain `from`?
    let to = bytes_to_hex(&transaction.to); // does trace contain `to`?
    let tx_status = transaction_status_to_string(transaction.status);
    let tx_status_code = transaction.status;
    let tx_success = is_transaction_success(transaction.status);
    let prefix = if is_transaction { "" } else { "tx_" };

    row.change("tx_hash", ("", tx_hash.as_str()))
        .change("tx_index", ("", tx_index.to_string().as_str()))
        .change(format!("{}from", prefix).as_str(), ("", from.as_str()))
        .change(format!("{}to", prefix).as_str(), ("", to.as_str()))
        .change("tx_status", ("", tx_status.as_str()))
        .change("tx_status_code", ("", tx_status_code.to_string().as_str()))
        .change("tx_success", ("", tx_success.to_string().as_str()));
}

pub fn insert_empty_transaction_metadata(row: &mut TableChange, is_transaction: bool) {
    let tx_hash = "";
    let tx_index = 0;
    let from = "";
    let to = "";
    let tx_status = transaction_status_to_string(1);
    let tx_status_code = 1;
    let tx_success = true;
    let prefix = if is_transaction { "" } else { "tx_" };

    row.change("tx_hash", ("", tx_hash))
        .change("tx_index", ("", tx_index.to_string().as_str()))
        .change(format!("{}from", prefix).as_str(), ("", from))
        .change(format!("{}to", prefix).as_str(), ("", to))
        .change("tx_status", ("", tx_status.as_str()))
        .change("tx_status_code", ("", tx_status_code.to_string().as_str()))
        .change("tx_success", ("", tx_success.to_string().as_str()));
}
