use common::utils::bytes_to_hex;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_solana::{
    base58,
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{
    blocks::insert_blockinfo,
    keys::{inner_instruction_keys, instruction_keys},
    utils::{build_csv_string, insert_timestamp_without_number},
};

pub fn insert_instructions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, transaction: &ConfirmedTransaction, trx_index: &str, signature: &str) {
    transaction.walk_instructions().enumerate().for_each(|(instruction_index, instruction_view)| {
        insert_single_instruction(tables, clock, block, signature, trx_index, instruction_index, &instruction_view);
        insert_inner_instructions(tables, clock, block, signature, trx_index, instruction_index, &instruction_view);
    });
}

fn insert_single_instruction(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, signature: &str, trx_index: &str, instruction_index: usize, instruction_view: &InstructionView) {
    let executing_account = base58::encode(instruction_view.program_id());
    let account_arguments = build_csv_string(&instruction_view.accounts());
    let data = bytes_to_hex(&instruction_view.data());

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
        let inner_data = bytes_to_hex(inner_instruction.data());

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
