use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_cosmos::Block;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::size::insert_size;

pub fn insert_blocks(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    let header = block.header.as_ref().unwrap();
    let consensus = header.version.as_ref().unwrap();

    let consensus_block = consensus.block;
    let consensus_app = consensus.app;
    let chain_id = &header.chain_id;
    let last_commit_hash = bytes_to_hex(&header.last_commit_hash);
    let last_block_id = match &header.last_block_id {
        Some(block_id) => bytes_to_hex(&block_id.hash),
        None => String::new(),
    };
    let data_hash = bytes_to_hex(&header.data_hash);
    let validators_hash = bytes_to_hex(&header.validators_hash);
    let next_validators_hash = bytes_to_hex(&header.next_validators_hash);
    let consensus_hash = bytes_to_hex(&header.consensus_hash);
    let app_hash = bytes_to_hex(&header.app_hash);
    let last_results_hash = bytes_to_hex(&header.last_results_hash);
    let evidence_hash = bytes_to_hex(&header.evidence_hash);
    let proposer_address = bytes_to_hex(&header.proposer_address);

    let row = tables
        .push_change("blocks", &clock.number.to_string(), 0, table_change::Operation::Create)
        .change("version_consensus_block", ("", consensus_block.to_string().as_str()))
        .change("version_consensus_app", ("", consensus_app.to_string().as_str()))
        .change("chain_id", ("", chain_id.as_str()))
        .change("last_commit_hash", ("", last_commit_hash.as_str()))
        .change("last_block_id", ("", last_block_id.as_str()))
        .change("data_hash", ("", data_hash.as_str()))
        .change("validators_hash", ("", validators_hash.as_str()))
        .change("next_validators_hash", ("", next_validators_hash.as_str()))
        .change("consensus_hash", ("", consensus_hash.as_str()))
        .change("app_hash", ("", app_hash.as_str()))
        .change("last_results_hash", ("", last_results_hash.as_str()))
        .change("evidence_hash", ("", evidence_hash.as_str()))
        .change("proposer_address", ("", proposer_address.as_str()));

    insert_timestamp(row, clock, true, false);

    insert_size(row, block);
}
