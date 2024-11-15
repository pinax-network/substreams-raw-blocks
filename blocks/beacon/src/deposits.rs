use common::utils::bytes_to_hex;

use crate::{
    pb::{beacon::Deposit as RawDeposit, sf::beacon::r#type::v1::Deposit},
    structs::BlockTimestamp,
    utils::{encode_2d_array_to_csv_string, encode_hex_2d_array},
};

pub fn collect_deposits(deposits: &Vec<Deposit>, timestamp: &BlockTimestamp) -> Vec<RawDeposit> {
    let mut vec = Vec::<RawDeposit>::new();

    for (index, d) in deposits.iter().enumerate() {
        vec.push(RawDeposit {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u64,
            // TODO: use encode_hex_2d_array once Array(Text) is supported
            proof: encode_2d_array_to_csv_string(&d.proof),
            pubkey: bytes_to_hex(&d.data.as_ref().unwrap().public_key),
            withdrawal_credentials: bytes_to_hex(&d.data.as_ref().unwrap().withdrawal_credentials),
            signature: bytes_to_hex(&d.data.as_ref().unwrap().signature),
            gwei: d.data.as_ref().unwrap().gwei,
        });
    }

    vec
}
