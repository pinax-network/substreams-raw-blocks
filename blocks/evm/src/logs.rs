use common::{block_time_to_date, bytes_to_hex, extract_topic};
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;

pub fn insert_logs(tables: &mut Tables, clock: &Clock, block: &Block) {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_time = timestamp.to_string();
    let block_number = clock.number.to_string();
    let block_hash = format!("0x{}", clock.id);
    let block_date = block_time_to_date(block_time.as_str());

    // logs
    for log in block.logs() {
        let log_index = log.index();
        let transaction = log.receipt.transaction;
        let tx_hash = bytes_to_hex(transaction.hash.to_vec());
        let tx_index = transaction.index;
        let tx_from = bytes_to_hex(transaction.from.to_vec());
        let tx_to = bytes_to_hex(transaction.to.to_vec());
        let contract_address = bytes_to_hex(log.address().to_vec());
        let topics = log.topics();
        let topic0 = extract_topic(topics, 0);
        let topic1 = extract_topic(topics, 1);
        let topic2 = extract_topic(topics, 2);
        let topic3 = extract_topic(topics, 3);
        let data = bytes_to_hex(log.data().to_vec());

        tables
            .create_row("logs", &log_index.to_string())
            .set("block_time", &block_time)
            .set("block_number", &block_number)
            .set("block_hash", &block_hash)
            .set("contract_address", &contract_address)
            .set("topic0", &topic0)
            .set("topic1", &topic1)
            .set("topic2", &topic2)
            .set("topic3", &topic3)
            .set("data", &data)
            .set("tx_hash", &tx_hash)
            .set_bigint("index", &log_index.to_string())
            .set_bigint("tx_index", &tx_index.to_string())
            .set("block_date", &block_date)
            .set("tx_from", &tx_from)
            .set("tx_to", &tx_to);
    }
}