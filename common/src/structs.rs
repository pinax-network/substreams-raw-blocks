use prost_types::Timestamp;

use crate::utils::block_time_to_date;

// Struct to collect block timestamp, date, number and hash
// Used in parquet sinks to pass to every table
pub struct BlockTimestamp {
    pub time: Timestamp,
    pub date: String,
    pub number: u64,
    pub hash: String,
}

// Canonical identity columns carried by every table, aligned with
// pinax-network/firehose-parquet (`block_num, block_id, parent_num, parent_id, timestamp, date`).
// `lib_num` is not available inside a Substreams module and is therefore omitted.
#[derive(Clone, Debug, Default)]
pub struct BlockIdentity {
    pub block_num: u64,
    pub block_id: Vec<u8>,
    pub parent_num: u64,
    pub parent_id: Vec<u8>,
    pub timestamp: Timestamp,
    pub date: String,
}

impl BlockIdentity {
    pub fn new(block_num: u64, block_id: Vec<u8>, parent_num: u64, parent_id: Vec<u8>, timestamp: &Option<Timestamp>) -> Self {
        let timestamp = timestamp.clone().expect("clock timestamp is always set");
        let date = block_time_to_date(&timestamp.to_string());
        BlockIdentity {
            block_num,
            block_id,
            parent_num,
            parent_id,
            timestamp,
            date,
        }
    }
}
