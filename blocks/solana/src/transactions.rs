use common::blocks::insert_timestamp;
use substreams::{pb::substreams::Clock, proto::decode};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, TransactionError},
};

use crate::{blocks::insert_blockinfo, utils::insert_timestamp_without_number};

pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    block.transactions.iter().enumerate().for_each(|(index, transaction)| {
        let meta = transaction.meta.as_ref().expect("Transaction meta is missing");
        let trx = transaction.transaction.as_ref().expect("Transaction is missing");
        let message = trx.message.as_ref().expect("Transaction message is missing");
        let header = message.header.as_ref().expect("Transaction header is missing");

        let success = meta.err.is_none();

        // TODO: decode transaction error
        let err = match &meta.err {
            Some(err) => base58::encode(&err.err),
            None => String::new(),
        };

        let signatures: String = trx.signatures.iter().map(|sig| base58::encode(sig)).collect::<Vec<String>>().join(",");

        let first_signature = signatures.split(",").next().unwrap_or("");

        let account_keys = message.account_keys.iter().map(|key| base58::encode(key)).collect::<Vec<String>>().join(",");
        let recent_block_hash = base58::encode(&message.recent_blockhash);

        let log_messages = meta.log_messages.iter().map(|log| log.to_string()).collect::<Vec<String>>().join(",");

        let pre_balances = meta.pre_balances.iter().map(|balance| balance.to_string()).collect::<Vec<String>>().join(",");
        let post_balances = meta.post_balances.iter().map(|balance| balance.to_string()).collect::<Vec<String>>().join(",");
        let signers: Vec<String> = message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).collect();
        let signers_str = signers.join(",");
        let signer = signers.first().cloned().unwrap_or_default();

        let row = tables
            .push_change("transactions", first_signature, 0, table_change::Operation::Create)
            .change("id", ("", first_signature))
            .change("index", ("", index.to_string().as_str()))
            .change("fee", ("", meta.fee.to_string().as_str()))
            .change("required_signatures", ("", header.num_required_signatures.to_string().as_str()))
            .change("required_signed_accounts", ("", header.num_readonly_signed_accounts.to_string().as_str()))
            .change("required_unsigned_accounts", ("", header.num_readonly_unsigned_accounts.to_string().as_str()))
            .change("signature", ("", first_signature))
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
    })
}

pub fn decode_transaction_error(err: TransactionError) {}
