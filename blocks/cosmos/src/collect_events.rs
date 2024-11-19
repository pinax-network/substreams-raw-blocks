use common::utils::build_timestamp;
use substreams::pb::substreams::Clock;
use substreams_cosmos::Block;

use crate::{
    blocks::collect_block,
    consensus_param_updates::collect_consensus_params,
    misbehaviors::collect_misbehaviors,
    pb::cosmos::{Events as RawEvents, Transaction as RawTransaction, TransactionEvent as RawTransactionEvent, TransactionMessage},
    transaction_messages::collect_tx_transaction_messages,
    transactions::collect_transaction,
    tx_and_block_events::{collect_block_events, collect_transaction_events},
    utils::compute_tx_hash,
    validator_updates::collect_validator_updates,
};

pub fn collect_events(block: &Block, clock: &Clock) -> RawEvents {
    let timestamp = build_timestamp(&clock);

    let mut transactions: Vec<RawTransaction> = vec![];
    let mut transaction_events: Vec<RawTransactionEvent> = vec![];
    let mut transaction_messages: Vec<TransactionMessage> = vec![];

    for (i, tx_result) in block.tx_results.iter().enumerate() {
        let tx_hash = compute_tx_hash(block.txs.get(i).unwrap());

        transactions.push(collect_transaction(tx_result, &tx_hash, &timestamp, i));
        transaction_events.extend(collect_transaction_events(tx_result, &tx_hash, &timestamp));
        transaction_messages.extend(collect_tx_transaction_messages(block, i, &tx_hash, &timestamp));
    }

    RawEvents {
        blocks: vec![collect_block(block, &timestamp)],
        transactions,
        transaction_events,
        block_events: collect_block_events(block, &timestamp),
        misbehaviors: collect_misbehaviors(block, &timestamp),
        validator_updates: collect_validator_updates(block, &timestamp),
        consensus_param_updates: collect_consensus_params(block, &timestamp),
        transaction_messages,
    }
}
