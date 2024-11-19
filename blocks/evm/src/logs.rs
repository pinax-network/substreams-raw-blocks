use common::structs::BlockTimestamp;
use common::utils::{bytes_to_hex, extract_topic};
use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::evm::Log;
use crate::transactions::{is_transaction_success, transaction_status_to_string};

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L512
// DetailLevel: BASE (only successful transactions) & EXTENDED
pub fn collect_logs(block: &Block, timestamp: &BlockTimestamp, detail_level: &str) -> Vec<Log> {
    // Only required DetailLevel=BASE since traces are not available in BASE
    if detail_level == "Base" {
        return vec![];
    }

    let mut logs: Vec<Log> = vec![];

    for transaction in &block.transaction_traces {
        let receipt = transaction.receipt.as_ref().unwrap();
        for log in &receipt.logs {
            logs.push(Log {
                block_time: Some(timestamp.time),
                block_number: timestamp.number,
                block_hash: timestamp.hash.clone(),
                block_date: timestamp.date.clone(),
                tx_hash: bytes_to_hex(&transaction.hash),
                tx_index: transaction.index,
                tx_status: transaction_status_to_string(transaction.status),
                tx_status_code: transaction.status as u32,
                tx_success: is_transaction_success(transaction.status),
                tx_from: bytes_to_hex(&transaction.from),
                tx_to: bytes_to_hex(&transaction.to),
                index: log.index,
                block_index: log.block_index,
                contract_address: bytes_to_hex(&log.address),
                topic0: extract_topic(&log.topics, 0),
                topic1: extract_topic(&log.topics, 1),
                topic2: extract_topic(&log.topics, 2),
                topic3: extract_topic(&log.topics, 3),
                data: bytes_to_hex(&log.data),
            });
        }
    }

    logs
}
