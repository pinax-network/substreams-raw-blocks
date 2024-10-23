use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::utils::VOTE_INSTRUCTION;

#[substreams::handlers::map]
fn blocks_without_votes_all(mut block: Block) -> Result<Block, substreams::errors::Error> {
    block.transactions.retain(|trx| {
        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => return false,
        };
        let message = transaction.message.as_ref().expect("Message is missing");

        // Retain only transactions that do **not** contain a vote instruction
        !message.account_keys.iter().any(|v| v == &VOTE_INSTRUCTION)
    });

    Ok(block)
}
