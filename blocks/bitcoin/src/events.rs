use substreams::{errors::Error, pb::substreams::Clock};
use substreams_bitcoin::pb::btc::v1::Block;

use crate::pb::bitcoin::Events;

#[substreams::handlers::map]
pub fn map_events(_block: Block, _clock: Clock) -> Result<Events, Error> {
    Ok(Events::default())
}
