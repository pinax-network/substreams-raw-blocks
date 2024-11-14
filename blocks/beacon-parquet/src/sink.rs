use crate::{
    attestations::collect_attestations, blobs::collect_blobs, deposits::collect_deposits, pb::sf::beacon::r#type::v1::block::Body::*, structs::BlockTimestamp, withdrawals::collect_withdrawals,
};
use substreams::{errors::Error, pb::substreams::Clock};

use crate::{
    blocks::collect_blocks,
    pb::{
        beacon::rawblocks::Events,
        sf::beacon::r#type::v1::{AltairBody, BellatrixBody, Block as BeaconBlock, CapellaBody, DenebBody, Phase0Body},
    },
    utils::build_timestamp,
};

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: BeaconBlock) -> Result<Events, Error> {
    let spec = spec_to_string(block.spec);

    let body = block.body.as_ref().unwrap();
    let timestamp = build_timestamp(&clock);

    match (spec.as_str(), body) {
        ("Deneb", Deneb(body)) => Ok(output_deneb_body(&block, &spec, body, &timestamp)),
        ("Capella", Capella(body)) => Ok(output_capella_body(&block, &spec, body, &timestamp)),
        ("Bellatrix", Bellatrix(body)) => Ok(output_bellatrix_body(&block, &spec, body, &timestamp)),
        ("Altair", Altair(body)) => Ok(output_altair_body(&block, &spec, body, &timestamp)),
        ("Phase0", Phase0(body)) => Ok(output_phase0_body(&block, &spec, body, &timestamp)),
        _ => Ok(Events::default()),
    }
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

pub fn output_deneb_body(block: &BeaconBlock, spec: &str, body: &DenebBody, timestamp: &BlockTimestamp) -> Events {
    Events {
        blocks: collect_blocks(&block, &spec, &timestamp),
        blobs: collect_blobs(&body.embedded_blobs, &timestamp),
        deposits: collect_deposits(&body.deposits, &timestamp),
        withdrawals: collect_withdrawals(&body.execution_payload.as_ref().unwrap().withdrawals, &timestamp),
        attestations: collect_attestations(&body.attestations, &timestamp),
        attester_slashings: vec![],
        bls_to_execution_changes: vec![],
        proposer_slashings: vec![],
        voluntary_exits: vec![],
    }
}

pub fn output_capella_body(block: &BeaconBlock, spec: &str, body: &CapellaBody, timestamp: &BlockTimestamp) -> Events {
    Events {
        blocks: collect_blocks(&block, &spec, &timestamp),
        blobs: vec![],
        deposits: collect_deposits(&body.deposits, &timestamp),
        withdrawals: collect_withdrawals(&body.execution_payload.as_ref().unwrap().withdrawals, &timestamp),
        attestations: collect_attestations(&body.attestations, &timestamp),
        attester_slashings: vec![],
        bls_to_execution_changes: vec![],
        proposer_slashings: vec![],
        voluntary_exits: vec![],
    }
}

pub fn output_bellatrix_body(block: &BeaconBlock, spec: &str, body: &BellatrixBody, timestamp: &BlockTimestamp) -> Events {
    Events {
        blocks: collect_blocks(&block, &spec, &timestamp),
        blobs: vec![],
        deposits: collect_deposits(&body.deposits, &timestamp),
        withdrawals: vec![],
        attestations: collect_attestations(&body.attestations, &timestamp),
        attester_slashings: vec![],
        bls_to_execution_changes: vec![],
        proposer_slashings: vec![],
        voluntary_exits: vec![],
    }
}

pub fn output_altair_body(block: &BeaconBlock, spec: &str, body: &AltairBody, timestamp: &BlockTimestamp) -> Events {
    Events {
        blocks: collect_blocks(&block, &spec, &timestamp),
        blobs: vec![],
        deposits: collect_deposits(&body.deposits, &timestamp),
        withdrawals: vec![],
        attestations: collect_attestations(&body.attestations, &timestamp),
        attester_slashings: vec![],
        bls_to_execution_changes: vec![],
        proposer_slashings: vec![],
        voluntary_exits: vec![],
    }
}

pub fn output_phase0_body(block: &BeaconBlock, spec: &str, body: &Phase0Body, timestamp: &BlockTimestamp) -> Events {
    Events {
        blocks: collect_blocks(&block, &spec, &timestamp),
        blobs: vec![],
        deposits: collect_deposits(&body.deposits, &timestamp),
        withdrawals: vec![],
        attestations: collect_attestations(&body.attestations, &timestamp),
        attester_slashings: vec![],
        bls_to_execution_changes: vec![],
        proposer_slashings: vec![],
        voluntary_exits: vec![],
    }
}

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
