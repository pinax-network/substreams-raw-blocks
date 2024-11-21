use std::collections::HashMap;

use common::utils::build_timestamp;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_bitcoin::pb::btc::v1::{Block, Vout};

use crate::{blocks::collect_block, pb::bitcoin::Events, transactions::collect_transaction};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);

    let mut events = Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    for (index, transaction) in block.tx.iter().enumerate() {
        events.transactions.push(collect_transaction(transaction, &timestamp, index as u32));
    }

    Ok(events)
}

fn build_utxo_map(block: &Block) -> HashMap<String, Vec<Vout>> {
    let mut utxo_map = HashMap::new();
    for transaction in &block.tx {
        utxo_map.insert(transaction.txid.clone(), transaction.vout.clone());
    }
    utxo_map
}
