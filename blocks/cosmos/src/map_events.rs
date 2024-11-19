use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;

use crate::{collect_events::collect_events, pb::cosmos::Events};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    Ok(collect_events(&block, &clock))
}
