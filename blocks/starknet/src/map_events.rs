use substreams::{errors::Error, pb::substreams::Clock};

use crate::pb::pinax::starknet::v1::{Block, EventsOutput};

#[substreams::handlers::map]
pub fn map_events(_clock: Clock, _block: Block) -> Result<EventsOutput, Error> {
    Ok(EventsOutput::default())
}
