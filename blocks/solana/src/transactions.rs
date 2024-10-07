use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    pb::sf::solana::r#type::v1::{Block, TransactionError},
};

use crate::{
    blocks::insert_blockinfo,
    instructions::insert_instructions,
    token_balances::insert_token_balances,
    utils::{b58decode_and_build_csv_string, build_csv_string, insert_timestamp_without_number},
};

static VOTE_ACCOUNT_KEY: &str = "Vote111111111111111111111111111111111111111";

pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    for (index, transaction) in block.transactions.iter().enumerate() {
        let meta = transaction.meta.as_ref().expect("Transaction meta is missing");
        let trx = transaction.transaction.as_ref().expect("Transaction is missing");
        let message = trx.message.as_ref().expect("Transaction message is missing");
        let header = message.header.as_ref().expect("Transaction header is missing");

        // If vote transaction, skip it
        if message.account_keys.iter().any(|key| base58::encode(key) == VOTE_ACCOUNT_KEY) {
            continue;
        }
        let account_keys = b58decode_and_build_csv_string(&message.account_keys);

        let success = meta.err.is_none();

        // TODO: decode transaction error
        let err = match &meta.err {
            Some(err) => base58::encode(&err.err),
            None => String::new(),
        };

        let signatures: String = trx.signatures.iter().map(base58::encode).collect::<Vec<String>>().join(",");

        // let first_signature = &trx.signatures.first().map(|sig| base58::encode(sig)).unwrap_or_default();
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

        insert_instructions(tables, clock, block, transaction, index_str.as_str(), &first_signature);
    }
}

// TODO: decode transaction error
pub fn decode_transaction_error(err: TransactionError) {}
