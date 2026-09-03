use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt as ScalarBigInt;

use crate::pb::pinax::arweave::v2::{Block as BlockRow, BlockTag, Events, Transaction, TransactionTag};
use crate::pb::sf::arweave::r#type::v1::{BigInt, Block};

fn bigint_to_string(value: &Option<BigInt>) -> String {
    match value {
        Some(b) if !b.bytes.is_empty() => ScalarBigInt::from_unsigned_bytes_be(&b.bytes).to_string(),
        _ => "0".to_string(),
    }
}

fn lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let id = BlockIdentity::new(block.height, block.indep_hash.clone(), block.height.saturating_sub(1), block.previous_block.clone(), &clock.timestamp);
    let poa = block.poa.as_ref();

    let mut events = Events::default();
    events.blocks.push(BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        height: block.height,
        indep_hash: block.indep_hash.clone(),
        hash: block.hash.clone(),
        previous_block: block.previous_block.clone(),
        ver: block.ver,
        nonce: block.nonce.clone(),
        block_timestamp: block.timestamp,
        last_retarget: block.last_retarget,
        diff: bigint_to_string(&block.diff),
        cumulative_diff: bigint_to_string(&block.cumulative_diff),
        tx_root: block.tx_root.clone(),
        wallet_list: block.wallet_list.clone(),
        reward_addr: block.reward_addr.clone(),
        reward_pool: bigint_to_string(&block.reward_pool),
        weave_size: bigint_to_string(&block.weave_size),
        block_size: bigint_to_string(&block.block_size),
        hash_list_merkle: block.hash_list_merkle.clone(),
        poa_option: poa.map(|p| p.option.clone()),
        poa_tx_path: poa.map(|p| p.tx_path.clone()),
        poa_data_path: poa.map(|p| p.data_path.clone()),
        poa_chunk: poa.map(|p| p.chunk.clone()),
        num_transactions: block.txs.len() as u32,
        num_tags: block.tags.len() as u32,
    });

    events.block_tags = block
        .tags
        .iter()
        .enumerate()
        .map(|(index, tag)| BlockTag {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),
            index: index as u32,
            name: lossy(&tag.name),
            value: lossy(&tag.value),
            name_bytes: tag.name.clone(),
            value_bytes: tag.value.clone(),
        })
        .collect();

    for (index, tx) in block.txs.iter().enumerate() {
        let index = index as u32;
        events.transactions.push(Transaction {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            id: tx.id.clone(),
            index,
            format: tx.format,
            last_tx: tx.last_tx.clone(),
            owner: tx.owner.clone(),
            target: tx.target.clone(),
            quantity: bigint_to_string(&tx.quantity),
            data: tx.data.clone(),
            data_size: bigint_to_string(&tx.data_size),
            data_root: tx.data_root.clone(),
            signature: tx.signature.clone(),
            reward: bigint_to_string(&tx.reward),
            num_tags: tx.tags.len() as u32,
        });
        events.transaction_tags.extend(tx.tags.iter().enumerate().map(|(tag_index, tag)| TransactionTag {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),
            tx_id: tx.id.clone(),
            tx_index: index,
            index: tag_index as u32,
            name: lossy(&tag.name),
            value: lossy(&tag.value),
            name_bytes: tag.name.clone(),
            value_bytes: tag.value.clone(),
        }));
    }
    Ok(events)
}
