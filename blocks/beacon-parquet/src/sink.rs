use substreams::{errors::Error, pb::substreams::Clock};

use crate::{
    attestations::insert_attestations,
    attester_slashings::insert_attester_slashings,
    blocks::collect_blocks,
    bls_to_execution_changes::insert_bls_to_execution_changes,
    deposits::insert_deposits,
    pb::{
        beacon::rawblocks::Events,
        sf::beacon::r#type::v1::{AltairBody, BellatrixBody, Block as BeaconBlock, CapellaBody, DenebBody, Phase0Body},
    },
    proposer_slashings::insert_proposer_slashings,
    utils::build_timestamp,
    voluntary_exits::insert_voluntary_exits,
    withdrawals::insert_withdrawals,
};

use crate::blobs::insert_blobs;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: BeaconBlock) -> Result<Events, Error> {
    let spec = spec_to_string(block.spec);

    // insert_blocks(&mut tables, &block, &spec, &clock);

    let body = block.body.as_ref().unwrap();
    let timestamp = build_timestamp(&clock);

    Ok(Events {
        blocks: collect_blocks(&block, &spec, &timestamp),
        blobs: vec![],
        deposits: vec![],
        withdrawals: vec![],
        attestations: vec![],
        attester_slashings: vec![],
        bls_to_execution_changes: vec![],
        proposer_slashings: vec![],
        voluntary_exits: vec![],
    })

    // match (spec.as_str(), body) {
    //     ("Deneb", Deneb(body)) => insert_deneb_body(&mut tables, &clock, &body),
    //     ("Capella", Capella(body)) => insert_capella_body(&mut tables, &clock, &body),
    //     ("Bellatrix", Bellatrix(body)) => insert_bellatrix_body(&mut tables, &clock, &body),
    //     ("Altair", Altair(body)) => insert_altair_body(&mut tables, &clock, &body),
    //     ("Phase0", Phase0(body)) => insert_phase0_body(&mut tables, &clock, &body),
    //     _ => {}
    // }
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

// fn insert_deneb_body(tables: &mut DatabaseChanges, clock: &Clock, body: &DenebBody) {
//     insert_blobs(tables, &clock, &body.embedded_blobs);
//     insert_deposits(tables, clock, &body.deposits);
//     let withdrawals = &body.execution_payload.as_ref().unwrap().withdrawals;
//     insert_withdrawals(tables, clock, withdrawals);
//     insert_attestations(tables, clock, &body.attestations);
//     insert_attester_slashings(tables, clock, &body.attester_slashings);
//     insert_bls_to_execution_changes(tables, clock, &body.bls_to_execution_changes);
//     insert_proposer_slashings(tables, clock, &body.proposer_slashings);
//     insert_voluntary_exits(tables, clock, &body.voluntary_exits);
// }

pub fn collect_deneb_body(clock: &Clock, body: &DenebBody) {}

pub fn collect_capella_body(clock: &Clock, body: &CapellaBody) {}

pub fn collect_bellatrix_body(clock: &Clock, body: &BellatrixBody) {}

pub fn collect_altair_body(clock: &Clock, body: &AltairBody) {}

pub fn collect_phase0_body(clock: &Clock, body: &Phase0Body) {}

// fn insert_capella_body(tables: &mut DatabaseChanges, clock: &Clock, body: &CapellaBody) {
//     insert_deposits(tables, clock, &body.deposits);
//     let withdrawals = &body.execution_payload.as_ref().unwrap().withdrawals;
//     insert_withdrawals(tables, clock, withdrawals);
//     insert_attestations(tables, clock, &body.attestations);
//     insert_attester_slashings(tables, clock, &body.attester_slashings);
//     insert_proposer_slashings(tables, clock, &body.proposer_slashings);
//     insert_voluntary_exits(tables, clock, &body.voluntary_exits);
// }

// fn insert_bellatrix_body(tables: &mut DatabaseChanges, clock: &Clock, body: &BellatrixBody) {
//     insert_deposits(tables, clock, &body.deposits);
//     insert_attestations(tables, clock, &body.attestations);
//     insert_attester_slashings(tables, clock, &body.attester_slashings);
//     insert_proposer_slashings(tables, clock, &body.proposer_slashings);
//     insert_voluntary_exits(tables, clock, &body.voluntary_exits);
// }

// fn insert_altair_body(tables: &mut DatabaseChanges, clock: &Clock, body: &AltairBody) {
//     insert_deposits(tables, clock, &body.deposits);
//     insert_attestations(tables, clock, &body.attestations);
//     insert_attester_slashings(tables, clock, &body.attester_slashings);
//     insert_proposer_slashings(tables, clock, &body.proposer_slashings);
//     insert_voluntary_exits(tables, clock, &body.voluntary_exits);
// }

// fn insert_phase0_body(tables: &mut DatabaseChanges, clock: &Clock, body: &Phase0Body) {
//     insert_deposits(tables, clock, &body.deposits);
//     insert_attestations(tables, clock, &body.attestations);
//     insert_attester_slashings(tables, clock, &body.attester_slashings);
//     insert_proposer_slashings(tables, clock, &body.proposer_slashings);
//     insert_voluntary_exits(tables, clock, &body.voluntary_exits);
// }