use substreams_database_change::pb::database::TableChange;
use substreams_ethereum::pb::eth::v2::Block;

pub fn insert_size(row: &mut TableChange, block: &Block) {
    let size = block.size;
    row.change("size", ("", size.to_string().as_str()));

    // transaction status counts
    let all_transaction_status: Vec<i32> = block.transaction_traces.iter().map(|transaction| transaction.status).collect();
    insert_transaction_counts(row, all_transaction_status);

    // balance changes counts
    let all_balance_changes_reason: Vec<i32> = block.balance_changes.iter().map(|balance_change| balance_change.reason).collect();
    insert_balance_change_counts(row, all_balance_changes_reason);
}

pub fn insert_transaction_counts(row: &mut TableChange, all_transaction_status: Vec<i32>) {
    // transaction counts
    let mut total_transactions = 0;
    let mut successful_transactions = 0;
    let mut failed_transactions = 0;
    for status in all_transaction_status {
        if status == 1 {
            successful_transactions += 1;
        } else {
            failed_transactions += 1;
        }
        total_transactions += 1;
    }
    row.change("total_transactions", ("", total_transactions.to_string().as_str()))
        .change("successful_transactions", ("", successful_transactions.to_string().as_str()))
        .change("failed_transactions", ("", failed_transactions.to_string().as_str()));
}

pub fn insert_balance_change_counts(row: &mut TableChange, all_balance_changes_reason: Vec<i32>) {
    // transaction counts
    let total_balance_changes = all_balance_changes_reason.len();
    let mut total_withdrawals = 0;
    for reason in all_balance_changes_reason {
        if reason == 16 {
            total_withdrawals += 1;
        }
    }
    row.change("total_balance_changes", ("", total_balance_changes.to_string().as_str()))
        .change("total_withdrawals", ("", total_withdrawals.to_string().as_str()));
}
