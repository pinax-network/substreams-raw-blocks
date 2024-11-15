use substreams_cosmos::Block;

pub fn get_size(block: &Block) -> (u64, u64, u64) {
    let transactions = &block.tx_results;

    let mut total_transactions = 0;
    let mut successful_transactions = 0;

    for transaction in transactions.iter() {
        if transaction.code == 0 {
            successful_transactions += 1;
        }
        total_transactions += 1;
    }

    // return total transactions, successful transactions, failed transactions
    (total_transactions, successful_transactions, total_transactions - successful_transactions)
}
