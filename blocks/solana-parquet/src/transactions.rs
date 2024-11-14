use common::utils::string_array_to_string_with_escapes;
use substreams_solana::{base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

use crate::{
    pb::solana::rawblocks::Transaction as RawTransaction,
    structs::{BlockInfo, BlockTimestamp},
    tx_errors::TransactionErrorDecoder,
    utils::get_account_keys_extended,
};

pub fn collect_transactions(transactions: &Vec<(usize, &ConfirmedTransaction)>, block_info: &BlockInfo, timestamp: &BlockTimestamp) -> Vec<RawTransaction> {
    let mut trx_vec: Vec<RawTransaction> = Vec::new();

    for (index, transaction) in transactions {
        let meta = transaction.meta.as_ref().expect("Transaction meta is missing");
        let trx = transaction.transaction.as_ref().expect("Transaction is missing");
        let message = trx.message.as_ref().expect("Transaction message is missing");
        let header = message.header.as_ref().expect("Transaction header is missing");

        let account_keys = get_account_keys_extended(transaction);
        let success = meta.err.is_none();
        let err = match &meta.err {
            Some(err) => decode_transaction_error(&err.err),
            None => String::new(),
        };

        trx_vec.push(RawTransaction {
            block_time: Some(timestamp.time),
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),
            block_slot: block_info.slot,
            block_height: block_info.height,
            block_previous_block_hash: block_info.previous_block_hash.clone(),
            block_parent_slot: block_info.parent_slot,
            id: transaction.id(),
            index: *index as u32,
            fee: meta.fee,
            required_signatures: header.num_required_signatures,
            required_signed_accounts: header.num_readonly_signed_accounts,
            required_unsigned_accounts: header.num_readonly_unsigned_accounts,
            signature: transaction.id(),
            success,
            error: err,
            recent_block_hash: base58::encode(&message.recent_blockhash),
            account_keys,
            log_messages: string_array_to_string_with_escapes(&meta.log_messages),
            // TODO: output as uint array when sink-files supports it
            pre_balances: meta.pre_balances.iter().map(|balance| balance.to_string()).collect(),
            post_balances: meta.post_balances.iter().map(|balance| balance.to_string()).collect(),
            signatures: trx.signatures.iter().map(base58::encode).collect(),
            signer: message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).next().unwrap(),
            signers: message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).collect(),
        });
    }

    trx_vec
}

pub fn decode_transaction_error(err: &Vec<u8>) -> String {
    match TransactionErrorDecoder::decode_error(err) {
        Ok(decoded_error) => TransactionErrorDecoder::format_error(&decoded_error),
        Err(decode_error) => format!("Error decoding transaction error: {:?}", decode_error),
    }
}
