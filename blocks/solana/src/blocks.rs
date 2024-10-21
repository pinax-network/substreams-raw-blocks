use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges, TableChange};
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{account_activity::insert_account_activity, counters::insert_block_counters, rewards::insert_rewards, transactions::insert_transactions, utils::insert_timestamp_without_number};

static VOTE_ACCOUNT_KEY: &str = "Vote111111111111111111111111111111111111111";

pub fn insert_blocks(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, table_prefix: &str) {
    let row = tables.push_change("blocks", block.blockhash.as_str(), 0, table_change::Operation::Create);

    insert_blockinfo(row, block, false);
    insert_timestamp_without_number(row, clock, true, false);
    insert_block_counters(row, block);

    // TABLE::rewards
    insert_rewards(tables, clock, block);

    // Filter out vote transactions
    // Original index before filter is preserved
    let non_vote_trx: Vec<(&ConfirmedTransaction, usize)> = block
        .transactions
        .iter()
        .enumerate()
        .filter(|(_, trx)| {
            let message = trx.transaction.as_ref().unwrap().message.as_ref().unwrap();
            !message.account_keys.iter().any(|key| base58::encode(key) == VOTE_ACCOUNT_KEY)
        })
        .map(|(index, trx)| (trx, index))
        .collect();

    // TABLE::transactions
    insert_transactions(tables, clock, block, &non_vote_trx, table_prefix);

    // TABLE::account_activity
    insert_account_activity(tables, clock, block, &non_vote_trx);
}

pub fn insert_blockinfo(row: &mut TableChange, block: &Block, with_prefix: bool) {
    let height = block.block_height.as_ref().map_or_else(|| "0".to_string(), |bh| bh.block_height.to_string());

    let prefix_str = if with_prefix { "block_" } else { "" };
    row.change(format!("{}slot", prefix_str).as_str(), ("", block.slot.to_string().as_str()))
        .change(format!("{}height", prefix_str).as_str(), ("", height.as_str()))
        .change(format!("{}previous_block_hash", prefix_str).as_str(), ("", block.previous_blockhash.as_str()))
        .change(format!("{}parent_slot", prefix_str).as_str(), ("", block.parent_slot.to_string().as_str()));
}
