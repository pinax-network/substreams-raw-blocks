use substreams::{errors::Error, pb::substreams::Clock};
use substreams_bitcoin::pb::btc::v1::Block;

use crate::pb::bitcoin::Events;

#[substreams::handlers::map]
pub fn map_events(block: Block, clock: Clock) -> Result<Events, Error> {
    Ok(Events::default())
}
