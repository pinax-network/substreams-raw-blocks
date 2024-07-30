use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::TableChange;

use crate::utils::block_time_to_date;

pub fn insert_timestamp(row: &mut TableChange, clock: &Clock, is_block: bool) {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(timestamp.to_string().as_str());
    let block_time = timestamp.seconds.to_string(); // TODO: format to DateTime64(3, 'UTC')
    let block_number = clock.number.to_string();
    let block_hash = format!("0x{}", clock.id);
    let prefix = if is_block { "" } else { "block_" };

    row.change(format!("{}date", prefix).as_str(), ("", block_date.as_str()))
        .change(format!("{}time", prefix).as_str(), ("", block_time.as_str()))
        .change(format!("{}number", prefix).as_str(), ("", block_number.as_str()))
        .change(format!("{}hash", prefix).as_str(), ("", block_hash.as_str()));
}
