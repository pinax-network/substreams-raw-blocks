use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::withdrawals_keys, pb::sf::beacon::r#type::v1::Withdrawal};

pub fn insert_withdrawals(tables: &mut DatabaseChanges, clock: &Clock, withdrawals: &Vec<Withdrawal>) {
    for withdrawal in withdrawals {
        let index = withdrawal.withdrawal_index;
        let validator_index = withdrawal.validator_index;
        let address = bytes_to_hex(&withdrawal.address);
        let gwei = withdrawal.gwei;

        let keys = withdrawals_keys(&clock.id, index);

        let row = tables
            .push_change_composite("withdrawals", keys, 0, table_change::Operation::Create)
            .change("validator_index", ("", validator_index.to_string().as_str()))
            .change("address", ("", address.as_str()))
            .change("gwei", ("", gwei.to_string().as_str()));

        insert_timestamp(row, clock, false, true);
    }
}
