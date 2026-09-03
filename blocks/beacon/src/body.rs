// A spec-independent view over the `oneof Body` of a Firehose beacon block.
use crate::pb::sf::beacon::r#type::v1::{
    block::Body, Attestation, AttestationData, AttesterSlashing, Blob, Deposit, Eth1Data, ExecutionRequest, ProposerSlashing, SignedBlsToExecutionChange, SignedVoluntaryExit, SyncAggregate,
    Withdrawal,
};

pub struct AttestationView<'a> {
    pub aggregation_bits: &'a [u8],
    pub data: Option<&'a AttestationData>,
    pub signature: &'a [u8],
    pub committee_bits: Option<&'a [u8]>,
}

pub struct ExecutionPayloadView<'a> {
    pub parent_hash: &'a [u8],
    pub fee_recipient: &'a [u8],
    pub state_root: &'a [u8],
    pub receipts_root: &'a [u8],
    pub logs_bloom: &'a [u8],
    pub prev_randao: &'a [u8],
    pub block_number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: Option<&'a prost_types::Timestamp>,
    pub extra_data: &'a [u8],
    pub base_fee_per_gas: &'a [u8],
    pub block_hash: &'a [u8],
    pub transactions: &'a [Vec<u8>],
    pub withdrawals: &'a [Withdrawal],
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
}

#[derive(Default)]
pub struct BodyView<'a> {
    pub randao_reveal: &'a [u8],
    pub eth1_data: Option<&'a Eth1Data>,
    pub graffiti: &'a [u8],
    pub proposer_slashings: &'a [ProposerSlashing],
    pub attester_slashings: &'a [AttesterSlashing],
    pub attestations: Vec<AttestationView<'a>>,
    pub deposits: &'a [Deposit],
    pub voluntary_exits: &'a [SignedVoluntaryExit],
    pub sync_aggregate: Option<&'a SyncAggregate>,
    pub execution_payload: Option<ExecutionPayloadView<'a>>,
    pub bls_to_execution_changes: &'a [SignedBlsToExecutionChange],
    pub blob_kzg_commitments: &'a [Vec<u8>],
    pub blobs: &'a [Blob],
    pub execution_requests: Option<&'a ExecutionRequest>,
}

fn standard(atts: &[Attestation]) -> Vec<AttestationView<'_>> {
    atts.iter()
        .map(|a| AttestationView {
            aggregation_bits: &a.aggregation_bits,
            data: a.data.as_ref(),
            signature: &a.signature,
            committee_bits: None,
        })
        .collect()
}

macro_rules! payload {
    ($opt:expr, $ep:ident => $withdrawals:expr, $blob_gas_used:expr, $excess_blob_gas:expr) => {
        $opt.as_ref().map(|$ep| ExecutionPayloadView {
            parent_hash: &$ep.parent_hash,
            fee_recipient: &$ep.fee_recipient,
            state_root: &$ep.state_root,
            receipts_root: &$ep.receipts_root,
            logs_bloom: &$ep.logs_bloom,
            prev_randao: &$ep.prev_randao,
            block_number: $ep.block_number,
            gas_limit: $ep.gas_limit,
            gas_used: $ep.gas_used,
            timestamp: $ep.timestamp.as_ref(),
            extra_data: &$ep.extra_data,
            base_fee_per_gas: &$ep.base_fee_per_gas,
            block_hash: &$ep.block_hash,
            transactions: &$ep.transactions,
            withdrawals: $withdrawals,
            blob_gas_used: $blob_gas_used,
            excess_blob_gas: $excess_blob_gas,
        })
    };
}

pub fn body_view(body: &Option<Body>) -> BodyView<'_> {
    match body {
        Some(Body::Phase0(b)) => BodyView {
            randao_reveal: &b.rando_reveal,
            eth1_data: b.eth1_data.as_ref(),
            graffiti: &b.graffiti,
            proposer_slashings: &b.proposer_slashings,
            attester_slashings: &b.attester_slashings,
            attestations: standard(&b.attestations),
            deposits: &b.deposits,
            voluntary_exits: &b.voluntary_exits,
            ..Default::default()
        },
        Some(Body::Altair(b)) => BodyView {
            randao_reveal: &b.rando_reveal,
            eth1_data: b.eth1_data.as_ref(),
            graffiti: &b.graffiti,
            proposer_slashings: &b.proposer_slashings,
            attester_slashings: &b.attester_slashings,
            attestations: standard(&b.attestations),
            deposits: &b.deposits,
            voluntary_exits: &b.voluntary_exits,
            sync_aggregate: b.sync_aggregate.as_ref(),
            ..Default::default()
        },
        Some(Body::Bellatrix(b)) => BodyView {
            randao_reveal: &b.rando_reveal,
            eth1_data: b.eth1_data.as_ref(),
            graffiti: &b.graffiti,
            proposer_slashings: &b.proposer_slashings,
            attester_slashings: &b.attester_slashings,
            attestations: standard(&b.attestations),
            deposits: &b.deposits,
            voluntary_exits: &b.voluntary_exits,
            sync_aggregate: b.sync_aggregate.as_ref(),
            execution_payload: payload!(b.execution_payload, ep => &[], None, None),
            ..Default::default()
        },
        Some(Body::Capella(b)) => BodyView {
            randao_reveal: &b.rando_reveal,
            eth1_data: b.eth1_data.as_ref(),
            graffiti: &b.graffiti,
            proposer_slashings: &b.proposer_slashings,
            attester_slashings: &b.attester_slashings,
            attestations: standard(&b.attestations),
            deposits: &b.deposits,
            voluntary_exits: &b.voluntary_exits,
            sync_aggregate: b.sync_aggregate.as_ref(),
            execution_payload: payload!(b.execution_payload, ep => &ep.withdrawals, None, None),
            ..Default::default()
        },
        Some(Body::Deneb(b)) => BodyView {
            randao_reveal: &b.rando_reveal,
            eth1_data: b.eth1_data.as_ref(),
            graffiti: &b.graffiti,
            proposer_slashings: &b.proposer_slashings,
            attester_slashings: &b.attester_slashings,
            attestations: standard(&b.attestations),
            deposits: &b.deposits,
            voluntary_exits: &b.voluntary_exits,
            sync_aggregate: b.sync_aggregate.as_ref(),
            execution_payload: payload!(b.execution_payload, ep => &ep.withdrawals, Some(ep.blob_gas_used), Some(ep.excess_blob_gas)),
            bls_to_execution_changes: &b.bls_to_execution_changes,
            blob_kzg_commitments: &b.blob_kzg_commitments,
            blobs: &b.embedded_blobs,
            execution_requests: None,
        },
        Some(Body::Electra(b)) | Some(Body::Fusaka(b)) => BodyView {
            randao_reveal: &b.rando_reveal,
            eth1_data: b.eth1_data.as_ref(),
            graffiti: &b.graffiti,
            proposer_slashings: &b.proposer_slashings,
            attester_slashings: &b.attester_slashings,
            attestations: b
                .attestations
                .iter()
                .map(|a| AttestationView {
                    aggregation_bits: &a.aggregation_bits,
                    data: a.data.as_ref(),
                    signature: &a.signature,
                    committee_bits: Some(&a.committee_bits),
                })
                .collect(),
            deposits: &b.deposits,
            voluntary_exits: &b.voluntary_exits,
            sync_aggregate: b.sync_aggregate.as_ref(),
            execution_payload: payload!(b.execution_payload, ep => &ep.withdrawals, Some(ep.blob_gas_used), Some(ep.excess_blob_gas)),
            bls_to_execution_changes: &b.bls_to_execution_changes,
            blob_kzg_commitments: &b.blob_kzg_commitments,
            blobs: &b.embedded_blobs,
            execution_requests: b.execution_requests.as_ref(),
        },
        None => BodyView::default(),
    }
}
