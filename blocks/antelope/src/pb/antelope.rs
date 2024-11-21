// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
    #[prost(message, repeated, tag="2")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
    #[prost(message, repeated, tag="3")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
    #[prost(message, repeated, tag="4")]
    pub db_ops: ::prost::alloc::vec::Vec<DbOp>,
    #[prost(message, repeated, tag="5")]
    pub feature_ops: ::prost::alloc::vec::Vec<FeatureOp>,
    #[prost(message, repeated, tag="6")]
    pub perm_ops: ::prost::alloc::vec::Vec<PermOp>,
    #[prost(message, repeated, tag="7")]
    pub table_ops: ::prost::alloc::vec::Vec<TableOp>,
    #[prost(message, repeated, tag="8")]
    pub ram_ops: ::prost::alloc::vec::Vec<RamOp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// clock
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub number: u64,
    #[prost(string, tag="3")]
    pub date: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub hash: ::prost::alloc::string::String,
    /// block
    #[prost(string, tag="5")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub producer: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub confirmed: u32,
    #[prost(uint32, tag="8")]
    pub schedule_version: u32,
    #[prost(uint32, tag="9")]
    pub version: u32,
    #[prost(string, tag="10")]
    pub producer_signature: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub dpos_proposed_irreversible_blocknum: u32,
    #[prost(uint32, tag="12")]
    pub dpos_irreversible_blocknum: u32,
    /// roots
    #[prost(string, tag="13")]
    pub transaction_mroot: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub action_mroot: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="15")]
    pub blockroot_merkle_active_nodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="16")]
    pub blockroot_merkle_node_count: u32,
    #[prost(string, tag="17")]
    pub action_mroot_savanna: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub block_signing_key: ::prost::alloc::string::String,
    #[prost(uint32, repeated, tag="19")]
    pub confirm_count: ::prost::alloc::vec::Vec<u32>,
    /// counters
    #[prost(uint64, tag="20")]
    pub size: u64,
    #[prost(uint64, tag="21")]
    pub total_transactions: u64,
    #[prost(uint64, tag="22")]
    pub successful_transactions: u64,
    #[prost(uint64, tag="23")]
    pub failed_transactions: u64,
    #[prost(uint64, tag="24")]
    pub total_actions: u64,
    #[prost(uint64, tag="25")]
    pub total_db_ops: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// block
    #[prost(message, optional, tag="1")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// transaction
    #[prost(string, tag="5")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub index: u64,
    #[prost(int64, tag="7")]
    pub elapsed: i64,
    #[prost(uint64, tag="8")]
    pub net_usage: u64,
    #[prost(bool, tag="9")]
    pub scheduled: bool,
    #[prost(uint32, tag="10")]
    pub cpu_usage_micro_seconds: u32,
    #[prost(uint32, tag="11")]
    pub net_usage_words: u32,
    #[prost(string, tag="12")]
    pub status: ::prost::alloc::string::String,
    #[prost(uint32, tag="13")]
    pub status_code: u32,
    #[prost(bool, tag="14")]
    pub success: bool,
    #[prost(string, tag="15")]
    pub transaction_mroot: ::prost::alloc::string::String,
    /// creation flat node
    #[prost(int32, repeated, tag="16")]
    pub creator_action_indexes: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, repeated, tag="17")]
    pub execution_action_indexes: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// block
    #[prost(message, optional, tag="1")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// transaction
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub tx_success: bool,
    /// receipt
    #[prost(uint64, tag="7")]
    pub abi_sequence: u64,
    #[prost(uint64, tag="8")]
    pub code_sequence: u64,
    #[prost(string, tag="9")]
    pub digest: ::prost::alloc::string::String,
    #[prost(uint64, tag="10")]
    pub global_sequence: u64,
    #[prost(string, tag="11")]
    pub receipt_receiver: ::prost::alloc::string::String,
    #[prost(uint64, tag="12")]
    pub recv_sequence: u64,
    #[prost(uint64, repeated, tag="13")]
    pub auth_sequence: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag="14")]
    pub auth_sequence_account_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// action
    #[prost(string, tag="20")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub json_data: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub raw_data: ::prost::alloc::string::String,
    #[prost(uint32, tag="24")]
    pub index: u32,
    #[prost(uint32, tag="25")]
    pub action_ordinal: u32,
    #[prost(string, tag="26")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(bool, tag="27")]
    pub context_free: bool,
    #[prost(int64, tag="28")]
    pub elapsed: i64,
    #[prost(string, tag="29")]
    pub console: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub raw_return_value: ::prost::alloc::string::String,
    #[prost(string, tag="31")]
    pub json_return_value: ::prost::alloc::string::String,
    #[prost(uint32, tag="32")]
    pub creator_action_ordinal: u32,
    #[prost(uint32, tag="33")]
    pub closest_unnotified_ancestor_action_ordinal: u32,
    #[prost(string, tag="34")]
    pub action_mroot: ::prost::alloc::string::String,
    /// ram deltas
    #[prost(string, repeated, tag="35")]
    pub account_ram_deltas_account: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag="36")]
    pub account_ram_deltas: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbOp {
    /// block
    #[prost(message, optional, tag="1")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// transaction
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub tx_success: bool,
    /// action
    #[prost(uint32, tag="7")]
    pub action_index: u32,
    /// database operations
    #[prost(uint32, tag="8")]
    pub index: u32,
    #[prost(string, tag="9")]
    pub operation: ::prost::alloc::string::String,
    #[prost(uint32, tag="10")]
    pub operation_code: u32,
    #[prost(string, tag="11")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub scope: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub primary_key: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub old_payer: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub new_payer: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub old_data: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub new_data: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub old_data_json: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub new_data_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureOp {
    /// block
    #[prost(message, optional, tag="1")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// transaction
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub tx_success: bool,
    /// action
    #[prost(uint32, tag="7")]
    pub action_index: u32,
    /// feature operations
    #[prost(string, tag="8")]
    pub feature_digest: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub description_digest: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub protocol_feature_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermOp {
    /// block
    #[prost(message, optional, tag="1")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// transaction
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub tx_success: bool,
    /// action
    #[prost(uint32, tag="7")]
    pub action_index: u32,
    /// permission operations
    #[prost(string, tag="8")]
    pub operation: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub operation_code: u32,
    #[prost(uint64, tag="10")]
    pub id: u64,
    #[prost(uint64, tag="11")]
    pub parent_id: u64,
    #[prost(string, tag="12")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="14")]
    pub threshold: u32,
    /// -- authority --
    /// accounts
    #[prost(string, repeated, tag="15")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag="16")]
    pub accounts_weight: ::prost::alloc::vec::Vec<u32>,
    /// keys
    #[prost(string, repeated, tag="17")]
    pub keys_public_key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag="18")]
    pub keys_weight: ::prost::alloc::vec::Vec<u32>,
    /// waits
    #[prost(uint32, repeated, tag="19")]
    pub wait_sec: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag="20")]
    pub wait_weight: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableOp {
    /// block
    #[prost(message, optional, tag="1")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// transaction
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub tx_success: bool,
    /// action
    #[prost(uint32, tag="7")]
    pub action_index: u32,
    /// table operations
    #[prost(uint32, tag="8")]
    pub index: u32,
    #[prost(string, tag="9")]
    pub operation: ::prost::alloc::string::String,
    #[prost(uint32, tag="10")]
    pub operation_code: u32,
    #[prost(string, tag="11")]
    pub payer: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub scope: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub table_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RamOp {
    /// block
    #[prost(message, optional, tag="1")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(string, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub block_date: ::prost::alloc::string::String,
    /// transaction
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub tx_success: bool,
    /// action
    #[prost(uint32, tag="7")]
    pub action_index: u32,
    /// ram operations
    #[prost(string, tag="8")]
    pub operation: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub operation_code: u32,
    #[prost(string, tag="10")]
    pub payer: ::prost::alloc::string::String,
    #[prost(int64, tag="11")]
    pub delta: i64,
    #[prost(uint64, tag="12")]
    pub usage: u64,
    #[prost(string, tag="13")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(uint32, tag="14")]
    pub namespace_code: u32,
    #[prost(string, tag="15")]
    pub action: ::prost::alloc::string::String,
    #[prost(uint32, tag="16")]
    pub action_code: u32,
    #[prost(string, tag="17")]
    pub unique_key: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
