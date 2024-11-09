use common::{blocks::insert_timestamp, utils::hex_array_to_string};
use substreams::{pb::substreams::Clock, Hex};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{keys::blob_keys, pb::sf::beacon::r#type::v1::Blob};

pub fn insert_blobs(tables: &mut DatabaseChanges, clock: &Clock, blobs: &Vec<Blob>) {
    for b in blobs {
        let index = b.index;
        let blob = Hex::encode(&b.blob);
        let kzg_commitment = Hex::encode(&b.kzg_commitment);
        let kzg_proof = Hex::encode(&b.kzg_proof);
        let kzg_commitment_inclusion_proof = hex_array_to_string(&b.kzg_commitment_inclusion_proof);

        let keys = blob_keys(&clock.id, index);

        let row = tables
            .push_change_composite("blobs", keys, 0, table_change::Operation::Create)
            .change("blob", ("", blob.as_str()))
            .change("kzg_commitment", ("", kzg_commitment.as_str()))
            .change("kzg_proof", ("", kzg_proof.as_str()))
            .change("kzg_commitment_inclusion_proof", ("", kzg_commitment_inclusion_proof.as_str()));

        insert_timestamp(row, clock, false, true);
    }
}
