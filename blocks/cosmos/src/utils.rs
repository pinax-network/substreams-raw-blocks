use sha2::{Digest, Sha256};
use substreams::Hex;

// Should be included in Substreams Cosmos
pub fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return Hex::encode(tx_hash);
}
