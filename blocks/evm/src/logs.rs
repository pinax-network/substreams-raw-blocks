use std::collections::HashMap;

use common::utils::{bytes_to_hex, extract_topic};
use common::sinks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::Block;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L512
pub fn insert_logs(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    // logs
    for log in block.logs() {
        let log_index = log.index().to_string();
        let transaction = log.receipt.transaction;
        let tx_hash = bytes_to_hex(transaction.hash.to_vec());
        let tx_index = transaction.index.to_string();
        let tx_from = bytes_to_hex(transaction.from.to_vec());
        let tx_to = bytes_to_hex(transaction.to.to_vec());
        let contract_address = bytes_to_hex(log.address().to_vec());
        let topics = log.topics();
        let topic0 = extract_topic(topics, 0);
        let topic1 = extract_topic(topics, 1);
        let topic2 = extract_topic(topics, 2);
        let topic3 = extract_topic(topics, 3);
        let data = bytes_to_hex(log.data().to_vec());

        let keys = HashMap::from([
            ("contract_address".to_string(), contract_address.to_string()),
            ("tx_hash".to_string(), tx_hash.to_string()),
            ("log_index".to_string(), log_index.to_string()),
        ]);
        let row = tables
            .push_change_composite("logs", keys, 0, table_change::Operation::Create)
            .change("contract_address", ("", contract_address.as_str()))
            .change("topic0", ("", topic0.as_str()))
            .change("topic1", ("", topic1.as_str()))
            .change("topic2", ("", topic2.as_str()))
            .change("topic3", ("", topic3.as_str()))
            .change("data", ("", data.as_str()))
            .change("tx_hash", ("", tx_hash.as_str()))
            .change("index", ("", log_index.as_str()))
            .change("tx_index", ("", tx_index.as_str()))
            .change("tx_from", ("", tx_from.as_str()))
            .change("tx_to", ("", tx_to.as_str()));

        insert_timestamp(row, clock, true);
    }
}