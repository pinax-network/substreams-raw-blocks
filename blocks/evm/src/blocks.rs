use common::keys::block_keys;
use common::utils::bytes_to_hex;
use common::sinks::insert_timestamp;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::Block;

// https://github.com/streamingfast/firehose-ethereum/blob/develop/proto/sf/ethereum/type/v2/type.proto
pub fn insert_blocks(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    let header = block.clone().header.unwrap();
    let parent_hash = bytes_to_hex(header.parent_hash);
    let nonce = header.nonce.to_string();
    let ommers_hash = bytes_to_hex(header.uncle_hash);
    let logs_bloom = bytes_to_hex(header.logs_bloom);
    let transactions_root = bytes_to_hex(header.transactions_root);
    let state_root = bytes_to_hex(header.state_root);
    let receipts_root = bytes_to_hex(header.receipt_root);
    let miner = bytes_to_hex(header.coinbase);
    let size = block.size.to_string();
    let mix_hash = bytes_to_hex(header.mix_hash);
    let extra_data = bytes_to_hex(header.extra_data);
    let gas_limit = header.gas_limit.to_string();
    let gas_used = header.gas_used.to_string();

    // blocks
    let row = tables
        .push_change_composite("blocks", block_keys(&clock), 0, table_change::Operation::Create)
        .change("parent_hash", ("", parent_hash.as_str()))
        .change("nonce", ("", nonce.as_str()))
        .change("ommers_hash", ("", ommers_hash.as_str()))
        .change("logs_bloom", ("", logs_bloom.as_str()))
        .change("transactions_root", ("", transactions_root.as_str()))
        .change("state_root", ("", state_root.as_str()))
        .change("receipts_root", ("", receipts_root.as_str()))
        .change("miner", ("", miner.as_str()))
        .change("size", ("", size.as_str()))
        .change("mix_hash", ("", mix_hash.as_str()))
        .change("extra_data", ("", extra_data.as_str()))
        .change("gas_limit", ("", gas_limit.as_str()))
        .change("gas_used", ("", gas_used.as_str()));

    insert_timestamp(row, clock, true);

    let mut total_transactions = 0;
    let mut successful_transactions = 0;
    let mut failed_transactions = 0;
    for traces in block.transaction_traces.iter() {
        if traces.status == 1 {
            successful_transactions += 1;
        } else {
            failed_transactions += 1;
        }
        total_transactions += 1;
    }
    row.change("total_transactions", ("", total_transactions.to_string().as_str()))
        .change("successful_transactions", ("", successful_transactions.to_string().as_str()))
        .change("failed_transactions", ("", failed_transactions.to_string().as_str()));

    // optional fields
    match header.difficulty {
        Some(difficulty) => row.change("difficulty", ("", difficulty.with_decimal(0).to_string().as_str())),
        None => row.change("difficulty", ("", "0")), // Nullable
    };

    match header.total_difficulty {
        Some(total_difficulty) => row.change("total_difficulty", ("", total_difficulty.with_decimal(0).to_string().as_str())),
        None => row.change("total_difficulty", ("", "0")), // Nullable
    };

    match header.blob_gas_used {
        Some(blob_gas_used) => row.change("blob_gas_used", ("", blob_gas_used.to_string().as_str())),
        None => row.change("blob_gas_used", ("", "0")), // Nullable
    };
    match header.base_fee_per_gas {
        Some(base_fee_per_gas) => row.change("base_fee_per_gas", ("", base_fee_per_gas.with_decimal(0).to_string().as_str())),
        None => row.change("base_fee_per_gas", ("", "0")), // Nullable
    };
    match header.parent_beacon_root.len() {
        0 => row.change("parent_beacon_root", ("", "0")), // Nullable,
        _ => row.change("parent_beacon_root", ("", bytes_to_hex(header.parent_beacon_root).as_str())),
    };
}