use common::{blocks::insert_timestamp, utils::bytes_to_hex};
use substreams::{pb::substreams::Clock, Hex};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::keys::transaction_message_keys;

pub fn insert_transaction_messages(tables: &mut DatabaseChanges, clock: &Clock, tx_as_bytes: &[u8], tx_hash: &str) {
    if let Ok(tx) = <TxPartial as prost::Message>::decode(tx_as_bytes) {
        if let Some(body) = tx.body {
            for (index, message) in body.messages.iter().enumerate() {
                let message_type = &message.type_url[1..];
                let message_value_hex = bytes_to_hex(&message.value);

                let keys = transaction_message_keys(tx_hash, &index.to_string());

                let row = tables
                    .push_change_composite("transaction_messages", keys, 0, table_change::Operation::Create)
                    .change("index", ("", index.to_string().as_str()))
                    .change("type", ("", message_type))
                    .change("value", ("", message_value_hex.as_str()));

                insert_timestamp(row, clock, false, true);
            }
        }
    }
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
