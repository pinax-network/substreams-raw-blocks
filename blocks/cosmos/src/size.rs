use substreams_cosmos::Block;
use substreams_database_change::pb::database::TableChange;

pub fn insert_size(row: &mut TableChange, block: &Block) {
    let transactions = &block.tx_results;

    let mut total_transactions = 0;
    let mut successful_transactions = 0;

    for transaction in transactions.iter() {
        if transaction.code == 0 {
            successful_transactions += 1;
        }
        total_transactions += 1;
    }

    let failed_transactions = total_transactions - successful_transactions;

    row.change("total_transactions", ("", total_transactions.to_string().as_str()))
        .change("successful_transactions", ("", successful_transactions.to_string().as_str()))
        .change("failed_transactions", ("", failed_transactions.to_string().as_str()));
}
