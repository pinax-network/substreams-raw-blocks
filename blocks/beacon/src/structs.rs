use prost_types::Timestamp;

pub struct BlockTimestamp {
    pub time: Timestamp,
    pub date: String,
    pub number: u64,
    pub hash: String,
}
