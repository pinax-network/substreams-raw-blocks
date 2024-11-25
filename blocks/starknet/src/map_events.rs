use common::utils::build_timestamp_with_prefix;
use substreams::{errors::Error, pb::substreams::Clock};

use crate::{
    blocks::collect_block,
    pb::{pinax::starknet::v1::EventsOutput, sf::starknet::r#type::v1::Block},
};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<EventsOutput, Error> {
    let timestamp = build_timestamp_with_prefix(&clock);

    let mut events = EventsOutput {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: vec![],
        access_lists: vec![],
        events: vec![],
        calls: vec![],
    };

    Ok(events)
}
