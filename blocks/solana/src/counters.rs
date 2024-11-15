use crate::structs::BlockCounters;

use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::utils::VOTE_INSTRUCTION;

pub fn get_block_counters(block: &Block) -> BlockCounters {
    // Counters
    let total_transactions = block.transactions.len();
    let mut successful_transactions = 0;
    let mut total_vote_transactions = 0;
    let mut successful_vote_transactions = 0;

    for trx in block.transactions.iter() {
        let transaction = trx.transaction.as_ref().expect("Transaction is missing");

        let message = transaction.message.as_ref().expect("Transaction message is missing");

        let is_vote = message.account_keys.iter().any(|key| key == &VOTE_INSTRUCTION);

        if is_vote {
            total_vote_transactions += 1;
        }

        if trx.meta.as_ref().map_or(false, |meta| meta.err.is_none()) {
            successful_transactions += 1;
            if is_vote {
                successful_vote_transactions += 1;
            }
        }
    }

    let failed_transactions = total_transactions - successful_transactions;
    let total_non_vote_transactions = total_transactions - total_vote_transactions;
    let successful_non_vote_transactions = successful_transactions - successful_vote_transactions;
    let failed_vote_transactions = total_vote_transactions - successful_vote_transactions;
    let failed_non_vote_transactions = failed_transactions - failed_vote_transactions;

    BlockCounters {
        total_transactions: total_transactions as u64,
        successful_transactions: successful_transactions as u64,
        failed_transactions: failed_transactions as u64,
        total_vote_transactions: total_vote_transactions as u64,
        successful_vote_transactions: successful_vote_transactions as u64,
        total_non_vote_transactions: total_non_vote_transactions as u64,
        successful_non_vote_transactions: successful_non_vote_transactions as u64,
        failed_vote_transactions: failed_vote_transactions as u64,
        failed_non_vote_transactions: failed_non_vote_transactions as u64,
    }
}
