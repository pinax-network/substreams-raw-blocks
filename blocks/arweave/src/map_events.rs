use common::utils::build_timestamp;
use substreams::pb::substreams::Clock;

use crate::{
    blocks::collect_block,
    pb::{pinax::arweave::v1::Events, sf::arweave::r#type::v1::Block},
    transactions::collect_transaction,
};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Events {
    let timestamp = build_timestamp(&clock);

    let mut events = Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: vec![],
        transaction_tags: vec![],
    };

    for (index, transaction) in block.txs.iter().enumerate() {
        let (tx, tags) = collect_transaction(transaction, &timestamp, index);
        events.transactions.push(tx);
        events.transaction_tags.extend(tags);
    }

    events
}
