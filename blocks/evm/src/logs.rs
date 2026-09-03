use common::structs::BlockIdentity;
use substreams_ethereum::pb::eth::v2::TransactionTrace;

use crate::pb::pinax::evm::v2::Log;
use crate::utils::empty_to_none;

// DetailLevel: BASE. Logs are read from the transaction receipt, which is
// populated at every detail level (EXTENDED duplicates them inside the calls).
pub fn collect_logs(tx: &TransactionTrace, id: &BlockIdentity) -> Vec<Log> {
    let Some(receipt) = tx.receipt.as_ref() else {
        return vec![];
    };

    receipt
        .logs
        .iter()
        .map(|log| Log {
            block_num: id.block_num,
            block_id: id.block_id.clone(),
            parent_num: id.parent_num,
            parent_id: id.parent_id.clone(),
            timestamp: Some(id.timestamp.clone()),
            date: id.date.clone(),

            block_number: id.block_num,
            tx_hash: tx.hash.clone(),
            tx_index: tx.index,
            log_index: log.index,
            block_index: log.block_index,
            address: log.address.clone(),
            topic0: log.topics.first().cloned(),
            topic1: log.topics.get(1).cloned(),
            topic2: log.topics.get(2).cloned(),
            topic3: log.topics.get(3).cloned(),
            data: empty_to_none(&log.data),

            ordinal: log.ordinal,
        })
        .collect()
}
