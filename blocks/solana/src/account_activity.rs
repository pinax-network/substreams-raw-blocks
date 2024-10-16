use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{blocks::insert_blockinfo, keys::account_activity_keys, utils::insert_timestamp_without_number};

pub fn insert_account_activity(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, transactions: &Vec<(&ConfirmedTransaction, usize)>) {
    for (transaction, index) in transactions {
        let meta = transaction.meta.as_ref().unwrap();
        let transaction_id = transaction.id();
        let transaction_index = index.to_string();
        let tx_success = meta.err.is_none();

        for (index, (pre_balance, post_balance)) in meta.pre_token_balances.iter().zip(meta.post_token_balances.iter()).enumerate() {
            // let pre_token_amount = &pre_balance.ui_token_amount.as_ref().unwrap().ui_amount_string;
            // let post_token_amount = &post_balance.ui_token_amount.as_ref().unwrap().ui_amount_string;
            let pre_token_amount = pre_balance.ui_token_amount.as_ref().unwrap().ui_amount;
            let post_token_amount = post_balance.ui_token_amount.as_ref().unwrap().ui_amount;
            let token_balance_change = format!("{}", post_token_amount - pre_token_amount);

            let pre_amount = *meta.pre_balances.get(index).unwrap();
            let post_amount = *meta.post_balances.get(index).unwrap();
            let amount_change = (post_amount as i128) - (pre_amount as i128);

            let account = transaction.account_at(pre_balance.account_index as u8).to_string();
            let mint = pre_balance.mint.as_str();
            let owner = pre_balance.owner.as_str();
            let trx = transaction.transaction.as_ref().unwrap();

            // If signatures has a length of 2, it means that the transaction has two signers, which are the first two accounts in the message
            let nb_signatures = trx.signatures.len();
            let signed = trx.message.as_ref().unwrap().account_keys.iter().take(nb_signatures).any(|key| base58::encode(key) == account);

            let writable = meta.loaded_writable_addresses.iter().any(|addr| base58::encode(addr) == account);

            let keys = account_activity_keys(&transaction_id, account.as_str());

            let row = tables
                .push_change_composite("account_activity", keys, 0, table_change::Operation::Create)
                .change("address", ("", account.as_str()))
                .change("tx_index", ("", transaction_index.as_str()))
                .change("tx_id", ("", transaction_id.as_str()))
                .change("tx_success", ("", tx_success.to_string().as_str()))
                .change("signed", ("", signed.to_string().as_str()))
                .change("writable", ("", writable.to_string().as_str()))
                .change("token_mint_address", ("", mint))
                .change("pre_balance", ("", pre_amount.to_string().as_str()))
                .change("post_balance", ("", post_amount.to_string().as_str()))
                .change("balance_change", ("", amount_change.to_string().as_str()))
                .change("pre_token_balance", ("", pre_token_amount.to_string().as_str()))
                .change("post_token_balance", ("", post_token_amount.to_string().as_str()))
                .change("token_balance_change", ("", token_balance_change.as_str()))
                .change("token_balance_owner", ("", owner));

            insert_timestamp_without_number(row, clock, false, false);
            insert_blockinfo(row, block, true);
        }
    }
}
