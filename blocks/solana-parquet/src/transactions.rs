use common::utils::{number_array_to_string, string_array_to_string, string_array_to_string_with_escapes};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{
    blocks::insert_blockinfo,
    instruction_calls::{insert_instruction_calls, TxInfo},
    pb::solana::rawblocks::Transaction as RawTransaction,
    structs::{BlockInfo, BlockTimestamp},
    tx_errors::TransactionErrorDecoder,
    utils::{build_csv_string, encode_byte_vectors_to_base58_string, get_account_keys_extended, insert_timestamp_without_number},
};

pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, transactions: &Vec<(usize, &ConfirmedTransaction)>, table_prefix: &str) {
    for (index, transaction) in transactions {
        let meta = transaction.meta.as_ref().expect("Transaction meta is missing");
        let trx = transaction.transaction.as_ref().expect("Transaction is missing");
        let message = trx.message.as_ref().expect("Transaction message is missing");
        let header = message.header.as_ref().expect("Transaction header is missing");

        let account_keys = string_array_to_string(&get_account_keys_extended(transaction));

        let success = meta.err.is_none();

        let err = match &meta.err {
            Some(err) => decode_transaction_error(&err.err),
            None => String::new(),
        };

        let signatures = encode_byte_vectors_to_base58_string(&trx.signatures);
        let first_signature = transaction.id();

        let recent_block_hash = base58::encode(&message.recent_blockhash);
        let log_messages = string_array_to_string_with_escapes(&meta.log_messages);
        // let log_messages = build_csv_string(&meta.log_messages);
        let pre_balances = number_array_to_string(&meta.pre_balances);
        let post_balances = number_array_to_string(&meta.post_balances);
        let signers: Vec<String> = message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).collect();
        let signers_str = string_array_to_string(&signers);
        let signer = signers.first().unwrap();
        let index_str = index.to_string();

        let row = tables
            .push_change(format!("{}transactions", table_prefix), &first_signature, 0, table_change::Operation::Create)
            .change("id", ("", first_signature.as_str()))
            .change("index", ("", index_str.as_str()))
            .change("fee", ("", meta.fee.to_string().as_str()))
            .change("required_signatures", ("", header.num_required_signatures.to_string().as_str()))
            .change("required_signed_accounts", ("", header.num_readonly_signed_accounts.to_string().as_str()))
            .change("required_unsigned_accounts", ("", header.num_readonly_unsigned_accounts.to_string().as_str()))
            .change("signature", ("", first_signature.as_str()))
            .change("success", ("", success.to_string().as_str()))
            .change("error", ("", err.as_str()))
            .change("recent_block_hash", ("", recent_block_hash.as_str()))
            .change("account_keys", ("", account_keys.as_str()))
            .change("log_messages", ("", log_messages.as_str()))
            .change("pre_balances", ("", pre_balances.as_str()))
            .change("post_balances", ("", post_balances.as_str()))
            .change("signatures", ("", signatures.as_str()))
            .change("signer", ("", signer.as_str()))
            .change("signers", ("", signers_str.as_str()));

        insert_timestamp_without_number(row, clock, false, true);
        insert_blockinfo(row, block, true);

        let tx_info = TxInfo {
            tx_id: &first_signature,
            tx_index: &index_str,
            tx_signer: &signer,
            tx_success: &success.to_string(),
            log_messages: &log_messages,
        };

        insert_instruction_calls(tables, clock, block, transaction, &tx_info, table_prefix);
    }
}

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
