use substreams_cosmos::Block;

pub fn get_size(block: &Block) -> BlockSize {
    let transactions = &block.tx_results;

    let mut total_transactions = 0;
    let mut successful_transactions = 0;

    for transaction in transactions.iter() {
        if transaction.code == 0 {
            successful_transactions += 1;
        }
        total_transactions += 1;
    }

    BlockSize {
        total_transactions,
        successful_transactions,
        failed_transactions: total_transactions - successful_transactions,
    }
}

pub struct BlockSize {
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
}
