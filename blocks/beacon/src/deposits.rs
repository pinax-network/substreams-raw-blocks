use common::{
    blocks::insert_timestamp,
    utils::{bytes_to_hex, hex_array_to_string},
};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{keys::deposit_keys, pb::sf::beacon::r#type::v1::Deposit};

pub fn insert_deposits(tables: &mut DatabaseChanges, clock: &Clock, deposits: &Vec<Deposit>) {
    for (index, deposit) in deposits.iter().enumerate() {
        let proof = hex_array_to_string(&deposit.proof);
        let deposit_data = deposit.data.as_ref().unwrap();
        let pubkey = bytes_to_hex(&deposit_data.public_key);
        let withdrawal_credentials = bytes_to_hex(&deposit_data.withdrawal_credentials);
        let signature = bytes_to_hex(&deposit_data.signature);
        let gwei = deposit_data.gwei;

        let keys = deposit_keys(&clock.id, index as u64);

        let row = tables
            .push_change_composite("deposits", keys, 0, substreams_database_change::pb::database::table_change::Operation::Create)
            .change("proof", ("", proof.as_str()))
            .change("pubkey", ("", pubkey.as_str()))
            .change("withdrawal_credentials", ("", withdrawal_credentials.as_str()))
            .change("signature", ("", signature.as_str()))
            .change("gwei", ("", gwei.to_string().as_str()));

        insert_timestamp(row, clock, false, true);
    }
}
