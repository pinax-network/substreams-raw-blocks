use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::voluntary_exit_keys, pb::sf::beacon::r#type::v1::SignedVoluntaryExit};

pub fn insert_voluntary_exits(tables: &mut DatabaseChanges, clock: &Clock, voluntary_exits: &[SignedVoluntaryExit]) {
    for (index, voluntary_exit) in voluntary_exits.iter().enumerate() {
        let message = voluntary_exit.message.as_ref().unwrap();
        let epoch = message.epoch;
        let validator_index = message.validator_index;
        let signature = bytes_to_hex(&voluntary_exit.signature);

        let keys = voluntary_exit_keys(&clock.id, index as u64);

        let row = tables
            .push_change_composite("voluntary_exits", keys, 0, table_change::Operation::Create)
            .change("epoch", ("", epoch.to_string().as_str()))
            .change("validator_index", ("", validator_index.to_string().as_str()))
            .change("signature", ("", signature.as_str()));

        insert_timestamp(row, clock, false, true);
    }
}
