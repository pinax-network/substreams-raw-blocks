use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::{blocks::insert_blockinfo, keys::token_balance_keys, utils::insert_timestamp_without_number};

pub fn insert_token_balances(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    for (index, transaction) in block.transactions.iter().enumerate() {
        let meta = transaction.meta.as_ref().unwrap();
        let transaction_id = transaction.id();
        let transaction_index = index.to_string();

        for (pre_balance, post_balance) in meta.pre_token_balances.iter().zip(meta.post_token_balances.iter()) {
            let pre_amount = &pre_balance.ui_token_amount.as_ref().unwrap().ui_amount_string;
            let post_amount = &post_balance.ui_token_amount.as_ref().unwrap().ui_amount_string;

            // Don't insert if the balance didn't change
            if pre_amount == post_amount {
                continue;
            }

            let program_id = pre_balance.program_id.as_str();
            let account = transaction.account_at(pre_balance.account_index as u8).to_string();
            let mint = pre_balance.mint.as_str();
            let owner = pre_balance.owner.as_str();

            let keys = token_balance_keys(&transaction.id(), program_id, account.as_str());

            let row = tables
                .push_change_composite("token_balances", keys, 0, table_change::Operation::Create)
                .change("transaction_id", ("", transaction_id.as_str()))
                .change("transaction_index", ("", transaction_index.as_str()))
                .change("program_id", ("", program_id))
                .change("account", ("", account.as_str()))
                .change("mint", ("", mint))
                .change("owner", ("", owner))
                .change("pre_amount", ("", pre_amount.as_str()))
                .change("post_amount", ("", post_amount.as_str()));

            insert_timestamp_without_number(row, clock, false, false);
            insert_blockinfo(row, block, true);
        }
    }
}
