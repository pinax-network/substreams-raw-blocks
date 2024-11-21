use common::structs::BlockTimestamp;
use substreams_bitcoin::pb::btc::v1::Block;

use crate::pb::bitcoin::Block as OutputBlock;

pub fn collect_block(block: &Block, timestamp: &BlockTimestamp) -> OutputBlock {
    OutputBlock {
        time: Some(timestamp.time),
        height: block.height as u32,
        date: timestamp.date.clone(),
        hash: block.hash.clone(),
        bits: block.bits.clone(),
        chainwork: block.chainwork.clone(),
        difficulty: block.difficulty,
        total_fees: calculate_total_fees(block),
        total_reward: calculate_total_reward(block),
        mint_reward: calculate_mint_reward(block.height),
        merkle_root: block.merkle_root.clone(),
        transaction_count: block.tx.len() as u64,
        nonce: block.nonce as u32,
        // Get the coinbase from the first transaction
        coinbase: block.tx.first().unwrap().vin.first().unwrap().coinbase.clone(),
        previous_block_hash: block.previous_hash.clone(),
        size: block.size,
        stripped_size: block.stripped_size,
        version: block.version,
        weight: block.weight,
    }
}

fn calculate_total_fees(block: &Block) -> f64 {
    let total_reward = calculate_total_reward(block);
    let mint_reward = calculate_mint_reward(block.height);

    total_reward - mint_reward
}

fn calculate_total_reward(block: &Block) -> f64 {
    // Assume the first transaction is the coinbase transaction
    if let Some(coinbase_tx) = block.tx.first() {
        coinbase_tx.vout.iter().map(|vout| vout.value).sum()
    } else {
        0.0 // No transactions in the block (shouldn't happen in practice)
    }
}

fn calculate_mint_reward(height: i64) -> f64 {
    // Bitcoin's block reward halving schedule
    let halvings = height / 210_000;
    if halvings >= 64 {
        return 0.0; // All bitcoins mined
    }

    50.0 / 2_f64.powi(halvings as i32)
}
