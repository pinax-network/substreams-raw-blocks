use common::structs::BlockTimestamp;

use substreams::Hex;
use substreams_antelope::pb::TransactionTrace;
use substreams_antelope::Block;

use crate::pb::pinax::antelope::v1::Action;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn collect_tx_actions(block: &Block, transaction: &TransactionTrace, timestamp: &BlockTimestamp, tx_success: bool) -> Vec<Action> {
    let header = block.header.clone().unwrap_or_default();
    let mut actions: Vec<Action> = Vec::new();

    for trace in transaction.action_traces.iter() {
        let action = trace.action.clone().unwrap_or_default();
        let receipt = trace.receipt.clone().unwrap_or_default();

        // auth sequence
        let auth_sequence = receipt.auth_sequence.iter().map(|seq| seq.sequence).collect::<Vec<u64>>();
        let auth_sequence_account_name = receipt.auth_sequence.iter().map(|seq| seq.clone().account_name).collect::<Vec<String>>();

        // ram deltas
        let account_ram_deltas = trace.account_ram_deltas.iter().map(|account_ram| account_ram.delta).collect::<Vec<i64>>();
        let account_ram_deltas_account = trace.account_ram_deltas.iter().map(|account_ram| account_ram.clone().account).collect::<Vec<String>>();

        actions.push(Action {
            // block
            block_time: timestamp.time.to_string(),
            block_number: timestamp.number,
            block_hash: timestamp.hash.clone(),
            block_date: timestamp.date.clone(),

            // tranasction
            tx_hash: transaction.id.clone(),
            tx_success,

            // receipt
            abi_sequence: receipt.abi_sequence,
            code_sequence: receipt.code_sequence,
            digest: receipt.digest,
            global_sequence: receipt.global_sequence,
            receipt_receiver: receipt.receiver,
            recv_sequence: receipt.recv_sequence,

            // auth sequence
            auth_sequence,
            auth_sequence_account_name,

            // account ram deltas
            account_ram_deltas,
            account_ram_deltas_account,

            // action
            account: action.account,
            name: action.name,
            json_data: action.json_data,
            raw_data: Hex::encode(&action.raw_data),
            index: trace.execution_index,
            action_ordinal: trace.action_ordinal,
            receiver: trace.receiver.clone(),
            context_free: trace.context_free,
            elapsed: trace.elapsed,
            console: trace.console.clone(),
            raw_return_value: Hex::encode(&trace.raw_return_value),
            json_return_value: trace.json_return_value.clone(),
            creator_action_ordinal: trace.creator_action_ordinal,
            closest_unnotified_ancestor_action_ordinal: trace.closest_unnotified_ancestor_action_ordinal,
            action_mroot: Hex::encode(&header.action_mroot),
        });
    }

    actions
}
