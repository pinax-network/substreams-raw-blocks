use common::utils::bytes_to_hex;

use crate::{
    pb::{pinax::beacon::v1::Withdrawal as RawWithdrawal, sf::beacon::r#type::v1::Withdrawal},
    structs::BlockTimestamp,
};

pub fn collect_withdrawals(withdrawals: &Vec<Withdrawal>, timestamp: &BlockTimestamp) -> Vec<RawWithdrawal> {
    let mut vec = Vec::<RawWithdrawal>::new();

    for (index, w) in withdrawals.iter().enumerate() {
        vec.push(RawWithdrawal {
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

    vec
}
