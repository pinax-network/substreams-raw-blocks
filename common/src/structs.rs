use prost_types::Timestamp;

// Struct to collect block timestamp, date, number and hash
// Used in parquet sinks to pass to every table
pub struct BlockTimestamp {
    pub time: Timestamp,
    pub date: String,
    pub number: u64,
    pub hash: String,
}
