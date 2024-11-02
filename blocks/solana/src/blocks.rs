use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges, TableChange};
use substreams_solana::{
    b58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{account_activity::insert_account_activity, counters::insert_block_counters, rewards::insert_rewards, transactions::insert_transactions, utils::insert_timestamp_without_number};

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

pub fn insert_blocks(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    let row = tables.push_change("blocks", block.blockhash.as_str(), 0, table_change::Operation::Create);

    insert_blockinfo(row, block, false);
    insert_timestamp_without_number(row, clock, true, false);
    insert_block_counters(row, block);

    // TABLE::rewards
    insert_rewards(tables, clock, block);

    // Separate transactions into vote and non-vote arrays
    // Original index before separation is preserved
    let (non_vote_trx, vote_trx): (Vec<(usize, &ConfirmedTransaction)>, Vec<(usize, &ConfirmedTransaction)>) = block.transactions.iter().enumerate().partition(|(_index, trx)| {
        !trx.transaction
            .as_ref()
            .and_then(|t| t.message.as_ref())
            .map_or(false, |message| message.account_keys.iter().any(|key| key == &VOTE_INSTRUCTION))
    });

    // Process non-vote transactions
    insert_transactions(tables, clock, block, &non_vote_trx, "");
    insert_account_activity(tables, clock, block, &non_vote_trx, "");

    // Process vote transactions
    insert_transactions(tables, clock, block, &vote_trx, "vote_");
    insert_account_activity(tables, clock, block, &vote_trx, "vote_");
}

pub fn insert_blockinfo(row: &mut TableChange, block: &Block, with_prefix: bool) {
    let height = block.block_height.as_ref().map_or_else(|| "0".to_string(), |bh| bh.block_height.to_string());

    let prefix_str = if with_prefix { "block_" } else { "" };
    row.change(format!("{}slot", prefix_str).as_str(), ("", block.slot.to_string().as_str()))
        .change(format!("{}height", prefix_str).as_str(), ("", height.as_str()))
        .change(format!("{}previous_block_hash", prefix_str).as_str(), ("", block.previous_blockhash.as_str()))
        .change(format!("{}parent_slot", prefix_str).as_str(), ("", block.parent_slot.to_string().as_str()));
}
