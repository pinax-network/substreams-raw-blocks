use common::utils::build_timestamp;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_bitcoin::pb::btc::v1::Block;

use crate::{blocks::collect_block, pb::bitcoin::Events};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);

    let events = Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    Ok(events)
}
