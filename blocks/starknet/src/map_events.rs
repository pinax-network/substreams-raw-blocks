use common::utils::build_timestamp_with_prefix;
use substreams::{errors::Error, pb::substreams::Clock};

use crate::{
    blocks::collect_block,
    events::collect_events,
    pb::{pinax::starknet::v1::EventsOutput, sf::starknet::r#type::v1::Block},
    transactions::{collect_transaction, extract_fields_from_transaction},
    utils::build_block_hashes,
};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<EventsOutput, Error> {
    let timestamp = build_timestamp_with_prefix(&clock);
    let block_hashes = build_block_hashes(&block);

    let mut events = EventsOutput {
        blocks: vec![collect_block(&block, &timestamp, &block_hashes)],
        transactions: vec![],
        messages_sent: vec![],
        events: vec![],
        calls: vec![],
    };

    for (index, transaction) in block.transactions.iter().enumerate() {
        let tx_data = extract_fields_from_transaction(transaction);

        // messages_sent should be a field in `transaction` when complex arrays are supported by substreams-sink-files
        let (trx, messages_sent) = collect_transaction(&block, transaction, index as u32, &timestamp, &block_hashes, &tx_data);

        events.transactions.push(trx);
        events.messages_sent.extend(messages_sent);
        events.events.extend(collect_events(&block, transaction, &timestamp, &block_hashes, &tx_data));
    }

    Ok(events)
}
