use common::utils::build_timestamp;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::collect_events::collect_events;
use crate::pb::antelope::Events;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);
    Ok(collect_events(&block, &timestamp))
}
