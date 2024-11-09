use std::collections::HashMap;

pub fn blob_keys(block_hash: &str, index: u64) -> HashMap<String, String> {
    HashMap::from([("block_hash".to_string(), block_hash.to_string()), ("index".to_string(), index.to_string())])
}

pub fn deposit_keys(block_hash: &str, index: u64) -> HashMap<String, String> {
    HashMap::from([("block_hash".to_string(), block_hash.to_string()), ("index".to_string(), index.to_string())])
}

pub fn withdrawals_keys(block_hash: &str, index: u64) -> HashMap<String, String> {
    HashMap::from([("block_hash".to_string(), block_hash.to_string()), ("withdrawal_index".to_string(), index.to_string())])
}

pub fn attestation_keys(block_hash: &str, index: u64) -> HashMap<String, String> {
    HashMap::from([("block_hash".to_string(), block_hash.to_string()), ("index".to_string(), index.to_string())])
}

pub fn attester_slashing_keys(block_hash: &str, index: u64) -> HashMap<String, String> {
    HashMap::from([("block_hash".to_string(), block_hash.to_string()), ("index".to_string(), index.to_string())])
}
