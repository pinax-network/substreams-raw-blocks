use common::structs::BlockIdentity;
use substreams_solana::pb::sf::solana::r#type::v1::{ConfirmedTransaction, TokenBalance as TokenBalanceSource};

use crate::pb::pinax::solana::v2::TokenBalance;
use crate::utils::account_keys_extended;

pub fn collect_token_balances(tx: &ConfirmedTransaction, index: u32, signature: &[u8], id: &BlockIdentity) -> Vec<TokenBalance> {
    let Some(meta) = tx.meta.as_ref() else {
        return vec![];
    };
    let keys = account_keys_extended(tx);
    let mut rows = Vec::with_capacity(meta.pre_token_balances.len() + meta.post_token_balances.len());
    for (balance_type, balances) in [("pre", &meta.pre_token_balances), ("post", &meta.post_token_balances)] {
        for (balance_index, tb) in balances.iter().enumerate() {
            rows.push(token_balance(tb, balance_type, balance_index as u32, index, signature, &keys, id));
        }
    }
    rows
}

fn token_balance(tb: &TokenBalanceSource, balance_type: &str, balance_index: u32, transaction_index: u32, signature: &[u8], keys: &[Vec<u8>], id: &BlockIdentity) -> TokenBalance {
    let ui = tb.ui_token_amount.clone().unwrap_or_default();
    TokenBalance {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        slot: id.block_num,
        transaction_index,
        balance_index,
        balance_type: balance_type.to_string(),
        account_index: tb.account_index,
        mint: tb.mint.clone(),
        owner: tb.owner.clone(),
        program_id: tb.program_id.clone(),
        amount: if ui.amount.is_empty() { "0".to_string() } else { ui.amount.clone() },
        ui_amount: tb.ui_token_amount.as_ref().map(|u| u.ui_amount),
        decimals: ui.decimals,
        ui_amount_string: ui.ui_amount_string.clone(),

        signature: signature.to_vec(),
        account: keys.get(tb.account_index as usize).cloned().unwrap_or_default(),
    }
}
