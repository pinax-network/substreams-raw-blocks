use common::utils::{bytes_to_hex, to_string_array_to_string};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges, TableChange};
use substreams_solana::{
    base58,
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

use crate::{
    blocks::insert_blockinfo,
    keys::{inner_instruction_keys, instruction_keys},
    utils::insert_timestamp_without_number,
};

pub fn insert_instruction_calls(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, transaction: &ConfirmedTransaction, tx_info: &TxInfo, table_prefix: &str) {
    for (instruction_index, instruction_view) in transaction.walk_instructions().enumerate() {
        if !instruction_view.is_root() {
            continue;
        }

        insert_outer_instruction(tables, clock, block, tx_info, instruction_index, &instruction_view, table_prefix);
        insert_inner_instructions(tables, clock, block, tx_info, instruction_index, &instruction_view, table_prefix);
    }
}

fn insert_outer_instruction(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, tx_info: &TxInfo, instruction_index: usize, instruction_view: &InstructionView, table_prefix: &str) {
    let executing_account = base58::encode(instruction_view.program_id());
    let account_arguments = to_string_array_to_string(&instruction_view.accounts());
    let data = bytes_to_hex(&instruction_view.data());

    let keys = instruction_keys(tx_info.tx_id, instruction_index.to_string().as_str());

    let inner_instructions_str = build_inner_instructions_str(instruction_view);

    let row = tables
        .push_change_composite(format!("{}instruction_calls", table_prefix), keys, 0, table_change::Operation::Create)
        .change("outer_instruction_index", ("", instruction_index.to_string().as_str()))
        .change("outer_executing_account", ("", executing_account.as_str()))
        .change("inner_instruction_index", ("", "-1"))
        .change("inner_executing_account", ("", ""))
        .change("executing_account", ("", executing_account.as_str()))
        .change("is_inner", ("", "false"))
        .change("data", ("", data.as_str()))
        .change("account_arguments", ("", account_arguments.as_str()))
        .change("inner_instructions", ("", inner_instructions_str.as_str()));

    insert_timestamp_without_number(row, clock, false, true);
    insert_blockinfo(row, block, true);
    insert_tx_info(row, tx_info);
}

fn insert_inner_instructions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block, tx_info: &TxInfo, instruction_index: usize, instruction_view: &InstructionView, table_prefix: &str) {
    for (inner_index, inner_instruction) in instruction_view.inner_instructions().enumerate() {
        let inner_data = bytes_to_hex(inner_instruction.data());
        let executing_account = inner_instruction.program_id().to_string();
        let account_arguments = to_string_array_to_string(&inner_instruction.accounts());

        let keys = inner_instruction_keys(tx_info.tx_id, instruction_index.to_string().as_str(), inner_index.to_string().as_str());

        let row = tables
            .push_change_composite(format!("{}instruction_calls", table_prefix), keys, 0, table_change::Operation::Create)
            .change("outer_instruction_index", ("", instruction_index.to_string().as_str()))
            .change("inner_instruction_index", ("", inner_index.to_string().as_str()))
            .change("inner_executing_account", ("", executing_account.as_str()))
            .change("outer_executing_account", ("", instruction_view.program_id().to_string().as_str()))
            .change("executing_account", ("", executing_account.as_str()))
            .change("is_inner", ("", "true"))
            .change("data", ("", inner_data.as_str()))
            .change("account_arguments", ("", account_arguments.as_str()))
            .change("inner_instructions", ("", ""));

        insert_timestamp_without_number(row, clock, false, true);
        insert_blockinfo(row, block, true);
        insert_tx_info(row, tx_info);
    }
}

fn build_inner_instructions_str(instruction_view: &InstructionView) -> String {
    let mut inner_instructions_str = String::new();

    let inner_instructions = instruction_view.inner_instructions().enumerate().collect::<Vec<_>>();
    let last_index = inner_instructions.len().saturating_sub(1);

    for (inner_index, inner_instruction) in inner_instructions {
        // TODO: Check why Dune uses base58 encoding for data
        let inner_data = base58::encode(inner_instruction.data());
        let executing_account = inner_instruction.program_id().to_string();
        let account_arguments = to_string_array_to_string(&inner_instruction.accounts());

        inner_instructions_str.push_str(&format!(
            "('{}','{}',{})",
            inner_data.replace("'", "''"),        // Escape single quotes
            executing_account.replace("'", "''"), // Escape single quotes
            account_arguments
        ));
        if inner_index != last_index {
            inner_instructions_str.push_str(",");
        }
    }

    format!("[{}]", inner_instructions_str)
}

fn insert_tx_info(row: &mut TableChange, tx_info: &TxInfo) {
    row.change("tx_id", ("", tx_info.tx_id))
        .change("tx_index", ("", tx_info.tx_index))
        .change("tx_signer", ("", tx_info.tx_signer))
        .change("tx_success", ("", tx_info.tx_success))
        .change("log_messages", ("", tx_info.log_messages));
}

pub struct TxInfo<'a> {
    pub tx_id: &'a str,
    pub tx_index: &'a str,
    pub tx_signer: &'a str,
    pub tx_success: &'a str,
    pub log_messages: &'a str,
}
