use common::{structs::BlockTimestamp, utils::bytes_to_hex};
use substreams_cosmos::Block;

use crate::pb::cosmos::rawblocks::TransactionMessage;

pub fn collect_transaction_messages(block: &Block, timestamp: &BlockTimestamp) -> Vec<TransactionMessage> {
    let mut vec: Vec<TransactionMessage> = vec![];

    for i in 0..block.tx_results.len() {
        if let Ok(tx) = <TxPartial as prost::Message>::decode(block.txs[i].as_slice()) {
            if let Some(body) = tx.body {
                for (index, message) in body.messages.iter().enumerate() {
                    vec.push(TransactionMessage {
                        block_time: Some(timestamp.time),
                        block_number: timestamp.number,
                        block_date: timestamp.date.clone(),
                        block_hash: bytes_to_hex(&block.hash),
                        tx_hash: bytes_to_hex(&block.txs[i]),
                        index: index as u32,
                        r#type: message.type_url[1..].to_string(),
                        value: bytes_to_hex(&message.value),
                    });
                }
            }
        }
    }

    vec
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxPartial {
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<TxBodyPartial>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxBodyPartial {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
