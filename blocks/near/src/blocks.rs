// use common::structs::BlockTimestamp;
// use substreams::Hex;
// use substreams_near::pb::sf::near::r#type::v1::Block;

// use crate::pb::pinax::near::v1::BlockChunk;

// pub fn collect_block_chunk(block: &Block, index: usize, timestamp: &BlockTimestamp) -> BlockChunk {
//     let header = block.header.as_ref().expect("No block header found");
//     let chunk_header = block.chunk_headers.get(index).expect("No chunk header found");

//     let shard = block.shards.get(index).expect("No shard found");
//     let chunk = shard.chunk.as_ref().expect("No chunk found");

//     BlockChunk {
//         block_time: timestamp.time.to_string(),
//         block_height: timestamp.number,
//         block_date: timestamp.date.clone(),
//         block_hash: timestamp.hash.clone(),
//         block_prev_hash: header.prev_hash.as_ref().map_or("".to_string(), |h| Hex::encode(h.bytes.clone())),
//         block_total_supply: header.total_supply.as_ref().unwrap().bytes.clone(),

//         hash: Hex::encode(&chunk_header.chunk_hash),
//         gas_price: header.gas_price.as_ref().unwrap().bytes.clone(),
//         author_account_id: chunk.author.clone(),
//         epoch_id: Hex::encode(header.epoch_id.as_ref().unwrap().bytes.clone()),
//         shard_id: chunk_header.shard_id,
//         signature: Hex::encode(chunk_header.signature.as_ref().unwrap().bytes.clone()),
//         gas_limit: chunk_header.gas_limit,
//         gas_used: chunk_header.gas_used,
//     }
// }
