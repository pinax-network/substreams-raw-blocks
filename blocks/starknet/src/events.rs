use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;

use crate::pb::pinax::starknet::v2::{Block as BlockRow, DeclaredClass, DeployedContract, Event, Events, MessageSent, NonceDiff, ReplacedClass, StorageDiff};
use crate::pb::sf::starknet::r#type::v1::{Block, L1DaMode};
use crate::transactions::{collect_transaction, felt_to_uint256};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let id = BlockIdentity::new(
        block.block_number,
        block.block_hash.clone(),
        block.block_number.saturating_sub(1),
        block.parent_hash.clone(),
        &clock.timestamp,
    );
    let mut events = Events::default();

    for (index, tx) in block.transactions.iter().enumerate() {
        let index = index as u32;
        let receipt = tx.receipt.clone().unwrap_or_default();
        events.transactions.push(collect_transaction(tx, index, &id));
        for (event_index, e) in receipt.events.iter().enumerate() {
            events.events.push(Event {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),
                tx_hash: receipt.transaction_hash.clone(),
                tx_index: index,
                event_index: event_index as u32,
                from_address: e.from_address.clone(),
                keys: e.keys.clone(),
                data: e.data.clone(),
            });
        }
        for (message_index, m) in receipt.messages_sent.iter().enumerate() {
            events.messages_sent.push(MessageSent {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),
                tx_hash: receipt.transaction_hash.clone(),
                tx_index: index,
                message_index: message_index as u32,
                from_address: m.from_address.clone(),
                to_address: m.to_address.clone(),
                payload: m.payload.clone(),
            });
        }
    }

    let state_update = block.state_update.clone().unwrap_or_default();
    let diff = state_update.state_diff.clone().unwrap_or_default();
    let mut storage_index = 0u32;
    for contract in &diff.storage_diffs {
        for entry in &contract.storage_entries {
            events.storage_diffs.push(StorageDiff {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),
                index: storage_index,
                contract_address: contract.address.clone(),
                key: entry.key.clone(),
                value: entry.value.clone(),
            });
            storage_index += 1;
        }
    }
    events.declared_classes = diff
        .declared_classes
        .iter()
        .enumerate()
        .map(|(i, c)| DeclaredClass {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),
            index: i as u32,
            class_hash: c.class_hash.clone(),
            compiled_class_hash: c.compiled_class_hash.clone(),
        })
        .collect();
    events.deployed_contracts = diff
        .deployed_contracts
        .iter()
        .enumerate()
        .map(|(i, c)| DeployedContract {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),
            index: i as u32,
            address: c.address.clone(),
            class_hash: c.class_hash.clone(),
        })
        .collect();
    events.replaced_classes = diff
        .replaced_classes
        .iter()
        .enumerate()
        .map(|(i, c)| ReplacedClass {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),
            index: i as u32,
            contract_address: c.contract_address.clone(),
            class_hash: c.class_hash.clone(),
        })
        .collect();
    events.nonce_diffs = diff
        .nonces
        .iter()
        .enumerate()
        .map(|(i, n)| NonceDiff {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),
            index: i as u32,
            contract_address: n.contract_address.clone(),
            nonce: felt_to_uint256(&n.nonce),
        })
        .collect();

    let l1_gas = block.l1_gas_price.clone().unwrap_or_default();
    let l1_data_gas = block.l1_data_gas_price.clone().unwrap_or_default();
    events.blocks.push(BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        block_number: block.block_number,
        block_hash: block.block_hash.clone(),
        parent_hash: block.parent_hash.clone(),
        new_root: block.new_root.clone(),
        old_root: state_update.old_root.clone(),
        block_timestamp: block.timestamp,
        sequencer_address: block.sequencer_address.clone(),
        l1_gas_price_in_fri: felt_to_uint256(&l1_gas.price_in_fri),
        l1_gas_price_in_wei: felt_to_uint256(&l1_gas.price_in_wei),
        l1_data_gas_price_in_fri: felt_to_uint256(&l1_data_gas.price_in_fri),
        l1_data_gas_price_in_wei: felt_to_uint256(&l1_data_gas.price_in_wei),
        l1_da_mode: L1DaMode::try_from(block.l1_da_mode)
            .map(|m| m.as_str_name().trim_start_matches("L1_DA_MODE_").to_string())
            .unwrap_or_else(|_| "UNKNOWN".into()),
        starknet_version: block.starknet_version.clone(),
        num_transactions: block.transactions.len() as u32,
        num_events: events.events.len() as u32,
        num_messages_sent: events.messages_sent.len() as u32,
        num_storage_diffs: events.storage_diffs.len() as u32,
        num_declared_classes: diff.declared_classes.len() as u32,
        num_deprecated_declared_classes: diff.deprecated_declared_classes.len() as u32,
        num_deployed_contracts: diff.deployed_contracts.len() as u32,
        num_replaced_classes: diff.replaced_classes.len() as u32,
        num_nonce_diffs: diff.nonces.len() as u32,
        deprecated_declared_classes: diff.deprecated_declared_classes.clone(),
    });
    Ok(events)
}
