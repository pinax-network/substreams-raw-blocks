use common::{
    blocks::insert_timestamp,
    utils::{bytes_to_hex, hex_array_to_string},
};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{
    keys::deposit_keys,
    pb::{beacon::rawblocks::Deposit as RawDeposit, sf::beacon::r#type::v1::Deposit},
    structs::BlockTimestamp,
    utils::encode_hex_2d_array,
};

pub fn collect_deposits(deposits: &Vec<Deposit>, timestamp: &BlockTimestamp) -> Vec<RawDeposit> {
    let mut deposits_vec = Vec::<RawDeposit>::new();

    for (index, d) in deposits.iter().enumerate() {
        deposits_vec.push(RawDeposit {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: timestamp.hash.clone(),
            index: index as u64,
            proof: encode_hex_2d_array(&d.proof),
            pubkey: bytes_to_hex(&d.data.as_ref().unwrap().public_key),
            withdrawal_credentials: bytes_to_hex(&d.data.as_ref().unwrap().withdrawal_credentials),
            signature: bytes_to_hex(&d.data.as_ref().unwrap().signature),
            gwei: d.data.as_ref().unwrap().gwei,
        });
    }

    deposits_vec
}
