use common::{structs::BlockTimestamp, utils::bytes_to_hex};
use serde_json::json;
use substreams::Hex;

use crate::{
    pb::{
        pinax::starknet::v1::Block as BlockOutput,
        sf::starknet::r#type::v1::{Block, StateDiff},
    },
    utils::BlockHashes,
};

pub fn collect_block(block: &Block, timestamp: &BlockTimestamp, block_hashes: &BlockHashes) -> BlockOutput {
    let l1_data_gas_price = block.l1_data_gas_price.as_ref().expect("L1 data gas price missing");
    let l1_gas_price = block.l1_gas_price.as_ref().expect("L1 gas price missing");
    let state_diff = block.state_update.as_ref().expect("State diff missing").state_diff.as_ref().expect("State diff missing");

    BlockOutput {
        time: timestamp.time.to_string(),
        number: timestamp.number,
        date: timestamp.date.clone(),
        hash: timestamp.hash.clone(),
        l1_da_mode: l1_da_mode_to_string(block.l1_da_mode),
        l1_data_gas_price_in_fri: l1_data_gas_price.price_in_fri.clone(),
        l1_data_gas_price_in_wei: l1_data_gas_price.price_in_wei.clone(),
        l1_gas_price_in_fri: l1_gas_price.price_in_fri.clone(),
        l1_gas_price_in_wei: l1_gas_price.price_in_wei.clone(),
        starknet_version: block.starknet_version.clone(),
        tx_count: block.transactions.len() as u32,
        new_root: block_hashes.new_root.clone(),
        parent_hash: block_hashes.parent_hash.clone(),
        sequencer_address: block_hashes.sequencer_address.clone(),
        state_diff: state_diff_to_string(state_diff),
    }
}

pub fn l1_da_mode_to_string(l1_da_mode: i32) -> String {
    match l1_da_mode {
        0 => "Unknown".to_string(),
        1 => "Calldata".to_string(),
        2 => "Blob".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn state_diff_to_string(state_diff: &StateDiff) -> String {
    let json_str = json!({
        "deployed_contracts": state_diff.deployed_contracts.iter().map(|contract| {
            json!({
                "address": bytes_to_hex(&contract.address),
                "class_hash": bytes_to_hex(&contract.class_hash),
            })
        }).collect::<Vec<_>>(),
        "storage_diffs": state_diff.storage_diffs.iter().map(|diff| {
            json!({
                "address": bytes_to_hex(&diff.address),
                "storage_entries": diff.storage_entries.iter().map(|entry| {
                    json!({
                        "key": bytes_to_hex(&entry.key),
                        "value": bytes_to_hex(&entry.value),
                    })
                }).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>(),
        "nonces": state_diff.nonces.iter().map(|nonce| {
            json!({
                "contract_address": bytes_to_hex(&nonce.contract_address),
                "nonce": Hex::encode(&nonce.nonce),
            })
        }).collect::<Vec<_>>(),
        "deprecated_declared_classes": state_diff.deprecated_declared_classes.iter().map(|class_hash| {
            bytes_to_hex(class_hash)
        }).collect::<Vec<_>>(),
        "declared_classes": state_diff.declared_classes.iter().map(|class| {
            json!({
                "class_hash": bytes_to_hex(&class.class_hash),
                "compiled_class_hash": bytes_to_hex(&class.compiled_class_hash),

            })
        }).collect::<Vec<_>>(),
        "replaced_classes": state_diff.replaced_classes.iter().map(|class| {
            json!({
                "contract_address": bytes_to_hex(&class.contract_address),
                "class_hash": bytes_to_hex(&class.class_hash),
            })
        }).collect::<Vec<_>>(),
    });

    json_str.to_string()
}
