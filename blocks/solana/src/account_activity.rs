use std::collections::HashSet;

use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{
    blocks::insert_blockinfo,
    keys::account_activity_keys,
    utils::{get_account_keys_extended, insert_timestamp_without_number},
};

pub fn insert_account_activity(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, transactions: &[(&ConfirmedTransaction, usize)]) {
    for (transaction, index) in transactions {
        let meta = match transaction.meta.as_ref() {
            Some(m) => m,
            None => continue, // Skip if metadata is missing
        };
        let transaction_id = transaction.id();
        let transaction_index = index.to_string(); // Consider reusing a buffer if performance is critical
        let tx_success = meta.err.is_none();

        let trx = match transaction.transaction.as_ref() {
            Some(t) => t,
            None => continue, // Skip if transaction data is missing
        };

        let account_keys_extended = get_account_keys_extended(transaction);

        // Precompute a HashSet of Base58-encoded writable addresses for efficient lookup
        let writable_addresses: HashSet<String> = meta.loaded_writable_addresses.iter().map(|addr| base58::encode(addr)).collect();

        // Precompute a mapping from account_index to pre_token_balance_index
        let account_to_token_balance_map: Vec<Option<usize>> = {
            // Determine the maximum account_index to size the vector appropriately
            let max_account_index = meta.pre_token_balances.iter().map(|balance| balance.account_index as usize).max().unwrap_or(0);
            let mut map = vec![None; max_account_index + 1];
            for (i, balance) in meta.pre_token_balances.iter().enumerate() {
                let idx = balance.account_index as usize;
                // Assuming that the last occurrence is preferred if duplicates exist
                map[idx] = Some(i);
            }
            map
        };

        for (balance_index, (pre_balance, post_balance)) in meta.pre_balances.iter().zip(meta.post_balances.iter()).enumerate() {
            let address = match account_keys_extended.get(balance_index) {
                Some(addr) => addr,
                None => continue, // Skip if address is missing
            };

            // Skip if address is a program derived address
            if address.contains("111111111111") {
                continue;
            }

            // Efficiently retrieve the pre_token_balance_index using the precomputed map
            let pre_token_balance_index = account_to_token_balance_map.get(balance_index).copied().flatten().unwrap_or(usize::MAX);

            let (pre_token_balance, post_token_balance, token_balance_change, mint, owner) = if pre_token_balance_index != usize::MAX {
                extract_token_balance_changes(&meta.pre_token_balances, &meta.post_token_balances, pre_token_balance_index)
            } else {
                (None, None, None, None, None)
            };

            let balance_change = *post_balance as i128 - *pre_balance as i128;

            let signed = is_signed(trx, balance_index);
            let writable = writable_addresses.contains(address);

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
                .change("pre_token_balance", ("", pre_token_balance.as_ref().map(|v| v.to_string()).as_deref().unwrap_or("")))
                .change("post_token_balance", ("", post_token_balance.as_ref().map(|v| v.to_string()).as_deref().unwrap_or("")))
                .change("token_balance_change", ("", token_balance_change.as_ref().map(|v| v.to_string()).as_deref().unwrap_or("")))
                .change("token_balance_owner", ("", owner.as_deref().unwrap_or("")));

            insert_timestamp_without_number(row, clock, false, false);
            insert_blockinfo(row, block, true);
        }
    }
}

fn extract_token_balance_changes(
    pre_balances: &[substreams_solana::pb::sf::solana::r#type::v1::TokenBalance],
    post_balances: &[substreams_solana::pb::sf::solana::r#type::v1::TokenBalance],
    index: usize,
) -> (Option<f64>, Option<f64>, Option<f64>, Option<String>, Option<String>) {
    let pre_balance = pre_balances.get(index).and_then(|pre_balance_entry| pre_balance_entry.ui_token_amount.as_ref().map(|ui| ui.ui_amount));
    let post_balance = post_balances
        .get(index)
        .and_then(|post_balance_entry| post_balance_entry.ui_token_amount.as_ref().map(|ui| ui.ui_amount));
    let mint = pre_balances
        .get(index)
        .and_then(|pre_balance_entry| if !pre_balance_entry.mint.is_empty() { Some(pre_balance_entry.mint.clone()) } else { None });
    let owner = pre_balances
        .get(index)
        .and_then(|pre_balance_entry| if !pre_balance_entry.owner.is_empty() { Some(pre_balance_entry.owner.clone()) } else { None });

    let token_balance_change = match (pre_balance, post_balance) {
        (Some(pre), Some(post)) => {
            // Use a more precise calculation method to avoid floating-point precision issues
            let pre_scaled = (pre * 1_000_000_000.0).round() as i64;
            let post_scaled = (post * 1_000_000_000.0).round() as i64;
            Some((post_scaled - pre_scaled) as f64 / 1_000_000_000.0)
        }
        _ => None,
    };

    (pre_balance, post_balance, token_balance_change, mint, owner)
}

fn is_signed(trx: &substreams_solana::pb::sf::solana::r#type::v1::Transaction, index: usize) -> bool {
    trx.signatures.len() > index
}
