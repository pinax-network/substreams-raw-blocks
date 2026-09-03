use common::structs::BlockIdentity;
use substreams::scalar::BigInt;

use crate::body::BodyView;
use crate::pb::pinax::beacon::v2::*;
use crate::pb::sf::beacon::r#type::v1::{AttestationData, Block as BeaconBlock, Spec};

// Spec names follow firehose-parquet: the Firehose enum name (PHASE0, ALTAIR, ..., ELECTRA, FUSAKA).
fn spec_text(spec: i32) -> String {
    Spec::try_from(spec).map(|s| s.as_str_name().to_string()).unwrap_or_else(|_| "UNSPECIFIED".to_string())
}

macro_rules! identity {
    ($id:expr) => {
        (
            $id.block_num,
            $id.block_id.clone(),
            $id.parent_num,
            $id.parent_id.clone(),
            Some($id.timestamp.clone()),
            $id.date.clone(),
        )
    };
}

pub fn collect_block(block: &BeaconBlock, body: &BodyView, id: &BlockIdentity) -> Block {
    let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
    let eth1 = body.eth1_data.cloned().unwrap_or_default();
    Block {
        block_num,
        block_id,
        parent_num,
        parent_id,
        timestamp,
        date,

        slot: block.slot,
        parent_slot: block.parent_slot,
        proposer_index: block.proposer_index,
        root: block.root.clone(),
        parent_root: block.parent_root.clone(),
        state_root: block.state_root.clone(),
        body_root: block.body_root.clone(),
        signature: block.signature.clone(),
        spec: spec_text(block.spec),

        version: block.version,
        randao_reveal: body.randao_reveal.to_vec(),
        graffiti: body.graffiti.to_vec(),
        eth1_deposit_root: eth1.deposit_root.clone(),
        eth1_deposit_count: eth1.deposit_count,
        eth1_block_hash: eth1.block_hash.clone(),
        sync_committee_bits: body.sync_aggregate.map(|s| s.sync_commitee_bits.clone()),
        sync_committee_signature: body.sync_aggregate.map(|s| s.sync_comittee_signature.clone()),
        blob_kzg_commitments: body.blob_kzg_commitments.to_vec(),
        num_attestations: body.attestations.len() as u32,
        num_deposits: body.deposits.len() as u32,
        num_voluntary_exits: body.voluntary_exits.len() as u32,
        num_withdrawals: body.execution_payload.as_ref().map(|p| p.withdrawals.len()).unwrap_or_default() as u32,
        num_blobs: body.blobs.len() as u32,
        num_execution_transactions: body.execution_payload.as_ref().map(|p| p.transactions.len()).unwrap_or_default() as u32,
    }
}

struct AttData {
    slot: u64,
    committee_index: u64,
    beacon_block_root: Vec<u8>,
    source_epoch: u64,
    source_root: Vec<u8>,
    target_epoch: u64,
    target_root: Vec<u8>,
}

fn att_data(data: Option<&AttestationData>) -> AttData {
    let d = data.cloned().unwrap_or_default();
    let source = d.source.unwrap_or_default();
    let target = d.target.unwrap_or_default();
    AttData {
        slot: d.slot,
        committee_index: d.committee_index,
        beacon_block_root: d.beacon_block_root,
        source_epoch: source.epoch,
        source_root: source.root,
        target_epoch: target.epoch,
        target_root: target.root,
    }
}

pub fn collect_attestations(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<Attestation> {
    body.attestations
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            let d = att_data(a.data);
            Attestation {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                attestation_index: i as u32,
                slot: d.slot,
                committee_index: d.committee_index,
                aggregation_bits: a.aggregation_bits.to_vec(),
                beacon_block_root: d.beacon_block_root,
                source_epoch: d.source_epoch,
                source_root: d.source_root,
                target_epoch: d.target_epoch,
                target_root: d.target_root,
                signature: a.signature.to_vec(),
                committee_bits: a.committee_bits.map(|b| b.to_vec()),
            }
        })
        .collect()
}

pub fn collect_deposits(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<Deposit> {
    body.deposits
        .iter()
        .enumerate()
        .map(|(i, dep)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            let data = dep.data.clone().unwrap_or_default();
            Deposit {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                deposit_index: i as u32,
                pubkey: data.public_key,
                withdrawal_credentials: data.withdrawal_credentials,
                amount: data.gwei,
                signature: data.signature,
                proof: dep.proof.clone(),
            }
        })
        .collect()
}

pub fn collect_proposer_slashings(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<ProposerSlashing> {
    body.proposer_slashings
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            let h1 = s.signed_header_1.clone().unwrap_or_default();
            let h2 = s.signed_header_2.clone().unwrap_or_default();
            let m1 = h1.message.unwrap_or_default();
            let m2 = h2.message.unwrap_or_default();
            ProposerSlashing {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                slashing_index: i as u32,
                header_1_slot: m1.slot,
                header_1_proposer_index: m1.proposer_index,
                header_1_parent_root: m1.parent_root,
                header_1_state_root: m1.state_root,
                header_1_body_root: m1.body_root,
                header_2_slot: m2.slot,
                header_2_proposer_index: m2.proposer_index,
                header_2_parent_root: m2.parent_root,
                header_2_state_root: m2.state_root,
                header_2_body_root: m2.body_root,
                header_1_signature: h1.signature,
                header_2_signature: h2.signature,
            }
        })
        .collect()
}

pub fn collect_attester_slashings(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<AttesterSlashing> {
    body.attester_slashings
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            let a1 = s.attestation_1.clone().unwrap_or_default();
            let a2 = s.attestation_2.clone().unwrap_or_default();
            let d1 = att_data(a1.data.as_ref());
            let d2 = att_data(a2.data.as_ref());
            AttesterSlashing {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                slashing_index: i as u32,
                attestation_1_slot: d1.slot,
                attestation_1_committee_index: d1.committee_index,
                attestation_1_beacon_block_root: d1.beacon_block_root,
                attestation_1_source_epoch: d1.source_epoch,
                attestation_1_source_root: d1.source_root,
                attestation_1_target_epoch: d1.target_epoch,
                attestation_1_target_root: d1.target_root,
                attestation_2_slot: d2.slot,
                attestation_2_committee_index: d2.committee_index,
                attestation_2_beacon_block_root: d2.beacon_block_root,
                attestation_2_source_epoch: d2.source_epoch,
                attestation_2_source_root: d2.source_root,
                attestation_2_target_epoch: d2.target_epoch,
                attestation_2_target_root: d2.target_root,
                attestation_1_attesting_indices: a1.attesting_indices,
                attestation_1_signature: a1.signature,
                attestation_2_attesting_indices: a2.attesting_indices,
                attestation_2_signature: a2.signature,
            }
        })
        .collect()
}

pub fn collect_voluntary_exits(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<VoluntaryExit> {
    body.voluntary_exits
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            let m = e.message.clone().unwrap_or_default();
            VoluntaryExit {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                exit_index: i as u32,
                epoch: m.epoch,
                validator_index: m.validator_index,
                signature: e.signature.clone(),
            }
        })
        .collect()
}

pub fn collect_execution_payload(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<ExecutionPayload> {
    let Some(ep) = body.execution_payload.as_ref() else {
        return vec![];
    };
    let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
    vec![ExecutionPayload {
        block_num,
        block_id,
        parent_num,
        parent_id,
        timestamp,
        date,
        block_slot,
        parent_hash: ep.parent_hash.to_vec(),
        fee_recipient: ep.fee_recipient.to_vec(),
        state_root: ep.state_root.to_vec(),
        receipts_root: ep.receipts_root.to_vec(),
        prev_randao: ep.prev_randao.to_vec(),
        block_number: ep.block_number,
        gas_limit: ep.gas_limit,
        gas_used: ep.gas_used,
        payload_timestamp: ep.timestamp.map(|t| t.seconds),
        block_hash: ep.block_hash.to_vec(),
        base_fee_per_gas: ep.base_fee_per_gas.to_vec(),
        blob_gas_used: ep.blob_gas_used,
        excess_blob_gas: ep.excess_blob_gas,
        logs_bloom: ep.logs_bloom.to_vec(),
        extra_data: ep.extra_data.to_vec(),
        // Firehose serializes base_fee_per_gas as a minimal big-endian unsigned integer
        base_fee_per_gas_value: if ep.base_fee_per_gas.is_empty() {
            "0".to_string()
        } else {
            BigInt::from_unsigned_bytes_be(ep.base_fee_per_gas).to_string()
        },
        num_transactions: ep.transactions.len() as u32,
        num_withdrawals: ep.withdrawals.len() as u32,
    }]
}

pub fn collect_blob_sidecars(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<BlobSidecar> {
    body.blobs
        .iter()
        .map(|b| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            BlobSidecar {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                blob_index: b.index,
                blob: b.blob.clone(),
                kzg_commitment: b.kzg_commitment.clone(),
                kzg_proof: b.kzg_proof.clone(),
                kzg_commitment_inclusion_proof: b.kzg_commitment_inclusion_proof.clone(),
            }
        })
        .collect()
}

pub fn collect_withdrawals(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<Withdrawal> {
    let Some(ep) = body.execution_payload.as_ref() else {
        return vec![];
    };
    ep.withdrawals
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            Withdrawal {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                index: i as u32,
                withdrawal_index: w.withdrawal_index,
                validator_index: w.validator_index,
                address: w.address.clone(),
                gwei: w.gwei,
            }
        })
        .collect()
}

pub fn collect_bls_to_execution_changes(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<BlsToExecutionChange> {
    body.bls_to_execution_changes
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            let m = c.message.clone().unwrap_or_default();
            BlsToExecutionChange {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                index: i as u32,
                validator_index: m.validator_index,
                from_bls_pubkey: m.from_bls_pub_key,
                to_execution_address: m.to_execution_address,
                signature: c.signature.clone(),
            }
        })
        .collect()
}

pub fn collect_execution_transactions(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<ExecutionTransaction> {
    let Some(ep) = body.execution_payload.as_ref() else {
        return vec![];
    };
    ep.transactions
        .iter()
        .enumerate()
        .map(|(i, tx)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            ExecutionTransaction {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                block_number: ep.block_number,
                index: i as u32,
                transaction: tx.clone(),
            }
        })
        .collect()
}

pub fn collect_deposit_requests(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<DepositRequest> {
    let Some(req) = body.execution_requests else {
        return vec![];
    };
    req.deposits
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            DepositRequest {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                request_index: i as u32,
                pubkey: d.pub_key.clone(),
                withdrawal_credentials: d.withdrawal_credentials.clone(),
                amount: d.amount,
                signature: d.signature.clone(),
                index: d.index,
            }
        })
        .collect()
}

pub fn collect_withdrawal_requests(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<WithdrawalRequest> {
    let Some(req) = body.execution_requests else {
        return vec![];
    };
    req.withdrawals
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            WithdrawalRequest {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                request_index: i as u32,
                source_address: w.source_address.clone(),
                validator_pubkey: w.validator_pub_key.clone(),
                amount: w.amount,
            }
        })
        .collect()
}

pub fn collect_consolidation_requests(block_slot: u64, body: &BodyView, id: &BlockIdentity) -> Vec<ConsolidationRequest> {
    let Some(req) = body.execution_requests else {
        return vec![];
    };
    req.consolidations
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let (block_num, block_id, parent_num, parent_id, timestamp, date) = identity!(id);
            ConsolidationRequest {
                block_num,
                block_id,
                parent_num,
                parent_id,
                timestamp,
                date,
                block_slot,
                request_index: i as u32,
                source_address: c.source_address.clone(),
                source_pubkey: c.source_pub_key.clone(),
                target_pubkey: c.target_pub_key.clone(),
            }
        })
        .collect()
}
