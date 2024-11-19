use substreams_antelope::Block;

pub struct BlockSize {
    pub size: u64,
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub total_actions: u64,
    pub total_db_ops: u64,
}

pub fn collect_size(block: &Block) -> BlockSize {
    // counters
    let mut size = 0;
    let mut total_transactions = 0;
    let mut successful_transactions = 0;
    let mut failed_transactions = 0;
    let mut total_actions = 0;
    let mut total_db_ops = 0;

    // How to calculate Antelope block size (estimated) in bytes
    // ------------------------------
    // action raw data
    // trace console
    // trace raw return value
    // trace receipt digest
    // transaction db op new data
    // transaction db op old data
    for transaction in block.transaction_traces() {
        let status = transaction.receipt.clone().unwrap_or_default().status;
        if status == 1 {
            successful_transactions += 1;
        } else {
            failed_transactions += 1;
        }
        total_transactions += 1;
        total_actions += transaction.action_traces.len();
        total_db_ops += transaction.db_ops.len();

        // remaining used for calculate block size
        for trace in transaction.action_traces.iter() {
            match &trace.receipt {
                Some(receipt) => {
                    size += receipt.digest.len();
                }
                None => {}
            }
            match &trace.action {
                Some(action) => {
                    size += action.raw_data.len();
                    size += action.json_data.len();
                }
                None => {}
            }
            size += trace.console.len();
            size += trace.raw_return_value.len();
            size += trace.json_return_value.len();
        }

        for db_op in transaction.db_ops.iter() {
            size += db_op.new_data.len();
            size += db_op.old_data.len();
            size += db_op.new_data_json.len();
            size += db_op.old_data.len();
            size += db_op.old_data_json.len();
        }
    }

    BlockSize {
        size: size as u64,
        total_transactions,
        successful_transactions,
        failed_transactions,
        total_actions: total_actions as u64,
        total_db_ops: total_db_ops as u64,
    }
}
