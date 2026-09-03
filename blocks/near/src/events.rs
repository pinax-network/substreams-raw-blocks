use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_near::pb::sf::near::r#type::v1::{
    action, receipt, state_change_value, Action as ActionSource, Block, ChunkHeader, ExecutionOutcomeWithId, IndexerExecutionOutcomeWithReceipt, IndexerTransactionWithOutcome, StateChangeWithCause,
};

use crate::pb::pinax::near::v2::{Action, Block as BlockRow, Chunk, Events, Receipt, StateChange, Transaction};
use crate::utils::*;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let header = block.header.clone().unwrap_or_default();
    let id = BlockIdentity::new(header.height, hash_bytes(&header.hash), header.prev_height, hash_bytes(&header.prev_hash), &clock.timestamp);

    let mut events = Events::default();
    let mut tx_index = 0u32;
    let mut receipt_index = 0u32;

    // Shards may be listed without a chunk ("missing chunk"); those are skipped.
    for shard in &block.shards {
        if let Some(chunk) = &shard.chunk {
            if let Some(chunk_header) = &chunk.header {
                events
                    .chunks
                    .push(collect_chunk(chunk_header, &chunk.author, chunk.transactions.len() as u32, chunk.receipts.len() as u32, &id));
            }
            for tx in &chunk.transactions {
                events.transactions.push(collect_transaction(shard.shard_id, tx, tx_index, &id));
                events.actions.extend(collect_transaction_actions(shard.shard_id, tx, &id));
                tx_index += 1;
            }
        }
        for outcome in &shard.receipt_execution_outcomes {
            events.receipts.push(collect_receipt(shard.shard_id, outcome, receipt_index, &id));
            events.actions.extend(collect_receipt_actions(shard.shard_id, outcome, &id));
            receipt_index += 1;
        }
    }
    events.state_changes = block.state_changes.iter().enumerate().map(|(i, sc)| collect_state_change(sc, i as u32, &id)).collect();
    events.blocks.push(collect_block(&block, tx_index, receipt_index, &id));
    Ok(events)
}

fn collect_block(block: &Block, num_transactions: u32, num_receipts: u32, id: &BlockIdentity) -> BlockRow {
    let h = block.header.clone().unwrap_or_default();
    BlockRow {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        height: h.height,
        hash: hash_bytes(&h.hash),
        prev_hash: hash_bytes(&h.prev_hash),
        prev_height: h.prev_height,
        epoch_id: hash_bytes(&h.epoch_id),
        author: block.author.clone(),
        gas_price: bigint_to_string(&h.gas_price),
        total_supply: bigint_to_string(&h.total_supply),
        chunks_included: h.chunks_included,
        latest_protocol_version: h.latest_protocol_version,

        next_epoch_id: hash_bytes(&h.next_epoch_id),
        prev_state_root: hash_bytes(&h.prev_state_root),
        chunk_receipts_root: hash_bytes(&h.chunk_receipts_root),
        chunk_headers_root: hash_bytes(&h.chunk_headers_root),
        chunk_tx_root: hash_bytes(&h.chunk_tx_root),
        outcome_root: hash_bytes(&h.outcome_root),
        challenges_root: hash_bytes(&h.challenges_root),
        timestamp_nanosec: h.timestamp_nanosec,
        random_value: hash_bytes(&h.random_value),
        chunk_mask: h.chunk_mask.clone(),
        block_ordinal: h.block_ordinal,
        last_final_block_height: h.last_final_block_height,
        last_final_block: hash_bytes(&h.last_final_block),
        last_ds_final_block_height: h.last_ds_final_block_height,
        last_ds_final_block: hash_bytes(&h.last_ds_final_block),
        next_bp_hash: hash_bytes(&h.next_bp_hash),
        block_merkle_root: hash_bytes(&h.block_merkle_root),
        epoch_sync_data_hash: h.epoch_sync_data_hash.clone(),
        signature: h.signature.as_ref().map(|s| s.bytes.clone()).unwrap_or_default(),
        num_approvals: h.approvals.len() as u32,
        num_validator_proposals: h.validator_proposals.len() as u32,
        num_shards: block.shards.len() as u32,
        num_transactions,
        num_receipts,
        num_state_changes: block.state_changes.len() as u32,
    }
}

fn collect_chunk(h: &ChunkHeader, author: &str, num_transactions: u32, num_receipts: u32, id: &BlockIdentity) -> Chunk {
    Chunk {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        shard_id: h.shard_id,
        chunk_hash: h.chunk_hash.clone(),
        prev_state_root: h.prev_state_root.clone(),
        gas_used: h.gas_used,
        gas_limit: h.gas_limit,
        height_created: h.height_created,
        height_included: h.height_included,
        encoded_length: h.encoded_length,
        author: author.to_string(),

        prev_block_hash: h.prev_block_hash.clone(),
        outcome_root: h.outcome_root.clone(),
        encoded_merkle_root: h.encoded_merkle_root.clone(),
        validator_reward: bigint_to_string(&h.validator_reward),
        balance_burnt: bigint_to_string(&h.balance_burnt),
        outgoing_receipts_root: h.outgoing_receipts_root.clone(),
        tx_root: h.tx_root.clone(),
        signature: h.signature.as_ref().map(|s| s.bytes.clone()).unwrap_or_default(),
        num_transactions,
        num_receipts,
    }
}

fn outcome_of(o: Option<&ExecutionOutcomeWithId>) -> Option<&substreams_near::pb::sf::near::r#type::v1::ExecutionOutcome> {
    o.and_then(|o| o.outcome.as_ref())
}

fn collect_transaction(shard_id: u64, tx: &IndexerTransactionWithOutcome, index: u32, id: &BlockIdentity) -> Transaction {
    let t = tx.transaction.clone().unwrap_or_default();
    let outcome_with_id = tx.outcome.as_ref().and_then(|o| o.execution_outcome.as_ref());
    let outcome = outcome_of(outcome_with_id);
    Transaction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        hash: hash_bytes(&t.hash),
        signer_id: t.signer_id.clone(),
        receiver_id: t.receiver_id.clone(),
        shard_id,
        nonce: t.nonce,
        actions: t.actions.iter().map(action_type_name).collect::<Vec<_>>().join(","),
        status: execution_status(outcome).to_string(),
        gas_burnt: outcome.map(|o| o.gas_burnt).unwrap_or_default(),

        index,
        public_key: public_key_bytes(&t.public_key),
        public_key_type: public_key_type(&t.public_key),
        signature: t.signature.as_ref().map(|s| s.bytes.clone()).unwrap_or_default(),
        tokens_burnt: bigint_to_string(&outcome.and_then(|o| o.tokens_burnt.clone())),
        executor_id: outcome.map(|o| o.executor_id.clone()).unwrap_or_default(),
        outcome_receipt_ids: outcome.map(|o| o.receipt_ids.iter().map(|r| r.bytes.clone()).collect()).unwrap_or_default(),
        logs: outcome.map(|o| o.logs.clone()).unwrap_or_default(),
        num_actions: t.actions.len() as u32,
        failure: execution_failure(outcome),
    }
}

fn collect_receipt(shard_id: u64, r: &IndexerExecutionOutcomeWithReceipt, index: u32, id: &BlockIdentity) -> Receipt {
    let receipt = r.receipt.clone().unwrap_or_default();
    let outcome_with_id = r.execution_outcome.as_ref();
    let outcome = outcome_of(outcome_with_id);
    let (kind, action, data) = match &receipt.receipt {
        Some(receipt::Receipt::Action(a)) => ("action", Some(a), None),
        Some(receipt::Receipt::Data(d)) => ("data", None, Some(d)),
        None => ("", None, None),
    };
    Receipt {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        receipt_id: hash_bytes(&receipt.receipt_id),
        predecessor_id: receipt.predecessor_id.clone(),
        receiver_id: receipt.receiver_id.clone(),
        shard_id,
        status: execution_status(outcome).to_string(),
        gas_burnt: outcome.map(|o| o.gas_burnt).unwrap_or_default(),
        executor_id: outcome.map(|o| o.executor_id.clone()).unwrap_or_default(),

        index,
        kind: kind.to_string(),
        signer_id: action.map(|a| a.signer_id.clone()).unwrap_or_default(),
        signer_public_key: action.map(|a| public_key_bytes(&a.signer_public_key)).unwrap_or_default(),
        gas_price: bigint_to_string(&action.and_then(|a| a.gas_price.clone())),
        input_data_ids: action.map(|a| a.input_data_ids.iter().map(|h| h.bytes.clone()).collect()).unwrap_or_default(),
        output_data_receivers: action.map(|a| a.output_data_receivers.iter().map(|d| d.receiver_id.clone()).collect()).unwrap_or_default(),
        data_id: data.map(|d| hash_bytes(&d.data_id)).unwrap_or_default(),
        data: data.map(|d| d.data.clone()).unwrap_or_default(),
        tokens_burnt: bigint_to_string(&outcome.and_then(|o| o.tokens_burnt.clone())),
        logs: outcome.map(|o| o.logs.clone()).unwrap_or_default(),
        outcome_receipt_ids: outcome.map(|o| o.receipt_ids.iter().map(|h| h.bytes.clone()).collect()).unwrap_or_default(),
        outcome_block_hash: outcome_with_id.map(|o| hash_bytes(&o.block_hash)).unwrap_or_default(),
        num_actions: action.map(|a| a.actions.len()).unwrap_or_default() as u32,
        failure: execution_failure(outcome),
    }
}

fn collect_transaction_actions(shard_id: u64, tx: &IndexerTransactionWithOutcome, id: &BlockIdentity) -> Vec<Action> {
    let Some(t) = tx.transaction.as_ref() else {
        return vec![];
    };
    t.actions
        .iter()
        .enumerate()
        .map(|(i, a)| collect_action("transaction", hash_bytes(&t.hash), i as u32, a, &t.signer_id, &t.receiver_id, shard_id, id))
        .collect()
}

fn collect_receipt_actions(shard_id: u64, r: &IndexerExecutionOutcomeWithReceipt, id: &BlockIdentity) -> Vec<Action> {
    let Some(receipt) = r.receipt.as_ref() else {
        return vec![];
    };
    let Some(receipt::Receipt::Action(action_receipt)) = &receipt.receipt else {
        return vec![];
    };
    action_receipt
        .actions
        .iter()
        .enumerate()
        .map(|(i, a)| collect_action("receipt", hash_bytes(&receipt.receipt_id), i as u32, a, &action_receipt.signer_id, &receipt.receiver_id, shard_id, id))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn collect_action(source: &str, parent_hash: Vec<u8>, action_index: u32, a: &ActionSource, signer_id: &str, receiver_id: &str, shard_id: u64, id: &BlockIdentity) -> Action {
    let mut row = Action {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        source: source.to_string(),
        parent_hash,
        action_index,
        r#type: action_type_name(a).to_string(),
        signer_id: signer_id.to_string(),
        receiver_id: receiver_id.to_string(),
        shard_id,
        ..Default::default()
    };
    match &a.action {
        Some(action::Action::FunctionCall(f)) => {
            row.method_name = Some(f.method_name.clone());
            row.args = Some(f.args.clone());
            row.gas = Some(f.gas);
            row.deposit = optional_bigint_to_string(&f.deposit);
        }
        Some(action::Action::Transfer(t)) => row.deposit = optional_bigint_to_string(&t.deposit),
        Some(action::Action::Stake(s)) => {
            row.stake = optional_bigint_to_string(&s.stake);
            row.public_key = Some(public_key_bytes(&s.public_key));
        }
        Some(action::Action::AddKey(k)) => {
            row.public_key = Some(public_key_bytes(&k.public_key));
            row.access_key_permission = access_key_json(&k.access_key);
        }
        Some(action::Action::DeleteKey(k)) => row.public_key = Some(public_key_bytes(&k.public_key)),
        Some(action::Action::DeleteAccount(d)) => row.beneficiary_id = Some(d.beneficiary_id.clone()),
        Some(action::Action::DeployContract(c)) => row.code = Some(c.code.clone()),
        Some(action::Action::Delegate(d)) => {
            if let Some(da) = d.delegate_action.as_ref() {
                row.delegate_sender_id = Some(da.sender_id.clone());
                row.delegate_receiver_id = Some(da.receiver_id.clone());
                row.delegate_nonce = Some(da.nonce);
                row.delegate_max_block_height = Some(da.max_block_height);
                row.delegate_num_actions = Some(da.actions.len() as u32);
                row.public_key = Some(public_key_bytes(&da.public_key));
            }
        }
        Some(action::Action::CreateAccount(_)) | None => {}
    }
    row
}

fn collect_state_change(sc: &StateChangeWithCause, index: u32, id: &BlockIdentity) -> StateChange {
    let value = sc.value.clone().unwrap_or_default();
    let cause = sc.cause.clone().unwrap_or_default();
    let (cause_tx_hash, cause_receipt_hash) = state_change_cause_hashes(&cause);
    let mut row = StateChange {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        r#type: state_change_type(&value).to_string(),
        cause: state_change_cause(&cause).to_string(),
        index,
        cause_tx_hash,
        cause_receipt_hash,
        ..Default::default()
    };
    match &value.value {
        Some(state_change_value::Value::AccountUpdate(v)) => {
            row.account_id = v.account_id.clone();
            if let Some(acc) = &v.account {
                row.account_amount = optional_bigint_to_string(&acc.amount);
                row.account_locked = optional_bigint_to_string(&acc.locked);
                row.account_code_hash = hash_bytes(&acc.code_hash);
                row.account_storage_usage = Some(acc.storage_usage);
            }
        }
        Some(state_change_value::Value::AccountDeletion(v)) => row.account_id = v.account_id.clone(),
        Some(state_change_value::Value::AccessKeyUpdate(v)) => {
            row.account_id = v.account_id.clone();
            row.public_key = public_key_bytes(&v.public_key);
            row.access_key_nonce = v.access_key.as_ref().map(|k| k.nonce);
            row.access_key_permission = access_key_json(&v.access_key);
        }
        Some(state_change_value::Value::AccessKeyDeletion(v)) => {
            row.account_id = v.account_id.clone();
            row.public_key = public_key_bytes(&v.public_key);
        }
        Some(state_change_value::Value::DataUpdate(v)) => {
            row.account_id = v.account_id.clone();
            row.key_base64 = base64_encode(&v.key);
            row.value_base64 = base64_encode(&v.value);
        }
        Some(state_change_value::Value::DataDeletion(v)) => {
            row.account_id = v.account_id.clone();
            row.key_base64 = base64_encode(&v.key);
        }
        Some(state_change_value::Value::ContractCodeUpdate(v)) => {
            row.account_id = v.account_id.clone();
            row.contract_code_size = Some(v.code.len() as u32);
        }
        Some(state_change_value::Value::ContractDeletion(v)) => row.account_id = v.account_id.clone(),
        None => {}
    }
    row
}
