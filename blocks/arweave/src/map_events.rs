use common::utils::build_timestamp;
use substreams::pb::substreams::Clock;

use crate::{
    blocks::collect_block,
    pb::{pinax::arweave::v1::Events, sf::arweave::r#type::v1::Block},
};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Events {
    let timestamp = build_timestamp(&clock);

    Events {
        blocks: vec![collect_block(&block, &timestamp)],
    }
}
