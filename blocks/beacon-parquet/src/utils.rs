use common::utils::{block_time_to_date, bytes_to_hex};
use substreams::pb::substreams::Clock;

use crate::structs::BlockTimestamp;

pub fn build_timestamp(clock: &Clock) -> BlockTimestamp {
    let timestamp = clock.timestamp.unwrap();
    let block_date = block_time_to_date(timestamp.to_string().as_str());

    BlockTimestamp {
        time: timestamp,
        date: block_date,
        hash: clock.id.clone(),
        number: clock.number,
    }
}

pub fn encode_hex_2d_array(hex_array: &Vec<Vec<u8>>) -> Vec<String> {
    hex_array.iter().map(|bytes| bytes_to_hex(bytes)).collect()
}
