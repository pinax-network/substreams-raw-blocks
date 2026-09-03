use common::structs::BlockIdentity;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

use crate::pb::pinax::solana::v2::Message;

pub fn collect_message(tx: &ConfirmedTransaction, index: u32, signature: &[u8], id: &BlockIdentity) -> Message {
    let message = tx.transaction.as_ref().and_then(|t| t.message.clone()).unwrap_or_default();
    let header = message.header.clone().unwrap_or_default();
    let meta = tx.meta.clone().unwrap_or_default();

    Message {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        slot: id.block_num,
        transaction_index: index,
        message_index: 0,
        num_required_signatures: header.num_required_signatures,
        num_readonly_signed_accounts: header.num_readonly_signed_accounts,
        num_readonly_unsigned_accounts: header.num_readonly_unsigned_accounts,
        recent_blockhash: message.recent_blockhash.clone(),
        versioned: message.versioned,
        account_keys: message.account_keys.clone(),
        loaded_writable_addresses: meta.loaded_writable_addresses.clone(),
        loaded_readonly_addresses: meta.loaded_readonly_addresses.clone(),

        signature: signature.to_vec(),
    }
}
