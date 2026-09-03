use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::{ConfirmedTransaction, Message, RewardType};

pub static VOTE_PROGRAM_ID: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

pub fn is_vote_transaction(message: &Message) -> bool {
    message.account_keys.iter().any(|key| key.as_slice() == VOTE_PROGRAM_ID)
}

// Solana hashes are base58 strings in Firehose; decode for the `bytes` columns.
pub fn base58_to_bytes(value: &str) -> Vec<u8> {
    bs58_decode(value).unwrap_or_else(|| value.as_bytes().to_vec())
}

fn bs58_decode(value: &str) -> Option<Vec<u8>> {
    const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut bytes: Vec<u8> = Vec::with_capacity(value.len());
    for c in value.bytes() {
        let mut carry = ALPHABET.iter().position(|&a| a == c)? as u32;
        for b in bytes.iter_mut().rev() {
            let v = *b as u32 * 58 + carry;
            *b = (v & 0xff) as u8;
            carry = v >> 8;
        }
        while carry > 0 {
            bytes.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    let leading = value.bytes().take_while(|&c| c == b'1').count();
    let mut out = vec![0u8; leading];
    out.extend(bytes);
    Some(out)
}

pub fn reward_type_text(value: i32) -> String {
    RewardType::try_from(value).map(|r| r.as_str_name().to_string()).unwrap_or_else(|_| "UNKNOWN".to_string())
}

// All account keys a transaction can address: static keys followed by the
// addresses loaded through address lookup tables (writable first, then readonly).
pub fn account_keys_extended(tx: &ConfirmedTransaction) -> Vec<Vec<u8>> {
    let message = tx.transaction.as_ref().and_then(|t| t.message.as_ref());
    let meta = tx.meta.as_ref();
    let mut keys: Vec<Vec<u8>> = message.map(|m| m.account_keys.clone()).unwrap_or_default();
    if let Some(meta) = meta {
        keys.extend(meta.loaded_writable_addresses.iter().cloned());
        keys.extend(meta.loaded_readonly_addresses.iter().cloned());
    }
    keys
}

pub fn non_empty_bytes(value: &[u8]) -> Option<Vec<u8>> {
    (!value.is_empty()).then(|| value.to_vec())
}

pub fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_base58() {
        assert_eq!(base58_to_bytes("Vote111111111111111111111111111111111111111"), VOTE_PROGRAM_ID.to_vec());
        assert_eq!(base58_to_bytes("11111111111111111111111111111111"), vec![0u8; 32]);
    }
}
