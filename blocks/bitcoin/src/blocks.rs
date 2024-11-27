use common::structs::BlockTimestamp;
use substreams_bitcoin::pb::btc::v1::Block;

use crate::pb::pinax::bitcoin::v1::Block as OutputBlock;

pub fn collect_block(block: &Block, timestamp: &BlockTimestamp) -> OutputBlock {
    // Get the coinbase from the first transaction
    let coinbase = block.tx.first().unwrap().vin.first().unwrap().coinbase.clone();

    OutputBlock {
        // clock
        time: timestamp.time.to_string(),
        height: block.height as u32,
        date: timestamp.date.clone(),
        hash: block.hash.clone(),

        // block
        bits: block.bits.clone(),
        chainwork: block.chainwork.clone(),
        difficulty: block.difficulty,
        merkle_root: block.merkle_root.clone(),
        nonce: block.nonce,
        coinbase,
        previous_block_hash: block.previous_hash.clone(),
        version: block.version,
        weight: block.weight,

        // counters
        size: block.size,
        stripped_size: block.stripped_size,
        transaction_count: block.tx.len() as u64,
        total_fees: calculate_total_fees(block),
        total_reward: calculate_total_reward(block),
        mint_reward: calculate_mint_reward(block.height),
        total_inputs: block.tx.iter().map(|tx| tx.vin.len() as u32).sum(),
        total_outputs: block.tx.iter().map(|tx| tx.vout.len() as u32).sum(),
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
