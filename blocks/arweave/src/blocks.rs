use common::structs::BlockTimestamp;
use substreams::Hex;

use crate::pb::{pinax::arweave::v1::Block as BlockOutput, sf::arweave::r#type::v1::Block};

pub fn collect_block(block: &Block, timestamp: &BlockTimestamp) -> BlockOutput {
    let poa = block.poa.as_ref();

    BlockOutput {
        time: timestamp.time.to_string(),
        height: timestamp.number,
        date: timestamp.date.clone(),
        indep_hash: timestamp.hash.to_string(),
        nonce: block.nonce.to_vec(),
        previous_block: Hex::encode(&block.previous_block),
        timestamp: block.timestamp,
        last_retarget: block.last_retarget,
        diff: block.diff.as_ref().unwrap().bytes.clone(),
        hash: Hex::encode(&block.hash),
        tx_root: Hex::encode(&block.tx_root),
        wallet_list: Hex::encode(&block.wallet_list),
        reward_addr: Hex::encode(&block.reward_addr),
        reward_pool: block.reward_pool.as_ref().unwrap().bytes.clone(),
        weave_size: block.weave_size.as_ref().unwrap().bytes.clone(),
        block_size: block.block_size.as_ref().unwrap().bytes.clone(),
        cumulative_diff: block.cumulative_diff.as_ref().unwrap().bytes.clone(),
        hash_list_merkle: Hex::encode(&block.hash_list_merkle),
        poa_option: poa.map(|p| p.option.clone()),
        poa_tx_path: poa.map(|p| Hex::encode(&p.tx_path)),
        poa_data_path: poa.map(|p| Hex::encode(&p.data_path)),
        poa_chunk: poa.map(|p| Hex::encode(&p.chunk)),
    }
}
