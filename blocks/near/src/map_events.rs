use substreams_near::pb::sf::near::r#type::v1::Block;

use crate::pb::pinax::near::v1::Events;

pub fn _map_events(_block: &Block) -> Events {
    Events { block_chunks: vec![] }
}
