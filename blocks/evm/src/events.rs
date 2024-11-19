use common::utils::build_timestamp;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;

use substreams_ethereum::pb::eth::v2::Block;

use crate::account_creations::collect_account_creations;
use crate::balance_changes::collect_balance_changes;
use crate::blocks::{block_detail_to_string, collect_block};
use crate::code_changes::collect_code_changes;
use crate::creation_traces::collect_creation_traces;
use crate::gas_changes::collect_gas_changes;
use crate::logs::collect_logs;
use crate::nonce_changes::collect_nonce_changes;
use crate::pb::evm::Events;
use crate::storage_changes::collect_storage_changes;
use crate::traces::collect_traces;
use crate::transactions::collect_transactions;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let timestamp = build_timestamp(&clock);
    let detail_level = block_detail_to_string(block.detail_level);

    Ok(Events {
        blocks: vec![collect_block(&block, &timestamp)],
        transactions: collect_transactions(&block, &timestamp),
        logs: collect_logs(&block, &timestamp, &detail_level),
        traces: collect_traces(&block, &timestamp, &detail_level),
        balance_changes: collect_balance_changes(&block, &timestamp),
        storage_changes: collect_storage_changes(&block, &timestamp),
        code_changes: collect_code_changes(&block, &timestamp),
        account_creations: collect_account_creations(&block, &timestamp),
        nonce_changes: collect_nonce_changes(&block, &timestamp),
        gas_changes: collect_gas_changes(&block, &timestamp),
        creation_traces: collect_creation_traces(&block, &timestamp),
    })
}
