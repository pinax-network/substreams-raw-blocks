use sha2::{Digest, Sha256};
use substreams::Hex;
use substreams_cosmos::pb::EventAttribute;

// Should be included in Substreams Cosmos
pub fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return Hex::encode(tx_hash);
}

// Builds a string in the format of an array of tuples (key, value)
pub fn build_attributes_array_string(attributes: &[EventAttribute]) -> String {
    let tuples: Vec<(&str, &str)> = attributes.iter().map(|attr| (attr.key.as_str(), attr.value.as_str())).collect();
    serde_json::to_string(&tuples).unwrap_or_default()
}
