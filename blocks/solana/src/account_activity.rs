use std::collections::HashMap;

use common::structs::BlockIdentity;
use substreams_solana::pb::sf::solana::r#type::v1::{ConfirmedTransaction, MessageHeader, TokenBalance};

use crate::pb::pinax::solana::v2::AccountActivity;

// One row per account of the transaction with its SOL balance movement and, for
// token accounts, the matching pre/post token amounts.
pub fn collect_account_activity(tx: &ConfirmedTransaction, index: u32, signature: &[u8], id: &BlockIdentity) -> Vec<AccountActivity> {
    let (Some(transaction), Some(meta)) = (tx.transaction.as_ref(), tx.meta.as_ref()) else {
        return vec![];
    };
    let message = transaction.message.clone().unwrap_or_default();
    let header = message.header.clone().unwrap_or_default();
    let keys = crate::utils::account_keys_extended(tx);
    let tx_success = meta.err.as_ref().map_or(true, |e| e.err.is_empty());
    let pre_tokens: HashMap<u32, &TokenBalance> = meta.pre_token_balances.iter().map(|b| (b.account_index, b)).collect();
    let post_tokens: HashMap<u32, &TokenBalance> = meta.post_token_balances.iter().map(|b| (b.account_index, b)).collect();
    let writable = writability(&header, keys.len(), meta.loaded_writable_addresses.len(), meta.loaded_readonly_addresses.len());

    meta.pre_balances
        .iter()
        .zip(meta.post_balances.iter())
        .enumerate()
        .map(|(i, (pre, post))| {
            let account_index = i as u32;
            let token = post_tokens.get(&account_index).or_else(|| pre_tokens.get(&account_index)).copied();
            let amount = |b: Option<&&TokenBalance>| b.and_then(|t| t.ui_token_amount.as_ref()).map(|u| if u.amount.is_empty() { "0".to_string() } else { u.amount.clone() });
            AccountActivity {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                slot: id.block_num,
                transaction_index: index,
                signature: signature.to_vec(),
                account_index,
                address: keys.get(i).cloned().unwrap_or_default(),
                signed: i < transaction.signatures.len(),
                writable: writable.get(i).copied().unwrap_or(false),
                tx_success,
                pre_balance: *pre,
                post_balance: *post,
                balance_change: (*post as i128 - *pre as i128) as i64,
                token_mint: token.map(|t| t.mint.clone()),
                token_owner: token.map(|t| t.owner.clone()),
                pre_token_amount: amount(pre_tokens.get(&account_index)),
                post_token_amount: amount(post_tokens.get(&account_index)),
                token_decimals: token.and_then(|t| t.ui_token_amount.as_ref()).map(|u| u.decimals),
            }
        })
        .collect()
}

// Account ordering: [signed writable][signed readonly][unsigned writable][unsigned readonly]
// for the static keys, then loaded writable, then loaded readonly addresses.
fn writability(header: &MessageHeader, total: usize, loaded_writable: usize, loaded_readonly: usize) -> Vec<bool> {
    let static_len = total.saturating_sub(loaded_writable + loaded_readonly);
    let signed = header.num_required_signatures as usize;
    let signed_readonly = header.num_readonly_signed_accounts as usize;
    let unsigned_readonly = header.num_readonly_unsigned_accounts as usize;
    (0..total)
        .map(|i| {
            if i < static_len {
                if i < signed {
                    i < signed.saturating_sub(signed_readonly)
                } else {
                    i < static_len.saturating_sub(unsigned_readonly)
                }
            } else {
                i < static_len + loaded_writable
            }
        })
        .collect()
}
