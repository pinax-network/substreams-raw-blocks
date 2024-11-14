use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{
    keys::withdrawals_keys,
    pb::{beacon::rawblocks::Withdrawal as RawWithdrawal, sf::beacon::r#type::v1::Withdrawal},
    structs::BlockTimestamp,
};

pub fn collect_withdrawals(withdrawals: &Vec<Withdrawal>, timestamp: &BlockTimestamp) -> Vec<RawWithdrawal> {
    let mut withdrawals_vec = Vec::<RawWithdrawal>::new();

    for (index, w) in withdrawals.iter().enumerate() {
        withdrawals_vec.push(RawWithdrawal {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            withdrawal_index: index as u64,
            validator_index: w.validator_index,
            address: bytes_to_hex(&w.address),
            gwei: w.gwei,
        });
    }

    withdrawals_vec
}
