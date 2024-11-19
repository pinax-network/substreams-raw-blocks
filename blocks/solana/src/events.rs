use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::collect_events::{collect_events_with_votes, collect_events_without_votes};
use crate::pb::solana::Events;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    Ok(collect_events_without_votes(&block, &clock))
}

#[substreams::handlers::map]
pub fn map_events_with_votes(clock: Clock, block: Block) -> Result<Events, Error> {
    Ok(collect_events_with_votes(&block, &clock))
}
