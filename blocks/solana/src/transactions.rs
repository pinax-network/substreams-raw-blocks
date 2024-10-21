use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionError},
};

use crate::{
    blocks::insert_blockinfo,
    instruction_calls::{insert_instruction_calls, TxInfo},
    instructions::insert_instructions,
    utils::{build_csv_string, get_account_keys_extended, insert_timestamp_without_number},
};

pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, transactions: &Vec<(&ConfirmedTransaction, usize)>) {
    for (transaction, index) in transactions {
        let meta = transaction.meta.as_ref().expect("Transaction meta is missing");
        let trx = transaction.transaction.as_ref().expect("Transaction is missing");
        let message = trx.message.as_ref().expect("Transaction message is missing");
        let header = message.header.as_ref().expect("Transaction header is missing");

        let account_keys = build_csv_string(&get_account_keys_extended(transaction));

        let success = meta.err.is_none();

        // TODO: decode transaction error
        let err = match &meta.err {
            Some(err) => base58::encode(&err.err),
            None => String::new(),
        };

        let signatures: String = trx.signatures.iter().map(base58::encode).collect::<Vec<String>>().join(",");
        let first_signature = transaction.id();

        let recent_block_hash = base58::encode(&message.recent_blockhash);
        let log_messages = build_csv_string(&meta.log_messages);
        let pre_balances = build_csv_string(&meta.pre_balances);
        let post_balances = build_csv_string(&meta.post_balances);
        let signers: Vec<String> = message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).collect();
        let signers_str = signers.join(",");
        let signer = signers.first().unwrap();
        let index_str = index.to_string();

        let row = tables
            .push_change("transactions", &first_signature, 0, table_change::Operation::Create)
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

        insert_instruction_calls(tables, clock, block, transaction, &tx_info);
    }
}

// TODO: decode transaction error
pub fn decode_transaction_error(err: TransactionError) {}
