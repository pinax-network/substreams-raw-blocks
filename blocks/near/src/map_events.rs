// use common::utils::build_timestamp;
// use substreams::pb::substreams::Clock;
// use substreams_near::pb::sf::near::r#type::v1::Block;

// use crate::{blocks::collect_block_chunk, pb::pinax::near::v1::Events};

// #[substreams::handlers::map]
// pub fn map_events(clock: Clock, block: Block) -> Events {
//     let timestamp = build_timestamp(&clock);

//     let mut events = Events { block_chunks: vec![] };

//     for i in 0..block.chunk_headers.len() {
//         events.block_chunks.push(collect_block_chunk(&block, i, &timestamp));
//     }

//     events
// }
