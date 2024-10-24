use common::blocks::insert_timestamp;
use substreams::{pb::substreams::Clock, Hex};
use substreams_cosmos::{pb::public_key, Block};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::validator_update_keys;

pub fn insert_validator_updates(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    for (index, validator_update) in block.validator_updates.iter().enumerate() {
        let index_str = index.to_string();
        let public_key = match validator_update.pub_key.as_ref().unwrap().sum.as_ref().unwrap() {
            public_key::Sum::Ed25519(bytes) => bytes,
            public_key::Sum::Secp256k1(bytes) => bytes,
        };
        let public_key_str = Hex::encode(public_key);
        let power = validator_update.power;

        let keys = validator_update_keys(&clock.number.to_string(), &index_str);

        let row = tables
            .push_change_composite("validator_updates", keys, 0, table_change::Operation::Create)
            .change("public_key", ("", public_key_str.as_str()))
            .change("power", ("", power.to_string().as_str()));

        insert_timestamp(row, clock, false, true);
    }
}
