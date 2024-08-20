use substreams_antelope::Block;
use substreams_database_change::pb::database::TableChange;

pub fn compute_size(block: &Block) -> usize {
    let mut size = 0;

    // How to calculate Antelope block size (estimated) in bytes
    // ------------------------------
    // action raw data
    // trace console
    // trace raw return value
    // trace receipt digest
    // transaction db op new data
    // transaction db op old data
    for transaction in block.transaction_traces() {
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
    size
}

pub fn insert_size(row: &mut TableChange, block: &Block) {
    let size = compute_size(block);
    row.change("size", ("", size.to_string().as_str()));
}
