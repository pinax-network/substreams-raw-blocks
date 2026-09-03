use common::structs::BlockIdentity;
use substreams::scalar::BigInt;

use crate::pb::pinax::starknet::v2::Transaction;
use crate::pb::sf::starknet::r#type::v1::{transaction_with_receipt::Transaction as Tx, ExecutionStatus, FeeDataAvailabilityMode, ResourceBounds, TransactionType, TransactionWithReceipt};

// Felts carrying amounts (fees, nonces, gas prices) are big-endian unsigned integers.
pub fn felt_to_uint256(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "0".to_string();
    }
    BigInt::from_unsigned_bytes_be(bytes).to_string()
}

fn opt_uint256(bytes: &[u8]) -> Option<String> {
    (!bytes.is_empty()).then(|| felt_to_uint256(bytes))
}

fn da_mode(v: i32) -> Option<String> {
    FeeDataAvailabilityMode::try_from(v)
        .ok()
        .map(|m| m.as_str_name().trim_start_matches("FEE_DATA_AVAILABILITY_MODE_").to_string())
}

fn non_empty(s: &str) -> Option<String> {
    (!s.is_empty()).then(|| s.to_string())
}

pub fn collect_transaction(tx: &TransactionWithReceipt, index: u32, id: &BlockIdentity) -> Transaction {
    let receipt = tx.receipt.clone().unwrap_or_default();
    let fee = receipt.actual_fee.clone().unwrap_or_default();
    let res = receipt.execution_resources.clone().unwrap_or_default();
    let da = res.data_availability.unwrap_or_default();

    let mut row = Transaction {
        block_num: id.block_num,
        block_id: id.block_id.clone(),
        parent_num: id.parent_num,
        parent_id: id.parent_id.clone(),
        timestamp: Some(id.timestamp.clone()),
        date: id.date.clone(),

        hash: receipt.transaction_hash.clone(),
        index,
        r#type: TransactionType::try_from(receipt.r#type)
            .map(|t| t.as_str_name().trim_start_matches("TRANSACTION_TYPE_").to_string())
            .unwrap_or_else(|_| "UNKNOWN".into()),
        execution_status: ExecutionStatus::try_from(receipt.execution_status)
            .map(|s| s.as_str_name().trim_start_matches("EXECUTION_STATUS_").to_string())
            .unwrap_or_else(|_| "UNKNOWN".into()),
        revert_reason: non_empty(&receipt.revert_reason),
        actual_fee_amount: felt_to_uint256(&fee.amount),
        actual_fee_unit: fee.unit.clone(),
        contract_address: receipt.contract_address.clone(),
        message_hash: non_empty(&receipt.message_hash),
        execution_resources_steps: res.steps,
        execution_resources_memory_holes: res.memory_holes,
        execution_resources_range_check_builtin_applications: res.range_check_builtin_applications,
        execution_resources_pedersen_builtin_applications: res.pedersen_builtin_applications,
        execution_resources_poseidon_builtin_applications: res.poseidon_builtin_applications,
        execution_resources_ec_op_builtin_applications: res.ec_op_builtin_applications,
        execution_resources_ecdsa_builtin_applications: res.ecdsa_builtin_applications,
        execution_resources_bitwise_builtin_applications: res.bitwise_builtin_applications,
        execution_resources_keccak_builtin_applications: res.keccak_builtin_applications,
        execution_resources_segment_arena_builtin: res.segment_arena_builtin,
        execution_resources_data_availability_l1_gas: da.l1_gas,
        execution_resources_data_availability_l1_data_gas: da.l1_data_gas,
        num_events: receipt.events.len() as u32,
        num_messages_sent: receipt.messages_sent.len() as u32,
        ..Default::default()
    };

    let mut bounds = |b: &Option<ResourceBounds>| {
        if let Some(b) = b {
            if let Some(l1) = &b.l1_gas {
                row.resource_bounds_l1_gas_max_amount = Some(l1.max_amount.clone());
                row.resource_bounds_l1_gas_max_price_per_unit = Some(l1.max_price_per_unit.clone());
            }
            if let Some(l2) = &b.l2_gas {
                row.resource_bounds_l2_gas_max_amount = Some(l2.max_amount.clone());
                row.resource_bounds_l2_gas_max_price_per_unit = Some(l2.max_price_per_unit.clone());
            }
        }
    };

    match &tx.transaction {
        Some(Tx::InvokeTransactionV0(t)) => {
            row.version = t.version.clone();
            row.max_fee = opt_uint256(&t.max_fee);
            row.signature = t.signature.clone();
            row.contract_address = t.contract_address.clone();
            row.entry_point_selector = t.entry_point_selector.clone();
            row.calldata = t.calldata.clone();
        }
        Some(Tx::InvokeTransactionV1(t)) => {
            row.version = t.version.clone();
            row.max_fee = opt_uint256(&t.max_fee);
            row.signature = t.signature.clone();
            row.nonce = opt_uint256(&t.nonce);
            row.sender_address = t.sender_address.clone();
            row.calldata = t.calldata.clone();
        }
        Some(Tx::InvokeTransactionV3(t)) => {
            bounds(&t.resource_bounds);
            row.version = t.version.clone();
            row.signature = t.signature.clone();
            row.nonce = opt_uint256(&t.nonce);
            row.sender_address = t.sender_address.clone();
            row.calldata = t.calldata.clone();
            row.tip = opt_uint256(&t.tip);
            row.paymaster_data = t.paymaster_data.clone();
            row.account_deployment_data = t.account_deployment_data.clone();
            row.nonce_data_availability_mode = da_mode(t.nonce_data_availability_mode);
            row.fee_data_availability_mode = da_mode(t.fee_data_availability_mode);
        }
        Some(Tx::L1HandlerTransaction(t)) => {
            row.version = t.version.clone();
            row.nonce = non_empty(&t.nonce).map(|n| {
                n.strip_prefix("0x")
                    .map(|h| BigInt::from_unsigned_bytes_be(&hex::decode(format!("{}{}", if h.len() % 2 == 1 { "0" } else { "" }, h)).unwrap_or_default()).to_string())
                    .unwrap_or(n)
            });
            row.contract_address = t.contract_address.clone();
            row.entry_point_selector = t.entry_point_selector.clone();
            row.calldata = t.calldata.clone();
        }
        Some(Tx::DeclareTransactionV0(t)) => {
            row.version = t.version.clone();
            row.max_fee = opt_uint256(&t.max_fee);
            row.signature = t.signature.clone();
            row.sender_address = t.sender_address.clone();
            row.class_hash = t.class_hash.clone();
        }
        Some(Tx::DeclareTransactionV1(t)) => {
            row.version = t.version.clone();
            row.max_fee = opt_uint256(&t.max_fee);
            row.signature = t.signature.clone();
            row.nonce = opt_uint256(&t.nonce);
            row.sender_address = t.sender_address.clone();
            row.class_hash = t.class_hash.clone();
        }
        Some(Tx::DeclareTransactionV2(t)) => {
            row.version = t.version.clone();
            row.max_fee = opt_uint256(&t.max_fee);
            row.signature = t.signature.clone();
            row.nonce = opt_uint256(&t.nonce);
            row.sender_address = t.sender_address.clone();
            row.class_hash = t.class_hash.clone();
            row.compiled_class_hash = t.compiled_class_hash.clone();
        }
        Some(Tx::DeclareTransactionV3(t)) => {
            bounds(&t.resource_bounds);
            row.version = t.version.clone();
            row.signature = t.signature.clone();
            row.nonce = opt_uint256(&t.nonce);
            row.sender_address = t.sender_address.clone();
            row.class_hash = t.class_hash.clone();
            row.compiled_class_hash = t.compiled_class_hash.clone();
            row.tip = opt_uint256(&t.tip);
            row.paymaster_data = t.paymaster_data.clone();
            row.account_deployment_data = t.account_deployment_data.clone();
            row.nonce_data_availability_mode = da_mode(t.nonce_data_availability_mode);
            row.fee_data_availability_mode = da_mode(t.fee_data_availability_mode);
        }
        Some(Tx::DeployTransactionV0(t)) => {
            row.version = t.version.clone();
            row.class_hash = t.class_hash.clone();
            row.contract_address_salt = t.contract_address_salt.clone();
            row.constructor_calldata = t.constructor_calldata.clone();
        }
        Some(Tx::DeployAccountTransactionV1(t)) => {
            row.version = t.version.clone();
            row.max_fee = opt_uint256(&t.max_fee);
            row.signature = t.signature.clone();
            row.nonce = opt_uint256(&t.nonce);
            row.class_hash = t.class_hash.clone();
            row.contract_address_salt = t.contract_address_salt.clone();
            row.constructor_calldata = t.constructor_calldata.clone();
        }
        Some(Tx::DeployAccountTransactionV3(t)) => {
            bounds(&t.resource_bounds);
            row.version = t.version.clone();
            row.signature = t.signature.clone();
            row.nonce = opt_uint256(&t.nonce);
            row.class_hash = t.class_hash.clone();
            row.contract_address_salt = t.contract_address_salt.clone();
            row.constructor_calldata = t.constructor_calldata.clone();
            row.tip = opt_uint256(&t.tip);
            row.paymaster_data = t.paymaster_data.clone();
            row.nonce_data_availability_mode = da_mode(t.nonce_data_availability_mode);
            row.fee_data_availability_mode = da_mode(t.fee_data_availability_mode);
        }
        None => {}
    }
    row
}
