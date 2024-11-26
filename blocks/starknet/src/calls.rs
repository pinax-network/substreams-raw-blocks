use common::structs::BlockTimestamp;

use crate::{
    pb::{
        pinax::starknet::v1::Call,
        sf::starknet::r#type::v1::{Block, TransactionWithReceipt},
    },
    transactions::TransactionData,
    utils::BlockHashes,
};

pub fn collect_calls(block: &Block, transaction: &TransactionWithReceipt, timestamp: &BlockTimestamp, block_hashes: &BlockHashes, tx_data: &TransactionData) -> Vec<Call> {
    let mut calls: Vec<Call> = Vec::new();

    calls
}
