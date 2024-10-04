use common::utils::{add_prefix_to_hex, block_time_to_date};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::TableChange;

pub fn insert_timestamp_without_number(row: &mut TableChange, clock: &Clock, is_block: bool, with_prefix: bool) {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(timestamp.to_string().as_str());
    let seconds = timestamp.seconds;
    let nanos = timestamp.nanos;
    let milliseconds = seconds * 1000 + i64::from(nanos) / 1_000_000;
    let block_time = milliseconds.to_string();
    let block_hash = if with_prefix { add_prefix_to_hex(&clock.id) } else { clock.id.to_string() };
    let prefix = if is_block { "" } else { "block_" };

    row.change(format!("{}date", prefix).as_str(), ("", block_date.as_str()))
        .change(format!("{}time", prefix).as_str(), ("", block_time.as_str()))
        .change(format!("{}hash", prefix).as_str(), ("", block_hash.as_str()));
}
