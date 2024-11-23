use common::structs::BlockTimestamp;
use substreams::Hex;
use substreams_cosmos::Block;

use crate::pb::pinax::cosmos::v1::TransactionMessage;

pub fn collect_tx_transaction_messages(block: &Block, tx_index: usize, tx_hash: &str, timestamp: &BlockTimestamp) -> Vec<TransactionMessage> {
    let mut vec: Vec<TransactionMessage> = vec![];

    if let Ok(tx) = <TxPartial as prost::Message>::decode(block.txs[tx_index].as_slice()) {
        if let Some(body) = tx.body {
            for (index, message) in body.messages.iter().enumerate() {
                vec.push(TransactionMessage {
                    block_time: Some(timestamp.time),
                    block_number: timestamp.number,
                    block_date: timestamp.date.clone(),
                    block_hash: timestamp.hash.clone(),
                    tx_hash: tx_hash.to_string(),
                    index: index as u32,
                    r#type: message.type_url[1..].to_string(),
                    value: Hex::encode(&message.value),
                });
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
