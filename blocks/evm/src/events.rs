use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::Block;

use crate::blocks::{collect_block, collect_uncles};
use crate::calls::{collect_call, collect_system_call};
use crate::logs::collect_logs;
use crate::params::Tables;
use crate::pb::pinax::evm::v2::Events;
use crate::state_changes::{
    collect_account_creations, collect_balance_changes, collect_block_balance_changes, collect_block_code_changes, collect_code_changes, collect_gas_changes, collect_keccak_preimages,
    collect_nonce_changes, collect_storage_changes, collect_system_account_creations, collect_system_balance_changes, collect_system_code_changes, collect_system_gas_changes,
    collect_system_keccak_preimages, collect_system_nonce_changes, collect_system_storage_changes,
};
use crate::transactions::{collect_access_lists, collect_set_code_authorizations, collect_transaction};

// All tables. `params` selects tables, see `params.rs` (e.g. `exclude=storage_changes,system_storage_changes`).
#[substreams::handlers::map]
pub fn map_events(params: String, clock: Clock, block: Block) -> Result<Events, Error> {
    let tables = Tables::from_params(&params)?;
    Ok(collect_events(&clock, &block, &tables))
}

// Firehose block passthrough, for `substreams estimate` of the raw block egress.
#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Block, Error> {
    Ok(block)
}

// Same as `map_block` with every `storage_changes` field removed, so that
// `substreams estimate` on both modules gives the egress saved by dropping storage changes.
#[substreams::handlers::map]
pub fn map_block_no_storage_changes(mut block: Block) -> Result<Block, Error> {
    crate::sizes::strip_storage_changes(&mut block);
    Ok(block)
}

pub fn block_identity(clock: &Clock, block: &Block) -> BlockIdentity {
    let parent_id = block.header.as_ref().map(|h| h.parent_hash.clone()).unwrap_or_default();
    BlockIdentity::new(block.number, block.hash.clone(), block.number.saturating_sub(1), parent_id, &clock.timestamp)
}

pub fn collect_events(clock: &Clock, block: &Block, tables: &Tables) -> Events {
    let id = block_identity(clock, block);
    let mut events = Events::default();

    if tables.has("blocks") {
        events.blocks.push(collect_block(block, &id));
    }
    if tables.has("uncles") {
        events.uncles = collect_uncles(block, &id);
    }

    for tx in &block.transaction_traces {
        if tables.has("transactions") {
            events.transactions.push(collect_transaction(tx, &id));
        }
        if tables.has("access_lists") {
            events.access_lists.extend(collect_access_lists(tx, &id));
        }
        if tables.has("set_code_authorizations") {
            events.set_code_authorizations.extend(collect_set_code_authorizations(tx, &id));
        }
        if tables.has("logs") {
            events.logs.extend(collect_logs(tx, &id));
        }

        // DetailLevel: EXTENDED only (calls are empty on BASE blocks)
        for call in &tx.calls {
            if tables.has("calls") {
                events.calls.push(collect_call(tx, call, &id));
            }
            if tables.has("balance_changes") {
                events.balance_changes.extend(collect_balance_changes(tx, call, &id));
            }
            if tables.has("code_changes") {
                events.code_changes.extend(collect_code_changes(tx, call, &id));
            }
            if tables.has("storage_changes") {
                events.storage_changes.extend(collect_storage_changes(tx, call, &id));
            }
            if tables.has("nonce_changes") {
                events.nonce_changes.extend(collect_nonce_changes(tx, call, &id));
            }
            if tables.has("gas_changes") {
                events.gas_changes.extend(collect_gas_changes(tx, call, &id));
            }
            if tables.has("account_creations") {
                events.account_creations.extend(collect_account_creations(tx, call, &id));
            }
            if tables.has("keccak_preimages") {
                events.keccak_preimages.extend(collect_keccak_preimages(tx, call, &id));
            }
        }
    }

    // Block-level data (no transaction): the block's own balance/code changes
    // (rewards, withdrawals, genesis...) and system calls introduced with Cancun.
    if tables.has("system_balance_changes") {
        events.system_balance_changes.extend(collect_block_balance_changes(block, &id));
    }
    if tables.has("system_code_changes") {
        events.system_code_changes.extend(collect_block_code_changes(block, &id));
    }
    for call in &block.system_calls {
        if tables.has("system_calls") {
            events.system_calls.push(collect_system_call(call, &id));
        }
        if tables.has("system_balance_changes") {
            events.system_balance_changes.extend(collect_system_balance_changes(call, &id));
        }
        if tables.has("system_code_changes") {
            events.system_code_changes.extend(collect_system_code_changes(call, &id));
        }
        if tables.has("system_storage_changes") {
            events.system_storage_changes.extend(collect_system_storage_changes(call, &id));
        }
        if tables.has("system_nonce_changes") {
            events.system_nonce_changes.extend(collect_system_nonce_changes(call, &id));
        }
        if tables.has("system_gas_changes") {
            events.system_gas_changes.extend(collect_system_gas_changes(call, &id));
        }
        if tables.has("system_account_creations") {
            events.system_account_creations.extend(collect_system_account_creations(call, &id));
        }
        if tables.has("system_keccak_preimages") {
            events.system_keccak_preimages.extend(collect_system_keccak_preimages(call, &id));
        }
    }

    events
}
