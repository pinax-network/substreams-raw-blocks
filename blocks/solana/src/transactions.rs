use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionError},
};

use crate::{
    blocks::insert_blockinfo,
    keys::{inner_instruction_keys, instruction_keys},
    utils::insert_timestamp_without_number,
};

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

        let signatures: String = trx.signatures.iter().map(base58::encode).collect::<Vec<String>>().join(",");

        let first_signature = trx.signatures.first().map(|sig| base58::encode(sig)).unwrap_or_default();

        let account_keys = decode_and_build_csv_string(&message.account_keys);
        let recent_block_hash = base58::encode(&message.recent_blockhash);
        let log_messages = build_csv_string(&meta.log_messages);
        let pre_balances = build_csv_string(&meta.pre_balances);
        let post_balances = build_csv_string(&meta.post_balances);
        let signers: Vec<String> = message.account_keys.iter().take(trx.signatures.len()).map(|key| base58::encode(key)).collect();
        let signers_str = signers.join(",");
        let signer = signers.first().unwrap();
        let index_str = index.to_string();

        let row = tables
            .push_change("transactions", first_signature, 0, table_change::Operation::Create)
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

        insert_instructions(tables, clock, block, transaction, index_str.as_str(), first_signature);
    })
}

fn insert_instructions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, transaction: &ConfirmedTransaction, trx_index: &str, signature: &str) {
    transaction.walk_instructions().enumerate().for_each(|(instruction_index, instruction_view)| {
        insert_single_instruction(tables, clock, block, signature, trx_index, instruction_index, &instruction_view);
        insert_inner_instructions(tables, clock, block, signature, trx_index, instruction_index, &instruction_view);
    });
}

fn insert_single_instruction(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, signature: &str, trx_index: &str, instruction_index: usize, instruction_view: &InstructionView) {
    let executing_account = base58::encode(instruction_view.program_id());
    let account_arguments = build_csv_string(&instruction_view.accounts());
    let data = base58::encode(instruction_view.data());

    let keys = instruction_keys(signature, instruction_index.to_string().as_str());

    let row = tables
        .push_change_composite("instructions", keys, 0, table_change::Operation::Create)
        .change("transaction_id", ("", signature))
        .change("transaction_index", ("", trx_index))
        .change("instruction_index", ("", instruction_index.to_string().as_str()))
        .change("data", ("", data.as_str()))
        .change("executing_account", ("", executing_account.as_str()))
        .change("account_arguments", ("", account_arguments.as_str()));

    insert_timestamp_without_number(row, clock, false, true);
    insert_blockinfo(row, block, true);
}

fn insert_inner_instructions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, signature: &str, trx_index: &str, instruction_index: usize, instruction_view: &InstructionView) {
    instruction_view.inner_instructions().enumerate().for_each(|(inner_index, inner_instruction)| {
        let executing_account = inner_instruction.program_id().to_string();
        let account_arguments = build_csv_string(&inner_instruction.accounts());
        let inner_data = base58::encode(inner_instruction.data());

        let keys = inner_instruction_keys(signature, instruction_index.to_string().as_str(), inner_index.to_string().as_str());

        let row = tables
            .push_change_composite("inner_instructions", keys, 0, table_change::Operation::Create)
            .change("transaction_id", ("", signature))
            .change("transaction_index", ("", trx_index))
            .change("instruction_index", ("", instruction_index.to_string().as_str()))
            .change("inner_instruction_index", ("", inner_index.to_string().as_str()))
            .change("data", ("", inner_data.as_str()))
            .change("executing_account", ("", executing_account.as_str()))
            .change("account_arguments", ("", account_arguments.as_str()));

        insert_timestamp_without_number(row, clock, false, true);
        insert_blockinfo(row, block, true);
    });
}

fn build_csv_string<T: ToString>(values: &[T]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(",")
}

fn decode_and_build_csv_string(values: &[Vec<u8>]) -> String {
    values.iter().map(|value| base58::encode(value)).collect::<Vec<String>>().join(",")
}

pub fn decode_transaction_error(err: TransactionError) {}
