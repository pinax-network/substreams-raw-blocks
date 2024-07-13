use common::{block_time_to_date, bytes_to_hex};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let header = block.header.unwrap();
    let timestamp = clock.timestamp.unwrap();
    let block_time = timestamp.to_string();
    let block_number = clock.number.to_string();
    let block_hash = format!("0x{}", clock.id);
    let block_date = block_time_to_date(block_time.as_str());

    tables
        .create_row("blocks", &block_hash)
        .set("time", &block_time)
        .set_bigint("number", &block_number)
        .set("date", block_date)
        .set("hash", block_hash)
        .set("parent_hash", bytes_to_hex(header.parent_hash))
        .set_bigint("nonce", &header.nonce.to_string())
        .set("ommers_hash", bytes_to_hex(header.uncle_hash))
        .set("logs_bloom", bytes_to_hex(header.logs_bloom))
        .set("transactions_root", bytes_to_hex(header.transactions_root))
        .set("state_root", bytes_to_hex(header.state_root))
        .set("receipts_root", bytes_to_hex(header.receipt_root))
        .set("miner", bytes_to_hex(header.coinbase))
        .set_bigint("difficulty", &header.difficulty.unwrap_or_default().with_decimal(0).to_string())
        .set_bigint("total_difficulty", &header.total_difficulty.unwrap_or_default().with_decimal(0).to_string())
        .set_bigint("size", &block.size.to_string())
        .set("mix_hash", bytes_to_hex(header.mix_hash))
        .set("extra_data", bytes_to_hex(header.extra_data))
        .set_bigint("gas_limit", &header.gas_limit.to_string())
        .set_bigint("gas_used", &header.gas_used.to_string())
        .set_bigint("blob_gas_used", &header.blob_gas_used.unwrap_or_default().to_string())
        .set_bigint("transaction_count", &block.transaction_traces.len().to_string())
        .set_bigint("base_fee_per_gas", &header.base_fee_per_gas.unwrap_or_default().with_decimal(0).to_string())
        .set("parent_beacon_root", bytes_to_hex(header.parent_beacon_root));

        // block_time
        // block_number
        // block_hash
        // contract_address
        // topic0
        // topic1
        // topic2
        // topic3
        // data
        // tx_hash
        // index
        // tx_index
        // block_date
        // tx_from
        // tx_to

    Ok(tables.to_entity_changes())
}
