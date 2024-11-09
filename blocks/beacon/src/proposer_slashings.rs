use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::proposer_slashing_keys, pb::sf::beacon::r#type::v1::ProposerSlashing};

pub fn insert_proposer_slashings(tables: &mut DatabaseChanges, clock: &Clock, proposer_slashings: &[ProposerSlashing]) {
    for (index, proposer_slashing) in proposer_slashings.iter().enumerate() {
        // Signed header 1
        let signed_header_1 = proposer_slashing.signed_header_1.as_ref().unwrap();
        let signed_header_1_message = signed_header_1.message.as_ref().unwrap();
        let signed_header_1_slot = signed_header_1_message.slot;
        let signed_header_1_proposer_index = signed_header_1_message.proposer_index;
        let signed_header_1_parent_root = bytes_to_hex(&signed_header_1_message.parent_root);
        let signed_header_1_state_root = bytes_to_hex(&signed_header_1_message.state_root);
        let signed_header_1_body_root = bytes_to_hex(&signed_header_1_message.body_root);
        let signed_header_1_signature = bytes_to_hex(&signed_header_1.signature);
        // Signed header 2
        let signed_header_2 = proposer_slashing.signed_header_2.as_ref().unwrap();
        let signed_header_2_message = signed_header_2.message.as_ref().unwrap();
        let signed_header_2_slot = signed_header_2_message.slot;
        let signed_header_2_proposer_index = signed_header_2_message.proposer_index;
        let signed_header_2_parent_root = bytes_to_hex(&signed_header_2_message.parent_root);
        let signed_header_2_state_root = bytes_to_hex(&signed_header_2_message.state_root);
        let signed_header_2_body_root = bytes_to_hex(&signed_header_2_message.body_root);
        let signed_header_2_signature = bytes_to_hex(&signed_header_2.signature);

        let keys = proposer_slashing_keys(&clock.id, index as u64);

        let row = tables
            .push_change_composite("proposer_slashings", keys, 0, table_change::Operation::Create)
            .change("signed_header_1_slot", ("", signed_header_1_slot.to_string().as_str()))
            .change("signed_header_1_proposer_index", ("", signed_header_1_proposer_index.to_string().as_str()))
            .change("signed_header_1_parent_root", ("", signed_header_1_parent_root.as_str()))
            .change("signed_header_1_state_root", ("", signed_header_1_state_root.as_str()))
            .change("signed_header_1_body_root", ("", signed_header_1_body_root.as_str()))
            .change("signed_header_1_signature", ("", signed_header_1_signature.as_str()))
            .change("signed_header_2_slot", ("", signed_header_2_slot.to_string().as_str()))
            .change("signed_header_2_proposer_index", ("", signed_header_2_proposer_index.to_string().as_str()))
            .change("signed_header_2_parent_root", ("", signed_header_2_parent_root.as_str()))
            .change("signed_header_2_state_root", ("", signed_header_2_state_root.as_str()))
            .change("signed_header_2_body_root", ("", signed_header_2_body_root.as_str()))
            .change("signed_header_2_signature", ("", signed_header_2_signature.as_str()));

        insert_timestamp(row, clock, false, true);
    }
}
