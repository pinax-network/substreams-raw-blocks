use common::blocks::insert_timestamp;
use common::keys::logs_keys;
use common::utils::{bytes_to_hex, extract_topic};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::{Log, TransactionTrace};

use crate::transactions::insert_transaction_metadata;

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L512
// DetailLevel: BASE (only successful transactions) & EXTENDED
pub fn insert_log(tables: &mut DatabaseChanges, clock: &Clock, log: &Log, transaction: &TransactionTrace) {
    let index = log.index;
    let block_index = log.block_index;
    let tx_hash = bytes_to_hex(&transaction.hash.to_vec());
    let contract_address = bytes_to_hex(&log.address.to_vec()); // EVM Address
    let topics = log.topics.clone();
    let topic0 = extract_topic(&topics, 0);
    let topic1 = extract_topic(&topics, 1);
    let topic2 = extract_topic(&topics, 2);
    let topic3 = extract_topic(&topics, 3);
    let data = bytes_to_hex(&log.data.to_vec());

    // Missing
    // - blob_gas_price
    // - blob_gas_used

    let keys = logs_keys(&clock, &tx_hash, &index);
    let row = tables
        .push_change_composite("logs", keys, 0, table_change::Operation::Create)
        .change("index", ("", index.to_string().as_str()))
        .change("block_index", ("", block_index.to_string().as_str()))
        .change("contract_address", ("", contract_address.as_str()))
        .change("topic0", ("", topic0.as_str()))
        .change("topic1", ("", topic1.as_str()))
        .change("topic2", ("", topic2.as_str()))
        .change("topic3", ("", topic3.as_str()))
        .change("data", ("", data.as_str()));

    insert_timestamp(row, clock, false);
    insert_transaction_metadata(row, transaction, false);
}
