use substreams::Hex;
use substreams_solana::{base58, block_view::InstructionView, pb::sf::solana::r#type::v1::ConfirmedTransaction};

use crate::{
    pb::solana::InstructionCall,
    structs::{BlockInfo, BlockTimestamp},
};

pub fn collect_tx_instruction_calls(transaction: &ConfirmedTransaction, index: usize, block_info: &BlockInfo, timestamp: &BlockTimestamp) -> Vec<InstructionCall> {
    let mut vec: Vec<InstructionCall> = vec![];

    let message = transaction.transaction.as_ref().expect("Transaction is missing").message.as_ref().expect("Message is missing");
    let signer = base58::encode(message.account_keys.first().expect("Signer is missing"));

    let tx_info = TxInfo {
        tx_id: transaction.id().to_string(),
        tx_index: index as u32,
        tx_signer: signer,
        tx_success: transaction.meta.as_ref().unwrap().err.is_none(),
        log_messages: transaction.meta.as_ref().unwrap().log_messages.clone(),
    };

    for (instruction_index, instruction_view) in transaction.walk_instructions().enumerate() {
        if !instruction_view.is_root() {
            continue;
        }

        collect_outer_instruction(&mut vec, timestamp, block_info, &tx_info, instruction_index, &instruction_view);
        collect_inner_instructions(&mut vec, timestamp, block_info, &tx_info, instruction_index, &instruction_view);
    }

    vec
}

fn collect_outer_instruction(vec: &mut Vec<InstructionCall>, timestamp: &BlockTimestamp, block_info: &BlockInfo, tx_info: &TxInfo, instruction_index: usize, instruction_view: &InstructionView) {
    let executing_account = base58::encode(instruction_view.program_id());
    // let account_arguments = to_string_array_to_string(&instruction_view.accounts());
    let account_arguments = instruction_view.accounts().iter().map(|arg| arg.to_string()).collect::<Vec<String>>();
    let data = Hex::encode(&instruction_view.data());

    vec.push(InstructionCall {
        block_time: Some(timestamp.time),
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),
        block_slot: block_info.slot,
        block_height: block_info.height,
        block_previous_block_hash: block_info.previous_block_hash.clone(),
        block_parent_slot: block_info.parent_slot,
        tx_id: tx_info.tx_id.clone(),
        tx_index: tx_info.tx_index,
        tx_signer: tx_info.tx_signer.to_string(),
        tx_success: tx_info.tx_success,
        log_messages: tx_info.log_messages.clone(),
        outer_instruction_index: instruction_index as u32,
        inner_instruction_index: -1,
        inner_executing_account: "".to_string(),
        outer_executing_account: executing_account.clone(),
        executing_account,
        is_inner: false,
        data,
        account_arguments,
        inner_instructions: build_inner_instructions_str(instruction_view),
    });
}

fn collect_inner_instructions(vec: &mut Vec<InstructionCall>, timestamp: &BlockTimestamp, block_info: &BlockInfo, tx_info: &TxInfo, instruction_index: usize, instruction_view: &InstructionView) {
    for (inner_index, inner_instruction) in instruction_view.inner_instructions().enumerate() {
        let inner_data = Hex::encode(inner_instruction.data());
        let executing_account = inner_instruction.program_id().to_string();
        let account_arguments = inner_instruction.accounts().iter().map(|arg| arg.to_string()).collect::<Vec<String>>();

        vec.push(InstructionCall {
            block_time: Some(timestamp.time),
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),
            block_slot: block_info.slot,
            block_height: block_info.height,
            block_previous_block_hash: block_info.previous_block_hash.clone(),
            block_parent_slot: block_info.parent_slot,
            tx_id: tx_info.tx_id.clone(),
            tx_index: tx_info.tx_index,
            tx_signer: tx_info.tx_signer.to_string(),
            tx_success: tx_info.tx_success,
            log_messages: tx_info.log_messages.clone(),
            outer_instruction_index: instruction_index as u32,
            inner_instruction_index: inner_index as i32,
            inner_executing_account: executing_account.clone(),
            outer_executing_account: instruction_view.program_id().to_string(),
            executing_account,
            is_inner: true,
            data: inner_data,
            account_arguments,
            inner_instructions: "".to_string(),
        });
    }
}

// TODO: Instead of building a string, return a complex array when supported by parquet sink
fn build_inner_instructions_str(instruction_view: &InstructionView) -> String {
    let inner_instructions: Vec<(String, String, Vec<String>)> = instruction_view
        .inner_instructions()
        .map(|inner_instruction| {
            (
                base58::encode(inner_instruction.data()),
                inner_instruction.program_id().to_string(),
                inner_instruction.accounts().iter().map(|arg| arg.to_string()).collect(),
            )
        })
        .collect();

    serde_json::to_string(&inner_instructions).expect("Failed to serialize inner instructions")
}

pub struct TxInfo {
    pub tx_id: String,
    pub tx_index: u32,
    pub tx_signer: String,
    pub tx_success: bool,
    pub log_messages: Vec<String>,
}
