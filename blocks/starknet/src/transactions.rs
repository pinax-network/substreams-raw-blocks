use common::{
    structs::BlockTimestamp,
    utils::{bytes_to_hex, u8_2d_vec_to_string_array},
};

use crate::{
    blocks::l1_da_mode_to_string,
    pb::{
        pinax::starknet::v1::Transaction,
        sf::starknet::r#type::v1::{transaction_with_receipt::Transaction as TrxType, Block, TransactionWithReceipt},
    },
    utils::BlockHashes,
};

pub fn collect_transaction(block: &Block, transaction: &TransactionWithReceipt, tx_index: u32, timestamp: &BlockTimestamp, block_hashes: &BlockHashes) -> Transaction {
    let receipt = transaction.receipt.as_ref().expect("Receipt is missing");

    let actual_fee = receipt.actual_fee.as_ref().expect("Actual fee missing");

    let execution_resources = receipt.execution_resources.as_ref().expect("Execution resources missing");
    let exec_data_availability = execution_resources.data_availability.unwrap();

    let l1_gas_price = block.l1_gas_price.as_ref().expect("L1 gas price missing");
    let l1_data_gas_price = block.l1_data_gas_price.as_ref().expect("L1 data gas price missing");

    let tx_data = extract_fields_from_transaction(transaction);

    Transaction {
        block_date: timestamp.date.clone(),
        block_time: Some(timestamp.time.clone()),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_l1_da_mode: l1_da_mode_to_string(block.l1_da_mode),
        block_l1_data_gas_price_in_fri: l1_data_gas_price.price_in_fri.clone(),
        block_l1_data_gas_price_in_wei: l1_data_gas_price.price_in_wei.clone(),
        block_l1_gas_price_in_fri: l1_gas_price.price_in_fri.clone(),
        block_l1_gas_price_in_wei: l1_gas_price.price_in_wei.clone(),
        block_starknet_version: block.starknet_version.clone(),
        block_new_root: block_hashes.new_root.clone(),
        block_parent_hash: block_hashes.parent_hash.clone(),
        block_sequencer_address: block_hashes.sequencer_address.clone(),
        index: tx_index,
        hash: bytes_to_hex(&receipt.transaction_hash),
        fee_data_availability_mode: String::new(),
        max_fee: tx_data.max_fee,
        nonce: tx_data.nonce,
        nonce_data_availability_mode: tx_data.nonce_data_availability_mode,
        resource_bounds_l1_gas_max_amount: tx_data.resource_bounds_l1_gax_max_amount,
        resource_bounds_l1_gas_max_price_per_unit: tx_data.resource_bounds_l1_gas_max_price_per_unit,
        tip: tx_data.tip,
        r#type: tx_type_to_string(receipt.r#type),
        version: tx_data.version,
        actual_fee_amount: actual_fee.amount.clone(),
        actual_fee_unit: actual_fee.unit.clone(),
        execution_resources_bitwise_builtin_applications: execution_resources.bitwise_builtin_applications.clone(),
        // TODO: Decide if we use u64 or convert that u64 to a hex string representing u256
        execution_resources_data_availability_l1_gas: exec_data_availability.l1_gas.clone(),
        execution_resources_data_availability_l1_data_gas: exec_data_availability.l1_data_gas.clone(),

        execution_resources_ec_op_builtin_applications: execution_resources.ec_op_builtin_applications.clone(),
        execution_resources_ecdsa_builtin_applications: execution_resources.ecdsa_builtin_applications.clone(),
        execution_resources_keccak_builtin_applications: execution_resources.keccak_builtin_applications.clone(),
        execution_resources_memory_holes: execution_resources.memory_holes.clone(),
        execution_resources_pedersen_builtin_applications: execution_resources.pedersen_builtin_applications.clone(),
        execution_resources_poseidon_builtin_applications: execution_resources.poseidon_builtin_applications.clone(),
        execution_resources_range_check_builtin_applications: execution_resources.range_check_builtin_applications.clone(),
        execution_resources_segment_arena_builtin: execution_resources.segment_arena_builtin.clone(),
        execution_resources_steps: execution_resources.steps.clone(),
        execution_status: execution_status_to_string(receipt.execution_status),
        // TODO: Check where to get this
        finality_status: String::new(),
        // TODO: Check if there's a difference betwen receipt_type and r#type (Dune has both fields)
        receipt_type: tx_type_to_string(receipt.r#type),
        calldata: tx_data.calldata,
        class_hash: tx_data.class_hash,
        compiled_class_hash: tx_data.compiled_class_hash,
        constructor_calldata: tx_data.constructor_calldata,
        contract_address: bytes_to_hex(&receipt.contract_address),
        contract_address_salt: tx_data.contract_address_salt,
        entry_point_selector: tx_data.entry_point_selector,
        sender_address: tx_data.sender_address,
        signature: tx_data.signature,
        message_hash: receipt.message_hash.clone(),
        revert_reason: receipt.revert_reason.clone(),
    }
}

pub fn extract_fields_from_transaction(transaction: &TransactionWithReceipt) -> TransactionData {
    let mut data = TransactionData::default();

    if let Some(trx) = transaction.transaction.as_ref() {
        match trx {
            TrxType::InvokeTransactionV0(tx) => {
                data.max_fee = tx.max_fee.clone();
                data.version = tx.version.clone();
                data.calldata = u8_2d_vec_to_string_array(&tx.calldata);
                data.entry_point_selector = bytes_to_hex(&tx.entry_point_selector);
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
            }
            TrxType::InvokeTransactionV1(tx) => {
                data.max_fee = tx.max_fee.clone();
                data.version = tx.version.clone();
                data.nonce = tx.nonce.clone();
                data.calldata = u8_2d_vec_to_string_array(&tx.calldata);
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.sender_address = bytes_to_hex(&tx.sender_address);
            }
            TrxType::InvokeTransactionV3(tx) => {
                let l1_gas = tx.resource_bounds.as_ref().unwrap().l1_gas.as_ref().unwrap();
                data.version = tx.version.clone();
                data.nonce = tx.nonce.clone();
                data.resource_bounds_l1_gax_max_amount = bigint_string_to_hex(&l1_gas.max_amount);
                data.resource_bounds_l1_gas_max_price_per_unit = bigint_string_to_hex(&l1_gas.max_price_per_unit);
                data.calldata = u8_2d_vec_to_string_array(&tx.calldata);
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.sender_address = bytes_to_hex(&tx.sender_address);
                data.fee_data_availability_mode = data_availability_mode_to_string(tx.fee_data_availability_mode);
                data.nonce_data_availability_mode = data_availability_mode_to_string(tx.nonce_data_availability_mode);
                data.tip = tx.tip.clone();
            }
            TrxType::L1HandlerTransaction(tx) => {
                data.nonce = bigint_string_to_hex(&tx.nonce);
                data.version = tx.version.clone();
                data.calldata = u8_2d_vec_to_string_array(&tx.calldata);
                data.entry_point_selector = bytes_to_hex(&tx.entry_point_selector);
            }
            TrxType::DeclareTransactionV0(tx) => {
                data.max_fee = tx.max_fee.clone();
                data.version = tx.version.clone();
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.class_hash = bytes_to_hex(&tx.class_hash);
                data.sender_address = bytes_to_hex(&tx.sender_address);
            }
            TrxType::DeclareTransactionV1(tx) => {
                data.max_fee = tx.max_fee.clone();
                data.version = tx.version.clone();
                data.nonce = tx.nonce.clone();
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.class_hash = bytes_to_hex(&tx.class_hash);
                data.sender_address = bytes_to_hex(&tx.sender_address);
            }
            TrxType::DeclareTransactionV2(tx) => {
                data.max_fee = tx.max_fee.clone();
                data.version = tx.version.clone();
                data.nonce = tx.nonce.clone();
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.class_hash = bytes_to_hex(&tx.class_hash);
                data.compiled_class_hash = bytes_to_hex(&tx.compiled_class_hash);
                data.sender_address = bytes_to_hex(&tx.sender_address);
            }
            TrxType::DeclareTransactionV3(tx) => {
                let l1_gas = tx.resource_bounds.as_ref().unwrap().l1_gas.as_ref().unwrap();
                data.version = tx.version.clone();
                data.nonce = tx.nonce.clone();
                data.resource_bounds_l1_gax_max_amount = bigint_string_to_hex(&l1_gas.max_amount);
                data.resource_bounds_l1_gas_max_price_per_unit = bigint_string_to_hex(&l1_gas.max_price_per_unit);
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.class_hash = bytes_to_hex(&tx.class_hash);
                data.compiled_class_hash = bytes_to_hex(&tx.compiled_class_hash);
                data.sender_address = bytes_to_hex(&tx.sender_address);
                data.fee_data_availability_mode = data_availability_mode_to_string(tx.fee_data_availability_mode);
                data.nonce_data_availability_mode = data_availability_mode_to_string(tx.nonce_data_availability_mode);
                data.tip = tx.tip.clone();
            }
            TrxType::DeployTransactionV0(tx) => {
                data.version = tx.version.clone();
                data.constructor_calldata = u8_2d_vec_to_string_array(&tx.constructor_calldata);
                data.contract_address_salt = bytes_to_hex(&tx.contract_address_salt);
                data.class_hash = bytes_to_hex(&tx.class_hash);
            }
            TrxType::DeployAccountTransactionV1(tx) => {
                data.max_fee = tx.max_fee.clone();
                data.version = tx.version.clone();
                data.nonce = tx.nonce.clone();
                data.constructor_calldata = u8_2d_vec_to_string_array(&tx.constructor_calldata);
                data.contract_address_salt = bytes_to_hex(&tx.contract_address_salt);
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.class_hash = bytes_to_hex(&tx.class_hash);
            }
            TrxType::DeployAccountTransactionV3(tx) => {
                let l1_gas = tx.resource_bounds.as_ref().unwrap().l1_gas.as_ref().unwrap();
                data.version = tx.version.clone();
                data.nonce = tx.nonce.clone();
                data.resource_bounds_l1_gax_max_amount = bigint_string_to_hex(&l1_gas.max_amount);
                data.resource_bounds_l1_gas_max_price_per_unit = bigint_string_to_hex(&l1_gas.max_price_per_unit);
                data.constructor_calldata = u8_2d_vec_to_string_array(&tx.constructor_calldata);
                data.contract_address_salt = bytes_to_hex(&tx.contract_address_salt);
                data.signature = u8_2d_vec_to_string_array(&tx.signature);
                data.class_hash = bytes_to_hex(&tx.class_hash);
                data.fee_data_availability_mode = data_availability_mode_to_string(tx.fee_data_availability_mode);
                data.nonce_data_availability_mode = data_availability_mode_to_string(tx.nonce_data_availability_mode);
                data.tip = tx.tip.clone();
            }
        }
    }

    data
}

#[derive(Default)]
pub struct TransactionData {
    pub max_fee: Vec<u8>,
    pub version: String,
    pub nonce: Vec<u8>,
    pub resource_bounds_l1_gax_max_amount: Vec<u8>,
    pub resource_bounds_l1_gas_max_price_per_unit: Vec<u8>,
    pub fee_data_availability_mode: String,
    pub nonce_data_availability_mode: String,
    pub calldata: Vec<String>,
    pub constructor_calldata: Vec<String>,
    pub entry_point_selector: String,
    pub contract_address_salt: String,
    pub signature: Vec<String>,
    pub class_hash: String,
    pub compiled_class_hash: String,
    pub sender_address: String,
    pub tip: Vec<u8>,
}

pub fn bigint_string_to_hex(bigint: &str) -> Vec<u8> {
    bigint
        .chars()
        .filter_map(|c| c.to_digit(10)) // Convert character to a digit (base 10)
        .map(|d| d as u8) // Convert to u8
        .collect() // Collect into a Vec<u8>
}

fn data_availability_mode_to_string(value: i32) -> String {
    match value {
        1 => "L1".to_string(),
        2 => "L2".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn tx_type_to_string(value: i32) -> String {
    match value {
        1 => "Invoke",
        2 => "Declare",
        3 => "Deploy",
        4 => "Deploy account",
        5 => "L1 handler",
        _ => "Unknown",
    }
    .to_string()
}

fn execution_status_to_string(value: i32) -> String {
    match value {
        1 => "Success",
        2 => "Reverted",
        _ => "Unknown",
    }
    .to_string()
}
