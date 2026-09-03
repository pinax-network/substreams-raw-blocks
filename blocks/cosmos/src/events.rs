use common::structs::BlockIdentity;
use sha2::{Digest, Sha256};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::{public_key, Event as EventSource, TxResults};
use substreams_cosmos::Block;

use crate::pb::pinax::cosmos::v2::{Block as BlockRow, ConsensusParamUpdate, Event, Events, Message, Misbehavior, Transaction, ValidatorUpdate};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let header = block.header.clone().unwrap_or_default();
    let height = block.height.max(0) as u64;
    let last_block_id = header.last_block_id.clone().unwrap_or_default();
    let id = BlockIdentity::new(height, block.hash.clone(), height.saturating_sub(1), last_block_id.hash.clone(), &clock.timestamp);

    let mut events = Events::default();

    // block-level events (begin_block / end_block)
    for (event_index, event) in block.events.iter().enumerate() {
        events.events.extend(collect_event_attributes(event, "block", &[], None, event_index as u32, &id));
    }

    for (tx_index, raw_tx) in block.txs.iter().enumerate() {
        let tx_hash = Sha256::digest(raw_tx).to_vec();
        let result = block.tx_results.get(tx_index).cloned().unwrap_or_default();
        let messages = collect_messages(raw_tx, &tx_hash, tx_index as u32, &id);
        for (event_index, event) in result.events.iter().enumerate() {
            events
                .events
                .extend(collect_event_attributes(event, "transaction", &tx_hash, Some(tx_index as i32), event_index as u32, &id));
        }
        events.transactions.push(collect_transaction(&result, raw_tx, tx_hash, tx_index as u32, messages.len() as u32, &id));
        events.messages.extend(messages);
    }

    events.misbehaviors = collect_misbehaviors(&block, &id);
    events.validator_updates = collect_validator_updates(&block, &id);
    events.consensus_param_updates = collect_consensus_param_updates(&block, &id);
    events.blocks.push(collect_block(&block, &events, &id));
    Ok(events)
}

fn collect_block(block: &Block, events: &Events, id: &BlockIdentity) -> BlockRow {
    let header = block.header.clone().unwrap_or_default();
    let version = header.version.clone().unwrap_or_default();
    let last_block_id = header.last_block_id.clone().unwrap_or_default();
    let part_set = last_block_id.part_set_header.clone().unwrap_or_default();
    let successful = block.tx_results.iter().filter(|r| r.code == 0).count() as u32;

    BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        height: block.height,
        hash: block.hash.clone(),
        time: block.time.as_ref().map(|t| t.seconds).unwrap_or_default(),
        chain_id: header.chain_id.clone(),
        proposer_address: header.proposer_address.clone(),
        last_block_id_hash: last_block_id.hash.clone(),
        validators_hash: header.validators_hash.clone(),
        next_validators_hash: header.next_validators_hash.clone(),
        num_txs: block.txs.len() as u32,

        version_block: version.block,
        version_app: version.app,
        last_commit_hash: header.last_commit_hash.clone(),
        data_hash: header.data_hash.clone(),
        consensus_hash: header.consensus_hash.clone(),
        app_hash: header.app_hash.clone(),
        last_results_hash: header.last_results_hash.clone(),
        evidence_hash: header.evidence_hash.clone(),
        last_block_id_part_set_total: part_set.total,
        last_block_id_part_set_hash: part_set.hash.clone(),
        num_successful_txs: successful,
        num_failed_txs: block.tx_results.len() as u32 - successful,
        num_events: events.events.len() as u32,
        num_misbehaviors: block.misbehavior.len() as u32,
        num_validator_updates: block.validator_updates.len() as u32,
    }
}

fn collect_transaction(result: &TxResults, raw_tx: &[u8], tx_hash: Vec<u8>, index: u32, num_messages: u32, id: &BlockIdentity) -> Transaction {
    Transaction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        tx_hash,
        index,
        code: result.code,
        gas_wanted: result.gas_wanted,
        gas_used: result.gas_used,
        log: result.log.clone(),
        info: result.info.clone(),
        codespace: result.codespace.clone(),

        success: result.code == 0,
        data: result.data.clone(),
        raw_tx: raw_tx.to_vec(),
        num_events: result.events.len() as u32,
        num_messages,
    }
}

fn collect_event_attributes(event: &EventSource, source: &str, tx_hash: &[u8], tx_index: Option<i32>, event_index: u32, id: &BlockIdentity) -> Vec<Event> {
    event
        .attributes
        .iter()
        .enumerate()
        .map(|(attribute_index, attr)| Event {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            source: source.to_string(),
            tx_hash: tx_hash.to_vec(),
            tx_index,
            event_index,
            r#type: event.r#type.clone(),
            key: attr.key.clone(),
            value: attr.value.clone(),

            attribute_index: attribute_index as u32,
        })
        .collect()
}

// Only the `body.messages` part of `cosmos.tx.v1beta1.Tx` is needed.
#[derive(Clone, PartialEq, ::prost::Message)]
struct TxPartial {
    #[prost(message, optional, tag = "1")]
    body: Option<TxBodyPartial>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
struct TxBodyPartial {
    #[prost(message, repeated, tag = "1")]
    messages: Vec<::prost_types::Any>,
}

fn collect_messages(raw_tx: &[u8], tx_hash: &[u8], tx_index: u32, id: &BlockIdentity) -> Vec<Message> {
    let Ok(tx) = <TxPartial as prost::Message>::decode(raw_tx) else {
        return vec![];
    };
    tx.body
        .map(|body| body.messages)
        .unwrap_or_default()
        .into_iter()
        .enumerate()
        .map(|(message_index, message)| Message {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            tx_hash: tx_hash.to_vec(),
            tx_index,
            message_index: message_index as u32,
            type_url: message.type_url,
            value: message.value,
        })
        .collect()
}

fn misbehavior_type_text(value: i32) -> String {
    match value {
        1 => "DUPLICATE_VOTE",
        2 => "LIGHT_CLIENT_ATTACK",
        _ => "UNKNOWN",
    }
    .to_string()
}

fn collect_misbehaviors(block: &Block, id: &BlockIdentity) -> Vec<Misbehavior> {
    block
        .misbehavior
        .iter()
        .enumerate()
        .map(|(index, m)| {
            let validator = m.validator.clone().unwrap_or_default();
            Misbehavior {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                index: index as u32,
                r#type: misbehavior_type_text(m.r#type),
                validator_address: validator.address,
                validator_power: validator.power,
                height: m.height,
                time: m.time.clone(),
                total_voting_power: m.total_voting_power,
            }
        })
        .collect()
}

fn collect_validator_updates(block: &Block, id: &BlockIdentity) -> Vec<ValidatorUpdate> {
    block
        .validator_updates
        .iter()
        .enumerate()
        .map(|(index, update)| {
            let (pub_key, pub_key_type) = match update.pub_key.as_ref().and_then(|k| k.sum.as_ref()) {
                Some(public_key::Sum::Ed25519(bytes)) => (bytes.clone(), "ed25519"),
                Some(public_key::Sum::Secp256k1(bytes)) => (bytes.clone(), "secp256k1"),
                None => (vec![], ""),
            };
            ValidatorUpdate {
                block_num: id.block_num,
                block_id: id.block_id.clone(),
                parent_num: id.parent_num,
                parent_id: id.parent_id.clone(),
                timestamp: Some(id.timestamp.clone()),
                date: id.date.clone(),

                index: index as u32,
                pub_key,
                pub_key_type: pub_key_type.to_string(),
                power: update.power,
            }
        })
        .collect()
}

fn collect_consensus_param_updates(block: &Block, id: &BlockIdentity) -> Vec<ConsensusParamUpdate> {
    let Some(params) = block.consensus_param_updates.as_ref() else {
        return vec![];
    };
    vec![ConsensusParamUpdate {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        block_max_bytes: params.block.as_ref().map(|b| b.max_bytes),
        block_max_gas: params.block.as_ref().map(|b| b.max_gas),
        evidence_max_age_num_blocks: params.evidence.as_ref().map(|e| e.max_age_num_blocks),
        evidence_max_age_duration_seconds: params.evidence.as_ref().and_then(|e| e.max_age_duration.as_ref()).map(|d| d.seconds),
        evidence_max_bytes: params.evidence.as_ref().map(|e| e.max_bytes),
        validator_pub_key_types: params.validator.as_ref().map(|v| v.pub_key_types.clone()).unwrap_or_default(),
        app_version: params.version.as_ref().map(|v| v.app),
    }]
}
