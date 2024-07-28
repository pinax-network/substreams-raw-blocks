use common::{keys::blocks_keys, utils::optional_bigint_to_string};
use common::utils::{bytes_to_hex, optional_uint64_to_string};
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
    let keys = blocks_keys(&clock);
    let row = tables
        .push_change_composite("blocks", keys, 0, table_change::Operation::Create)
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
        .change("gas_used", ("", gas_used.as_str()))

        // optional fields
        .change("parent_beacon_root", ("", bytes_to_hex(header.parent_beacon_root).as_str()))
        .change("blob_gas_used", ("", optional_uint64_to_string(header.blob_gas_used).as_str()))
        .change("difficulty", ("", optional_bigint_to_string(header.difficulty).as_str()))
        .change("total_difficulty", ("", optional_bigint_to_string(header.total_difficulty).as_str()))
        .change("base_fee_per_gas", ("", optional_bigint_to_string(header.base_fee_per_gas).as_str()))
    ;

    insert_timestamp(row, clock, true);

    // transaction counts
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

}