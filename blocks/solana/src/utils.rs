use common::utils::{add_prefix_to_hex, block_time_to_date};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::TableChange;
use substreams_solana::{b58, base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

pub static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

pub fn insert_timestamp_without_number(row: &mut TableChange, clock: &Clock, is_block: bool, with_prefix: bool) {
    let timestamp = clock.clone().timestamp.unwrap();
    let block_date = block_time_to_date(timestamp.to_string().as_str());
    let seconds = timestamp.seconds;
    let nanos = timestamp.nanos;
    let milliseconds = seconds * 1000 + i64::from(nanos) / 1_000_000;
    let block_time = milliseconds.to_string();
    let block_hash = if with_prefix { add_prefix_to_hex(&clock.id) } else { clock.id.to_string() };
    let prefix = if is_block { "" } else { "block_" };

    row.change(format!("{}date", prefix).as_str(), ("", block_date.as_str()))
        .change(format!("{}time", prefix).as_str(), ("", block_time.as_str()))
        .change(format!("{}hash", prefix).as_str(), ("", block_hash.as_str()));
}

pub fn build_csv_string<T: ToString>(values: &[T]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(",")
}

pub fn encode_byte_vectors_to_base58_string(values: &[Vec<u8>]) -> String {
    let encoded_values: Vec<String> = values.iter().map(|value| format!("\"{}\"", base58::encode(value))).collect();
    format!("[{}]", encoded_values.join(","))
}

// Get all encoded account keys including loaded writable and readonly addresses
pub fn get_account_keys_extended(transaction: &ConfirmedTransaction) -> Vec<String> {
    let message = transaction.transaction.as_ref().unwrap().message.as_ref().unwrap();
    let meta = transaction.meta.as_ref().unwrap();

    message
        .account_keys
        .iter()
        .chain(&meta.loaded_writable_addresses)
        .chain(&meta.loaded_readonly_addresses)
        .map(base58::encode)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_byte_vectors_multiple() {
        let input = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 8, 7, 6]];
        let expected = "[\"2VfUX\",\"8SxqM\",\"EPavZ\"]";
        let output = encode_byte_vectors_to_base58_string(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_0x1234() {
        let input = vec![vec![0x12, 0x34]];
        let expected = "[\"2PM\"]";
        let output = encode_byte_vectors_to_base58_string(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_mixed_vectors() {
        let input = vec![vec![1, 2, 3, 4], vec![0x12, 0x34], vec![5, 6, 7, 8]];
        let expected = "[\"2VfUX\",\"2PM\",\"8SxqM\"]";
        let output = encode_byte_vectors_to_base58_string(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_empty_vector() {
        let input: Vec<Vec<u8>> = vec![];
        let expected = "[]";
        let output = encode_byte_vectors_to_base58_string(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_single_zero_byte() {
        let input = vec![vec![0]];
        let expected = "[\"1\"]";
        let output = encode_byte_vectors_to_base58_string(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_leading_zeros() {
        let input = vec![vec![0, 0, 1, 2, 3]];
        let expected = "[\"11Ldp\"]"; // '11' represents the two leading zeros
        let output = encode_byte_vectors_to_base58_string(&input);
        assert_eq!(output, expected);
    }
}
