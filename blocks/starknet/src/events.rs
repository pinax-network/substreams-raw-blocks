use common::{
    structs::BlockTimestamp,
    utils::{bytes_to_hex, u8_2d_vec_to_string_array},
};

use crate::{
    pb::{
        pinax::starknet::v1::Event,
        sf::starknet::r#type::v1::{Block, TransactionWithReceipt},
    },
    transactions::TransactionData,
    utils::{data_availability_mode_to_string, BlockHashes},
};

pub fn collect_events(block: &Block, transaction: &TransactionWithReceipt, timestamp: &BlockTimestamp, block_hashes: &BlockHashes, tx_data: &TransactionData) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    let l1_gas_price = block.l1_gas_price.as_ref().expect("L1 gas price missing");
    let l1_data_gas_price = block.l1_data_gas_price.as_ref().expect("L1 data gas price missing");

    if let Some(receipt) = &transaction.receipt {
        for (i, event) in receipt.events.iter().enumerate() {
            events.push(Event {
                block_time: timestamp.time.to_string(),
                block_number: timestamp.number,
                block_date: timestamp.date.clone(),
                block_hash: block_hashes.new_root.clone(),
                block_new_root: block_hashes.new_root.clone(),
                block_parent_hash: block_hashes.parent_hash.clone(),
                block_sequencer_address: block_hashes.sequencer_address.clone(),
                block_starknet_version: block.starknet_version.clone(),
                block_l1_da_mode: data_availability_mode_to_string(block.l1_da_mode),
                block_l1_data_gas_price_in_fri: l1_data_gas_price.price_in_fri.clone(),
                block_l1_data_gas_price_in_wei: l1_data_gas_price.price_in_wei.clone(),
                block_l1_gas_price_in_fri: l1_gas_price.price_in_fri.clone(),
                block_l1_gas_price_in_wei: l1_gas_price.price_in_wei.clone(),
                event_index: i as u32,
                tx_hash: tx_data.hash.clone(),
                data: u8_2d_vec_to_string_array(&event.data),
                keys: u8_2d_vec_to_string_array(&event.keys),
                from_address: bytes_to_hex(&event.from_address),
                class_hash: tx_data.class_hash.clone(),
            });
        }
    }

    events
}
