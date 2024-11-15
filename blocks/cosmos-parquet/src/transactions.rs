use common::{blocks::insert_timestamp, structs::BlockTimestamp, utils::bytes_to_hex};
use substreams::pb::substreams::Clock;
use substreams_cosmos::Block;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::{events::insert_tx_events, pb::cosmos::rawblocks::Transaction as RawTransaction, transaction_messages::insert_transaction_messages, utils::compute_tx_hash};

pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    let transactions = &block.tx_results;

    for (i, transaction) in transactions.iter().enumerate() {
        let tx_as_bytes = block.txs.get(i).unwrap();
        let tx_hash = compute_tx_hash(tx_as_bytes);
        let code = transaction.code;
        let data = bytes_to_hex(&transaction.data);
        let log = &transaction.log;
        let info = &transaction.info;
        let gas_wanted = transaction.gas_wanted;
        let gas_used = transaction.gas_used;
        let codespace = &transaction.codespace;

        let row = tables
            .push_change("transactions", &tx_hash, 0, table_change::Operation::Create)
            .change("index", ("", i.to_string().as_str()))
            .change("code", ("", code.to_string().as_str()))
            .change("data", ("", data.as_str()))
            .change("log", ("", log.as_str()))
            .change("info", ("", info.as_str()))
            .change("gas_wanted", ("", gas_wanted.to_string().as_str()))
            .change("gas_used", ("", gas_used.to_string().as_str()))
            .change("codespace", ("", codespace.as_str()));

        insert_timestamp(row, clock, false, true);

        insert_tx_events(tables, clock, transaction, &tx_hash);

        insert_transaction_messages(tables, clock, tx_as_bytes, &tx_hash);
    }
}

pub fn collect_transactions(block: &Block, timestamp: &BlockTimestamp) -> Vec<RawTransaction> {
    let mut vec: Vec<RawTransaction> = vec![];

    for (i, transaction) in block.tx_results.iter().enumerate() {
        let tx_as_bytes = block.txs.get(i).unwrap();
        let tx_hash = compute_tx_hash(tx_as_bytes);
        let code = transaction.code;
        let data = bytes_to_hex(&transaction.data);

        vec.push(RawTransaction {
            block_time: Some(timestamp.time),
            block_number: timestamp.number,
            block_date: timestamp.date.clone(),
            block_hash: bytes_to_hex(&block.hash),
            index: i as u32,
            hash: tx_hash,
            code,
            data,
            log: transaction.log.clone(),
            info: transaction.info.clone(),
            gas_wanted: transaction.gas_wanted,
            gas_used: transaction.gas_used,
            codespace: transaction.codespace.clone(),
        });
    }

    vec
}
