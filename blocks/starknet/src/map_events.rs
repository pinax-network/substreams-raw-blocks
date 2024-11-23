use substreams::{errors::Error, pb::substreams::Clock};

use crate::pb::{pinax::starknet::EventsOutput, sf::starknet::r#type::v1::Block};

#[substreams::handlers::map]
pub fn map_events(_clock: Clock, _block: Block) -> Result<EventsOutput, Error> {
    Ok(EventsOutput::default())
}
