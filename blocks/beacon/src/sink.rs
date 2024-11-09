use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{
    deposits::insert_deposits,
    pb::sf::beacon::r#type::v1::{block::Body::*, Block as BeaconBlock},
    withdrawals::insert_withdrawals,
};

use crate::{blobs::insert_blobs, blocks::insert_blocks};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: BeaconBlock) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();

    // Table::blocks
    insert_blocks(&mut tables, &block, &clock);

    match block.body.as_ref() {
        Some(Deneb(body)) => {
            // Table::blobs
            insert_blobs(&mut tables, &clock, &body.embedded_blobs);
            insert_deposits(&mut tables, &clock, &body.deposits);
            let withdrawals = &body.execution_payload.as_ref().unwrap().withdrawals;
            insert_withdrawals(&mut tables, &clock, withdrawals);
        }
        Some(Capella(body)) => {
            insert_deposits(&mut tables, &clock, &body.deposits);
            let withdrawals = &body.execution_payload.as_ref().unwrap().withdrawals;
            insert_withdrawals(&mut tables, &clock, withdrawals);
        }
        Some(Bellatrix(body)) => {
            // Handle Bellatrix body
            insert_deposits(&mut tables, &clock, &body.deposits);
        }
        Some(Altair(body)) => {
            // Handle Altair body
            insert_deposits(&mut tables, &clock, &body.deposits);
        }
        Some(Phase0(body)) => {
            // Handle Phase0 body
            insert_deposits(&mut tables, &clock, &body.deposits);
        }
        _ => {}
    }

    // Table::deposits

    Ok(tables)
}
