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

        let pre_token_balances = &meta.pre_token_balances;
        let post_token_balances = &meta.post_token_balances;

        for (balance_index, (pre_balance, post_balance)) in meta.pre_balances.iter().zip(meta.post_balances.iter()).enumerate() {
            let pre_token_balance_index = pre_token_balances.iter().position(|balance| balance.account_index == balance_index as u32).unwrap_or(usize::MAX);

            // Start of Selection
            let mut pre_token_balance: Option<f64> = None;
            let mut post_token_balance: Option<f64> = None;
            let mut token_balance_change: Option<f64> = None;
            let mut mint: Option<String> = None;
            let mut owner: Option<String> = None;
            if pre_token_balance_index != usize::MAX {
                if let Some(pre_balance_entry) = pre_token_balances.get(pre_token_balance_index) {
                    if let Some(ui_token_amount) = pre_balance_entry.ui_token_amount.as_ref() {
                        pre_token_balance = Some(ui_token_amount.ui_amount);
                        mint = Some(pre_balance_entry.mint.clone());
                        owner = Some(pre_balance_entry.owner.clone());
                    }
                }
                if let Some(post_balance_entry) = post_token_balances.get(pre_token_balance_index) {
                    if let Some(ui_token_amount) = post_balance_entry.ui_token_amount.as_ref() {
                        post_token_balance = Some(ui_token_amount.ui_amount);
                    }
                }
                if let (Some(pre), Some(post)) = (pre_token_balance, post_token_balance) {
                    // Use a more precise calculation method to avoid floating-point precision issues
                    let pre_scaled = (pre * 1_000_000_000.0).round() as i64;
                    let post_scaled = (post * 1_000_000_000.0).round() as i64;
                    let change_scaled = post_scaled - pre_scaled;
                    token_balance_change = Some(change_scaled as f64 / 1_000_000_000.0);
                }
            }
            let balance_change = *post_balance as i128 - *pre_balance as i128;

            let trx = transaction.transaction.as_ref().unwrap();
            let message = trx.message.as_ref().unwrap();

            let address = base58::encode(message.account_keys.get(balance_index).unwrap_or(&Vec::new()));

            let nb_signatures = trx.signatures.len();
            let signed = balance_index < nb_signatures;

            // TODO: Find out why this is not working
            let writable = meta.loaded_writable_addresses.iter().any(|addr| base58::encode(addr) == address);

            let keys = account_activity_keys(&transaction_id, address.as_str());

            let row = tables
                .push_change_composite("account_activity", keys, 0, table_change::Operation::Create)
                .change("address", ("", address.as_str()))
                .change("tx_index", ("", transaction_index.as_str()))
                .change("tx_id", ("", transaction_id.as_str()))
                .change("tx_success", ("", tx_success.to_string().as_str()))
                .change("signed", ("", signed.to_string().as_str()))
                .change("writable", ("", writable.to_string().as_str()))
                .change("token_mint_address", ("", mint.as_deref().unwrap_or("")))
                .change("pre_balance", ("", pre_balance.to_string().as_str()))
                .change("post_balance", ("", post_balance.to_string().as_str()))
                .change("balance_change", ("", balance_change.to_string().as_str()))
                .change("pre_token_balance", ("", pre_token_balance.map(|v| v.to_string()).as_deref().unwrap_or("")))
                .change("post_token_balance", ("", post_token_balance.map(|v| v.to_string()).as_deref().unwrap_or("")))
                .change("token_balance_change", ("", token_balance_change.map(|v| v.to_string()).as_deref().unwrap_or("")))
                .change("token_balance_owner", ("", owner.as_deref().unwrap_or("")));

            insert_timestamp_without_number(row, clock, false, false);
            insert_blockinfo(row, block, true);
        }
    }
}
