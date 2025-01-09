use substreams_solana::{base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

use crate::{
    pb::pinax::solana::v1::Transaction as RawTransaction,
    structs::{BlockInfo, BlockTimestamp},
    tx_errors::TransactionErrorDecoder,
    utils::get_account_keys_extended,
};

pub fn collect_transaction(transaction: &ConfirmedTransaction, index: usize, block_info: &BlockInfo, timestamp: &BlockTimestamp) -> RawTransaction {
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

    let signers = message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).collect::<Vec<String>>();

    RawTransaction {
        // block
        block_time: timestamp.time.to_string(),
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),
        block_slot: block_info.slot,
        block_height: block_info.height,
        block_previous_block_hash: block_info.previous_block_hash.clone(),
        block_parent_slot: block_info.parent_slot,

        // transaction
        id: transaction.id(),
        index: index as u32,
        required_signatures: header.num_required_signatures,
        required_signed_accounts: header.num_readonly_signed_accounts,
        required_unsigned_accounts: header.num_readonly_unsigned_accounts,
        signature: transaction.id(),
        signatures: trx.signatures.iter().map(|sig| base58::encode(sig)).collect::<Vec<String>>(),

        // message
        recent_block_hash: base58::encode(&message.recent_blockhash),
        account_keys: account_keys.iter().map(|key| base58::encode(key)).collect::<Vec<String>>(),
        signer: message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).next().unwrap(),
        signers,

        // meta
        error: err,
        success,
        fee: meta.fee,
        compute_units_consumed: meta.compute_units_consumed(),
        log_messages: meta.log_messages.clone(),
        pre_balances: meta.pre_balances.clone().iter().map(|balance| balance.to_string()).collect::<Vec<String>>(),
        post_balances: meta.post_balances.clone().iter().map(|balance| balance.to_string()).collect::<Vec<String>>(),
    }
}

pub fn decode_transaction_error(err: &Vec<u8>) -> String {
    match TransactionErrorDecoder::decode_error(err) {
        Ok(decoded_error) => TransactionErrorDecoder::format_error(&decoded_error),
        Err(decode_error) => format!("Error decoding transaction error: {:?}", decode_error),
    }
}
