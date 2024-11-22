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

pub fn build_attributes_array(attributes: &[EventAttribute]) -> Vec<String> {
    attributes
        .iter()
        .map(|attr| format!("\"{key}\":\"{value}\"", key = attr.key, value = serde_json::to_string(&attr.value).unwrap_or_default()))
        .collect()
}
