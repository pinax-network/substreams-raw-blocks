use common::utils::block_time_to_date;
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
