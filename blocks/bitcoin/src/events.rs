use common::utils::build_timestamp;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_bitcoin::pb::btc::v1::Block;

use crate::{blocks::collect_block, inputs::collect_transaction_inputs, outputs::collect_transaction_outputs, pb::pinax::bitcoin::v1::Events, transactions::collect_transaction};

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
        events.inputs.extend(collect_transaction_inputs(transaction, &timestamp));
        events.outputs.extend(collect_transaction_outputs(transaction, &timestamp));
    }

    Ok(events)
}
