use common::utils::block_time_to_date;
use substreams::pb::substreams::Clock;
use substreams_solana::{b58, base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

use crate::structs::BlockTimestamp;

pub static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

pub fn get_timestamp_without_number(clock: &Clock) -> BlockTimestamp {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(timestamp.to_string().as_str());

    BlockTimestamp {
        time: clock.timestamp.expect("timestamp is required"),
        date: block_date,
        hash: clock.id.to_string(),
    }
}

pub fn build_csv_string<T: ToString>(values: &[T]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(",")
}

// Get all encoded account keys including loaded writable and readonly addresses
pub fn get_account_keys_extended(transaction: &ConfirmedTransaction) -> Vec<String> {
    let message = transaction.transaction.as_ref().unwrap().message.as_ref().unwrap();
    let meta = transaction.meta.as_ref().unwrap();

    message
        .account_keys
        .iter()
        .chain(&meta.loaded_writable_addresses)
        .chain(&meta.loaded_readonly_addresses)
        .map(base58::encode)
        .collect()
}
