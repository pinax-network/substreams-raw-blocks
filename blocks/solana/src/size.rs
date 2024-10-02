use substreams_database_change::pb::database::TableChange;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

// Start Generation Here
pub fn insert_size(row: &mut TableChange, block: &Block) {
    // Counters
    // let mut size = 0;
    let total_transactions = block.transactions.len();
    let total_rewards = block.rewards.len();
    let mut successful_transactions = 0;
    let mut total_instructions = 0;

    // Calculate Solana block size (estimated) in bytes
    // -----------------------------------------------
    // Transaction signatures
    // Instructions data
    // Logs and other relevant fields
    for transaction in block.transactions.iter() {
        if transaction.meta.as_ref().map_or(false, |meta| meta.err.is_none()) {
            successful_transactions += 1;
        }

        if let Some(tx) = transaction.transaction.as_ref() {
            // Estimate size based on signatures and instructions
            // size += tx.signatures.iter().map(|s| s.len()).sum::<usize>();
            if let Some(message) = tx.message.as_ref() {
                total_instructions += message.instructions.len();
                // for instruction in message.instructions.iter() {
                //     // size += instruction.program.len();
                //     // size += instruction.accounts.len() * 32; // Assuming each account is a 32-byte address
                //     // size += instruction.data.len();
                // }
            }
        }

        // Estimate size for logs if available
        // if let Some(meta) = transaction.meta.as_ref() {
        //     if let Some(logs) = &meta.log_messages {
        //         for log in logs.iter() {
        //             size += log.len();
        //         }
        //     }
        // }
    }

    let failed_transactions = total_transactions - successful_transactions;

    row.change("total_transactions", ("", total_transactions.to_string().as_str()))
        // .change("size", ("", size.to_string().as_str()))
        .change("successful_transactions", ("", successful_transactions.to_string().as_str()))
        .change("failed_transactions", ("", failed_transactions.to_string().as_str()))
        .change("total_instructions", ("", total_instructions.to_string().as_str()))
        .change("total_rewards", ("", total_rewards.to_string().as_str()));
}
