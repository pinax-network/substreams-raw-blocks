use common::structs::BlockIdentity;
use substreams_ethereum::pb::eth::v2::{Call as CallSource, TransactionTrace};

use crate::pb::pinax::evm::v2::{Call, SystemCall};
use crate::utils::{bigint_to_string, call_type_text};

// DetailLevel: EXTENDED
pub fn collect_call(tx: &TransactionTrace, call: &CallSource, id: &BlockIdentity) -> Call {
    Call {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        block_number: id.block_num,
        tx_hash: tx.hash.clone(),
        tx_index: tx.index,
        call_index: call.index,
        parent_index: call.parent_index,
        depth: call.depth,
        call_type: call_type_text(call.call_type),
        caller: call.caller.clone(),
        address: call.address.clone(),
        value: bigint_to_string(&call.value),
        gas_limit: call.gas_limit,
        gas_consumed: call.gas_consumed,
        input: call.input.clone(),
        output: call.return_data.clone(),
        status_failed: call.status_failed,
        status_reverted: call.status_reverted,
        state_reverted: call.state_reverted,
        executed_code: call.executed_code,
        suicide: call.suicide,

        address_delegates_to: call.address_delegates_to.clone(),
        failure_reason: call.failure_reason.clone(),
        begin_ordinal: call.begin_ordinal,
        end_ordinal: call.end_ordinal,
    }
}

// System calls (introduced with Cancun) run outside of any transaction.
pub fn collect_system_call(call: &CallSource, id: &BlockIdentity) -> SystemCall {
    SystemCall {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        block_number: id.block_num,
        call_index: call.index,
        parent_index: call.parent_index,
        depth: call.depth,
        call_type: call_type_text(call.call_type),
        caller: call.caller.clone(),
        address: call.address.clone(),
        value: bigint_to_string(&call.value),
        gas_limit: call.gas_limit,
        gas_consumed: call.gas_consumed,
        input: call.input.clone(),
        output: call.return_data.clone(),
        status_failed: call.status_failed,
        status_reverted: call.status_reverted,
        state_reverted: call.state_reverted,
        executed_code: call.executed_code,
        suicide: call.suicide,

        address_delegates_to: call.address_delegates_to.clone(),
        failure_reason: call.failure_reason.clone(),
        begin_ordinal: call.begin_ordinal,
        end_ordinal: call.end_ordinal,
    }
}
