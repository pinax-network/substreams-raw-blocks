use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::bls_to_execution_change_keys, pb::sf::beacon::r#type::v1::SignedBlsToExecutionChange};

pub fn insert_bls_to_execution_changes(tables: &mut DatabaseChanges, clock: &Clock, bls_to_execution_changes: &[SignedBlsToExecutionChange]) {
    for (index, bls_to_execution_change) in bls_to_execution_changes.iter().enumerate() {
        let change = &bls_to_execution_change.message.as_ref().unwrap();
        let validator_index = change.validator_index;
        let from_bls_pubkey = bytes_to_hex(&change.from_bls_pub_key);
        let to_execution_address = bytes_to_hex(&change.to_execution_address);
        let signature = bytes_to_hex(&bls_to_execution_change.signature);

        let keys = bls_to_execution_change_keys(&clock.id, index as u64);

        let row = tables
            .push_change_composite("bls_to_execution_changes", keys, 0, table_change::Operation::Create)
            .change("validator_index", ("", validator_index.to_string().as_str()))
            .change("from_bls_pubkey", ("", from_bls_pubkey.as_str()))
            .change("to_execution_address", ("", to_execution_address.as_str()))
            .change("signature", ("", signature.as_str()));

        insert_timestamp(row, clock, true, false);
    }
}
