use common::structs::BlockTimestamp;

use crate::{
    pb::{
        pinax::starknet::v1::Call,
        sf::starknet::r#type::v1::{Block, TransactionWithReceipt},
    },
    transactions::TransactionData,
    utils::BlockHashes,
};

pub fn _collect_calls(_block: &Block, _transaction: &TransactionWithReceipt, _timestamp: &BlockTimestamp, _block_hashes: &BlockHashes, _tx_data: &TransactionData) -> Vec<Call> {
    let mut _calls: Vec<Call> = Vec::new();

    _calls
}
