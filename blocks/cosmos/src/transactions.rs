use common::blocks::insert_timestamp;
use substreams::{pb::substreams::Clock, Hex};
use substreams_cosmos::Block;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::utils::compute_tx_hash;

pub fn insert_transactions(tables: &mut DatabaseChanges, clock: &Clock, block: &Block) {
    let transactions = &block.tx_results;

    for (i, transaction) in transactions.iter().enumerate() {
        let tx_as_bytes = block.txs.get(i).unwrap();
        let tx_hash = compute_tx_hash(tx_as_bytes);
        let code = transaction.code;
        let data = Hex::encode(&transaction.data);
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
    }
}
