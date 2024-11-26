use common::utils::build_timestamp_with_prefix;
use substreams::{errors::Error, pb::substreams::Clock};

use crate::{
    blocks::collect_block,
    pb::{pinax::starknet::v1::EventsOutput, sf::starknet::r#type::v1::Block},
    transactions::collect_transaction,
    utils::build_block_hashes,
};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<EventsOutput, Error> {
    let timestamp = build_timestamp_with_prefix(&clock);
    let block_hashes = build_block_hashes(&block);

    let mut events = EventsOutput {
        blocks: vec![collect_block(&block, &timestamp, &block_hashes)],
        transactions: vec![],
        access_lists: vec![],
        events: vec![],
        calls: vec![],
    };

    for (index, transaction) in block.transactions.iter().enumerate() {
        events.transactions.push(collect_transaction(&block, transaction, index as u32, &timestamp, &block_hashes));
    }

    Ok(events)
}
