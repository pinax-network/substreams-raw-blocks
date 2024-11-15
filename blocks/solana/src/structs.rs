use prost_types::Timestamp;

#[derive(Default)]
pub struct BlockCounters {
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub total_vote_transactions: u64,
    pub total_non_vote_transactions: u64,
    pub successful_vote_transactions: u64,
    pub successful_non_vote_transactions: u64,
    pub failed_vote_transactions: u64,
    pub failed_non_vote_transactions: u64,
}

#[derive(Default)]
pub struct BlockInfo {
    pub slot: u64,
    pub height: u64,
    pub previous_block_hash: String,
    pub parent_slot: u64,
}

#[derive(Default)]
pub struct BlockTimestamp {
    pub time: Timestamp,
    pub date: String,
    pub hash: String,
}
