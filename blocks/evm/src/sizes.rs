// Per-block payload sizes of the Firehose block, used to estimate egress with
// and without specific data families (e.g. "how much do storage changes cost?").
//
// Sizes are exact: each family is measured as `encoded_len(block) -
// encoded_len(block with that family removed)`, so protobuf framing is included.
use prost::Message;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Block, Call};

use crate::events::block_identity;
use crate::pb::pinax::evm::v2::{BlockSize, BlockSizes};
use crate::utils::detail_level_text;

#[substreams::handlers::map]
pub fn map_block_sizes(clock: Clock, block: Block) -> Result<BlockSizes, Error> {
    let id = block_identity(&clock, &block);
    let size_bytes = block.encoded_len() as u64;
    let size_without = |f: fn(&mut Block)| -> u64 {
        let mut stripped = block.clone();
        f(&mut stripped);
        size_bytes - stripped.encoded_len() as u64
    };

    let all_calls = || block.transaction_traces.iter().flat_map(|tx| tx.calls.iter()).chain(block.system_calls.iter());
    let storage_changes_bytes = size_without(strip_storage_changes);

    Ok(BlockSizes {
        block_sizes: vec![BlockSize {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            detail_level: detail_level_text(block.detail_level),
            size_bytes,
            size_without_storage_changes: size_bytes - storage_changes_bytes,
            storage_changes_bytes,
            keccak_preimages_bytes: size_without(|b| for_each_call(b, |c| c.keccak_preimages.clear())),
            gas_changes_bytes: size_without(|b| for_each_call(b, |c| c.gas_changes.clear())),
            balance_changes_bytes: size_without(|b| {
                b.balance_changes.clear();
                for_each_call(b, |c| c.balance_changes.clear())
            }),
            nonce_changes_bytes: size_without(|b| for_each_call(b, |c| c.nonce_changes.clear())),
            code_changes_bytes: size_without(|b| {
                b.code_changes.clear();
                for_each_call(b, |c| c.code_changes.clear())
            }),
            logs_bytes: size_without(|b| {
                for tx in b.transaction_traces.iter_mut() {
                    if let Some(receipt) = tx.receipt.as_mut() {
                        receipt.logs.clear();
                    }
                }
                for_each_call(b, |c| c.logs.clear())
            }),
            calls_bytes: size_without(|b| {
                for tx in b.transaction_traces.iter_mut() {
                    tx.calls.clear();
                }
                b.system_calls.clear();
            }),

            num_transactions: block.transaction_traces.len() as u32,
            num_calls: all_calls().count() as u32,
            num_logs: block.transaction_traces.iter().filter_map(|tx| tx.receipt.as_ref()).map(|r| r.logs.len()).sum::<usize>() as u32,
            num_storage_changes: all_calls().map(|c| c.storage_changes.len()).sum::<usize>() as u32,
            num_balance_changes: (block.balance_changes.len() + all_calls().map(|c| c.balance_changes.len()).sum::<usize>()) as u32,
            num_gas_changes: all_calls().map(|c| c.gas_changes.len()).sum::<usize>() as u32,
            num_keccak_preimages: all_calls().map(|c| c.keccak_preimages.len()).sum::<usize>() as u32,
        }],
    })
}

pub fn strip_storage_changes(block: &mut Block) {
    for_each_call(block, |call| call.storage_changes.clear());
}

// Data families that `map_block_trimmed` can remove from the Firehose block.
pub const FAMILIES: [&str; 8] = [
    "storage_changes",
    "keccak_preimages",
    "gas_changes",
    "balance_changes",
    "nonce_changes",
    "code_changes",
    "account_creations",
    "logs",
];

// Firehose block with the families named in `params` removed, e.g.
// `exclude=storage_changes,gas_changes,keccak_preimages`, for `substreams estimate`
// of any trimming combination. `exclude=calls` drops the whole call trees.
#[substreams::handlers::map]
pub fn map_block_trimmed(params: String, mut block: Block) -> Result<Block, Error> {
    for pair in params.split('&').map(str::trim).filter(|p| !p.is_empty()) {
        let Some(("exclude", list)) = pair.split_once('=').map(|(k, v)| (k.trim(), v)) else {
            return Err(Error::msg(format!("invalid param {pair:?}, expected exclude=<family>[,<family>...]")));
        };
        for family in list.split(',').map(str::trim).filter(|f| !f.is_empty()) {
            strip_family(&mut block, family)?;
        }
    }
    Ok(block)
}

fn strip_family(block: &mut Block, family: &str) -> Result<(), Error> {
    match family {
        "storage_changes" => strip_storage_changes(block),
        "keccak_preimages" => for_each_call(block, |c| c.keccak_preimages.clear()),
        "gas_changes" => for_each_call(block, |c| c.gas_changes.clear()),
        "balance_changes" => {
            block.balance_changes.clear();
            for_each_call(block, |c| c.balance_changes.clear())
        }
        "nonce_changes" => for_each_call(block, |c| c.nonce_changes.clear()),
        "code_changes" => {
            block.code_changes.clear();
            for_each_call(block, |c| c.code_changes.clear())
        }
        #[allow(deprecated)]
        "account_creations" => for_each_call(block, |c| c.account_creations.clear()),
        "logs" => {
            for tx in block.transaction_traces.iter_mut() {
                if let Some(receipt) = tx.receipt.as_mut() {
                    receipt.logs.clear();
                }
            }
            for_each_call(block, |c| c.logs.clear())
        }
        "calls" => {
            for tx in block.transaction_traces.iter_mut() {
                tx.calls.clear();
            }
            block.system_calls.clear();
        }
        other => return Err(Error::msg(format!("unknown family {other:?}, expected one of: calls, {}", FAMILIES.join(", ")))),
    }
    Ok(())
}

fn for_each_call(block: &mut Block, f: impl Fn(&mut Call)) {
    for tx in block.transaction_traces.iter_mut() {
        for call in tx.calls.iter_mut() {
            f(call);
        }
    }
    for call in block.system_calls.iter_mut() {
        f(call);
    }
}
