use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

use crate::{
    attestations::insert_attestations,
    deposits::insert_deposits,
    pb::sf::beacon::r#type::v1::{block::Body::*, AltairBody, BellatrixBody, Block as BeaconBlock, CapellaBody, DenebBody, Phase0Body},
    withdrawals::insert_withdrawals,
};

use crate::{blobs::insert_blobs, blocks::insert_blocks};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: BeaconBlock) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();
    let spec = spec_to_string(block.spec);

    insert_blocks(&mut tables, &block, &spec, &clock);

    if let Some(body) = block.body {
        match (spec.as_str(), body) {
            ("Deneb", Deneb(body)) => insert_deneb_body(&mut tables, &clock, &body),
            ("Capella", Capella(body)) => insert_capella_body(&mut tables, &clock, &body),
            ("Bellatrix", Bellatrix(body)) => insert_bellatrix_body(&mut tables, &clock, &body),
            ("Altair", Altair(body)) => insert_altair_body(&mut tables, &clock, &body),
            ("Phase0", Phase0(body)) => insert_phase0_body(&mut tables, &clock, &body),
            _ => {}
        }
    }

    Ok(tables)
}

fn spec_to_string(spec: i32) -> String {
    match spec {
        0 => "Unspecified".to_string(),
        1 => "Phase0".to_string(),
        2 => "Altair".to_string(),
        3 => "Bellatrix".to_string(),
        4 => "Capella".to_string(),
        5 => "Deneb".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn insert_deneb_body(tables: &mut DatabaseChanges, clock: &Clock, body: &DenebBody) {
    // Table::blobs
    insert_blobs(tables, &clock, &body.embedded_blobs);
    insert_deposits(tables, clock, &body.deposits);
    let withdrawals = &body.execution_payload.as_ref().unwrap().withdrawals;
    insert_withdrawals(tables, clock, withdrawals);
    insert_attestations(tables, clock, &body.attestations);
}

fn insert_capella_body(tables: &mut DatabaseChanges, clock: &Clock, body: &CapellaBody) {
    insert_deposits(tables, clock, &body.deposits);
    let withdrawals = &body.execution_payload.as_ref().unwrap().withdrawals;
    insert_withdrawals(tables, clock, withdrawals);
    insert_attestations(tables, clock, &body.attestations);
}

fn insert_bellatrix_body(tables: &mut DatabaseChanges, clock: &Clock, body: &BellatrixBody) {
    insert_deposits(tables, clock, &body.deposits);
    insert_attestations(tables, clock, &body.attestations);
}

fn insert_altair_body(tables: &mut DatabaseChanges, clock: &Clock, body: &AltairBody) {
    insert_deposits(tables, clock, &body.deposits);
    insert_attestations(tables, clock, &body.attestations);
}

fn insert_phase0_body(tables: &mut DatabaseChanges, clock: &Clock, body: &Phase0Body) {
    insert_deposits(tables, clock, &body.deposits);
    insert_attestations(tables, clock, &body.attestations);
}
