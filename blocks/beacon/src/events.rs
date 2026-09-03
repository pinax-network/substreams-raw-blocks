use common::structs::BlockIdentity;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;

use crate::body::body_view;
use crate::pb::pinax::beacon::v2::Events;
use crate::pb::sf::beacon::r#type::v1::Block;
use crate::tables::*;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let id = BlockIdentity::new(block.slot, block.root.clone(), block.parent_slot, block.parent_root.clone(), &clock.timestamp);
    let body = body_view(&block.body);
    let slot = block.slot;

    Ok(Events {
        blocks: vec![collect_block(&block, &body, &id)],
        attestations: collect_attestations(slot, &body, &id),
        deposits: collect_deposits(slot, &body, &id),
        proposer_slashings: collect_proposer_slashings(slot, &body, &id),
        attester_slashings: collect_attester_slashings(slot, &body, &id),
        voluntary_exits: collect_voluntary_exits(slot, &body, &id),
        execution_payload: collect_execution_payload(slot, &body, &id),
        blob_sidecars: collect_blob_sidecars(slot, &body, &id),
        withdrawals: collect_withdrawals(slot, &body, &id),
        bls_to_execution_changes: collect_bls_to_execution_changes(slot, &body, &id),
        execution_transactions: collect_execution_transactions(slot, &body, &id),
        deposit_requests: collect_deposit_requests(slot, &body, &id),
        withdrawal_requests: collect_withdrawal_requests(slot, &body, &id),
        consolidation_requests: collect_consolidation_requests(slot, &body, &id),
    })
}
