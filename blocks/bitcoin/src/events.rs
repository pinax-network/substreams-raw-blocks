use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_bitcoin::pb::btc::v1::{Block, Transaction as TransactionSource};

use crate::pb::pinax::bitcoin::v2::{Block as BlockRow, Events, Input, Output, Transaction};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let height = block.height.max(0) as u64;
    let id = BlockIdentity::new(height, hex_to_bytes(&block.hash), height.saturating_sub(1), hex_to_bytes(&block.previous_hash), &clock.timestamp);

    let mut events = Events {
        blocks: vec![collect_block(&block, &id)],
        transactions: Vec::with_capacity(block.tx.len()),
        inputs: vec![],
        outputs: vec![],
    };
    for (index, tx) in block.tx.iter().enumerate() {
        let index = index as u32;
        events.transactions.push(collect_transaction(&block, tx, index, &id));
        events.inputs.extend(collect_inputs(&block, tx, index, &id));
        events.outputs.extend(collect_outputs(&block, tx, index, &id));
    }
    Ok(events)
}

fn hex_to_bytes(value: &str) -> Vec<u8> {
    hex::decode(value).unwrap_or_else(|_| value.as_bytes().to_vec())
}

fn is_coinbase(tx: &TransactionSource) -> bool {
    tx.vin.iter().any(|vin| !vin.coinbase.is_empty())
}

// Block subsidy schedule (halving every 210,000 blocks), in BTC.
fn mint_reward(height: i64) -> f64 {
    let halvings = height / 210_000;
    if halvings >= 64 {
        return 0.0;
    }
    50.0 / 2_f64.powi(halvings as i32)
}

fn collect_block(block: &Block, id: &BlockIdentity) -> BlockRow {
    let coinbase_tx = block.tx.iter().find(|tx| is_coinbase(tx));
    let total_reward: f64 = coinbase_tx.map(|tx| tx.vout.iter().map(|v| v.value).sum()).unwrap_or(0.0);
    let mint_reward = mint_reward(block.height);

    BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        hash: block.hash.clone(),
        height: block.height,
        previous_hash: block.previous_hash.clone(),
        merkle_root: block.merkle_root.clone(),
        time: block.time,
        nonce: block.nonce,
        bits: block.bits.clone(),
        difficulty: block.difficulty,
        size: block.size,
        stripped_size: block.stripped_size,
        weight: block.weight,
        version: block.version,
        n_tx: block.n_tx,
        mediantime: block.mediantime,
        chainwork: block.chainwork.clone(),

        version_hex: block.version_hex.clone(),
        coinbase: coinbase_tx.and_then(|tx| tx.vin.first()).map(|v| v.coinbase.clone()).unwrap_or_default(),
        total_reward,
        mint_reward,
        total_fees: total_reward - mint_reward,
        num_inputs: block.tx.iter().map(|tx| tx.vin.len()).sum::<usize>() as u32,
        num_outputs: block.tx.iter().map(|tx| tx.vout.len()).sum::<usize>() as u32,
    }
}

fn collect_transaction(block: &Block, tx: &TransactionSource, index: u32, id: &BlockIdentity) -> Transaction {
    Transaction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        txid: tx.txid.clone(),
        hash: tx.hash.clone(),
        size: tx.size,
        vsize: tx.vsize,
        weight: tx.weight,
        version: tx.version,
        locktime: tx.locktime,
        block_hash: block.hash.clone(),
        block_height: block.height,
        block_time: block.time,
        tx_index: index,

        is_coinbase: is_coinbase(tx),
        hex: tx.hex.clone(),
        num_inputs: tx.vin.len() as u32,
        num_outputs: tx.vout.len() as u32,
        output_value: tx.vout.iter().map(|v| v.value).sum(),
    }
}

fn collect_inputs(block: &Block, tx: &TransactionSource, index: u32, id: &BlockIdentity) -> Vec<Input> {
    tx.vin
        .iter()
        .enumerate()
        .map(|(input_index, vin)| {
            let script_sig = vin.script_sig.clone().unwrap_or_default();
            Input {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                tx_hash: tx.txid.clone(),
                block_height: block.height,
                input_index: input_index as u32,
                prev_txid: vin.txid.clone(),
                prev_vout: vin.vout,
                sequence: vin.sequence,
                script_sig_asm: script_sig.asm.clone(),
                script_sig_hex: script_sig.hex.clone(),
                coinbase: vin.coinbase.clone(),
                witness: vin.txinwitness.clone(),

                tx_index: index,
                is_coinbase: !vin.coinbase.is_empty(),
            }
        })
        .collect()
}

fn collect_outputs(block: &Block, tx: &TransactionSource, index: u32, id: &BlockIdentity) -> Vec<Output> {
    tx.vout
        .iter()
        .map(|vout| {
            let script = vout.script_pub_key.clone().unwrap_or_default();
            Output {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                tx_hash: tx.txid.clone(),
                block_height: block.height,
                output_index: vout.n,
                value: vout.value,
                script_pubkey_asm: script.asm.clone(),
                script_pubkey_hex: script.hex.clone(),
                script_pubkey_type: script.r#type.clone(),
                script_pubkey_address: script.address.clone(),

                tx_index: index,
                req_sigs: script.req_sigs,
                script_pubkey_addresses: script.addresses.clone(),
            }
        })
        .collect()
}
