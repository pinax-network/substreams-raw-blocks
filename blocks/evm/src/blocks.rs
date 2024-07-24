use common::{block_time_to_date, bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;

pub fn insert_blocks(tables: &mut Tables, clock: &Clock, block: &Block) {
    let header = block.clone().header.unwrap();
    let timestamp = clock.clone().timestamp.unwrap();
    let block_time = timestamp.to_string();
    let block_number = clock.number.to_string();
    let block_hash = format!("0x{}", clock.id);
    let block_date = block_time_to_date(block_time.as_str());

    // blocks
    let row = tables
        .create_row("blocks", &block_hash)
        .set("time", &block_time)
        .set_bigint("number", &block_number)
        .set("date", &block_date)
        .set("hash", &block_hash)
        .set("parent_hash", bytes_to_hex(header.parent_hash))
        .set_bigint("nonce", &header.nonce.to_string())
        .set("ommers_hash", bytes_to_hex(header.uncle_hash))
        .set("logs_bloom", bytes_to_hex(header.logs_bloom))
        .set("transactions_root", bytes_to_hex(header.transactions_root))
        .set("state_root", bytes_to_hex(header.state_root))
        .set("receipts_root", bytes_to_hex(header.receipt_root))
        .set("miner", bytes_to_hex(header.coinbase))
        .set_bigint("size", &block.size.to_string())
        .set("mix_hash", bytes_to_hex(header.mix_hash))
        .set("extra_data", bytes_to_hex(header.extra_data))
        .set_bigint("gas_limit", &header.gas_limit.to_string())
        .set_bigint("gas_used", &header.gas_used.to_string());

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
    row.set_bigint("total_transactions", &total_transactions.to_string())
        .set_bigint("successful_transactions", &successful_transactions.to_string())
        .set_bigint("failed_transactions", &failed_transactions.to_string());

    // optional fields
    match header.difficulty {
        Some(difficulty) => row.set_bigint("difficulty", &difficulty.with_decimal(0).to_string()),
        None => row,
    };
    match header.total_difficulty {
        Some(total_difficulty) => row.set_bigint("total_difficulty", &total_difficulty.with_decimal(0).to_string()),
        None => row,
    };
    match header.blob_gas_used {
        Some(blob_gas_used) => row.set_bigint("blob_gas_used", &blob_gas_used.to_string()),
        None => row,
    };
    match header.base_fee_per_gas {
        Some(base_fee_per_gas) => row.set_bigint("base_fee_per_gas", &base_fee_per_gas.with_decimal(0).to_string()),
        None => row,
    };
    match header.parent_beacon_root.len() {
        0 => row,
        _ => row.set("parent_beacon_root", bytes_to_hex(header.parent_beacon_root)),
    };
}