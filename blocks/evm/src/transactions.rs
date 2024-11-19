use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use common::utils::optional_bigint_to_string;
use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::evm::Transaction;

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

pub fn collect_transactions(block: &Block, timestamp: &BlockTimestamp) -> Vec<Transaction> {
    let block_header = block.header.as_ref().unwrap();

    block
        .transaction_traces
        .iter()
        .map(|transaction| {
            let receipt = transaction.receipt.clone().unwrap();
            Transaction {
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
