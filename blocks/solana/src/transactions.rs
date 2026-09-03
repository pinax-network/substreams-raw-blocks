use common::structs::BlockIdentity;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

use crate::pb::pinax::solana::v2::{Transaction, VoteTransaction};
use crate::tx_errors::TransactionErrorDecoder;
use crate::utils::non_empty_bytes;

// All transactions, including failed ones (`success = false`, `err`/`error` set).
pub fn collect_transaction(tx: &ConfirmedTransaction, index: u32, id: &BlockIdentity) -> Transaction {
    let transaction = tx.transaction.clone().unwrap_or_default();
    let message = transaction.message.clone().unwrap_or_default();
    let meta = tx.meta.clone().unwrap_or_default();
    // Some endpoints send `TransactionError { err: [] }` for successful transactions.
    let err = meta.err.as_ref().and_then(|e| non_empty_bytes(&e.err));

    Transaction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        slot: id.block_num,
        transaction_index: index,
        signature: transaction.signatures.first().cloned().unwrap_or_default(),
        num_signatures: transaction.signatures.len() as u32,
        fee: meta.fee,
        success: err.is_none(),
        error: err.as_ref().map(|e| decode_transaction_error(e)),
        err,
        compute_units_consumed: meta.compute_units_consumed,
        log_messages: meta.log_messages.clone(),
        pre_balances: meta.pre_balances.clone(),
        post_balances: meta.post_balances.clone(),
        cost_units: meta.cost_units,
        return_data_program_id: meta.return_data.as_ref().map(|r| r.program_id.clone()),
        return_data: meta.return_data.as_ref().map(|r| r.data.clone()),

        signatures: transaction.signatures.clone(),
        signer: message.account_keys.first().cloned().unwrap_or_default(),
        num_instructions: message.instructions.len() as u32,
        num_inner_instructions: meta.inner_instructions.iter().map(|i| i.instructions.len()).sum::<usize>() as u32,
    }
}

pub fn to_vote_transaction(t: Transaction) -> VoteTransaction {
    VoteTransaction {
        block_num: t.block_num,
        block_id: t.block_id,
        parent_num: t.parent_num,
        parent_id: t.parent_id,
        timestamp: t.timestamp,
        date: t.date,
        slot: t.slot,
        transaction_index: t.transaction_index,
        signature: t.signature,
        num_signatures: t.num_signatures,
        fee: t.fee,
        err: t.err,
        success: t.success,
        compute_units_consumed: t.compute_units_consumed,
        log_messages: t.log_messages,
        pre_balances: t.pre_balances,
        post_balances: t.post_balances,
        cost_units: t.cost_units,
        return_data_program_id: t.return_data_program_id,
        return_data: t.return_data,
        signatures: t.signatures,
        error: t.error,
        signer: t.signer,
        num_instructions: t.num_instructions,
        num_inner_instructions: t.num_inner_instructions,
    }
}

pub fn decode_transaction_error(err: &[u8]) -> String {
    match TransactionErrorDecoder::decode_error(err) {
        Ok(decoded) => TransactionErrorDecoder::format_error(&decoded),
        Err(e) => format!("Error decoding transaction error: {:?}", e),
    }
}
