use substreams::{scalar::BigDecimal, Hex};
use substreams_ethereum::pb::eth::v2::BigInt;

// Timestamp to date conversion
// ex: 2015-07-30T16:02:18Z => 2015-07-30
pub fn block_time_to_date(block_time: &str) -> String {
    match block_time.split('T').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
    }
}

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    if bytes.is_empty() {
        return "".to_string();
    } else {
        format! {"0x{}", Hex::encode(bytes)}.to_string()
    }
}

pub fn optional_bigint_to_string(value: Option<BigInt>, default: &str) -> String {
    match value {
        Some(bigint) => bigint.with_decimal(0).to_string(),
        None => default.to_string(),
    }
}

pub fn optional_bigint_to_decimal(value: Option<BigInt>) -> BigDecimal {
    match value {
        Some(bigint) => bigint.with_decimal(0),
        None => 0.into(),
    }
}

pub fn optional_bigint_to_hex(value: Option<BigInt>) -> String {
    match value {
        Some(bigint) => bytes_to_hex(&bigint.bytes),
        None => "".to_string(),
    }
}

pub fn optional_u64_to_string(value: Option<u64>, default: &str) -> String {
    match value {
        Some(uint) => uint.to_string(),
        None => default.to_string(),
    }
}

pub fn optional_uint64_to_string(value: Option<u64>) -> String {
    match value {
        Some(int) => int.to_string(),
        None => "0".to_string(),
    }
}

pub fn extract_topic(topics: &Vec<Vec<u8>>, index: usize) -> String {
    if index < topics.len() {
        bytes_to_hex(&topics[index])
    } else {
        "".to_string()
    }
}

// The Method ID for the function signature is the first 4 bytes (or the first 8 digits) of the Keccak-256 hash.
pub fn extract_method_id(data: &Vec<u8>) -> String {
    if data.len() >= 4 {
        bytes_to_hex(&data[..4].to_vec())
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_time_to_date() {
        assert_eq!(block_time_to_date("2015-07-30T16:02:18Z"), "2015-07-30");
        assert_eq!(block_time_to_date("2020-01-01T00:00:00Z"), "2020-01-01");
        assert_eq!(block_time_to_date("1999-12-31T23:59:59Z"), "1999-12-31");
        assert_eq!(block_time_to_date("2000-02-29T12:34:56Z"), "2000-02-29");
    }

    #[test]
    fn test_invalid_timestamp() {
        assert_eq!(block_time_to_date("invalid_timestamp"), "invalid_timestamp");
        assert_eq!(block_time_to_date("2015-07-30 16:02:18"), "2015-07-30 16:02:18");
        assert_eq!(block_time_to_date(""), "");
    }

    #[test]
    fn test_empty_vector() {
        let bytes = vec![];
        let expected = "";
        assert_eq!(bytes_to_hex(&bytes), expected);
    }

    #[test]
    fn test_single_byte() {
        let bytes = vec![0x01];
        let expected = "0x01";
        assert_eq!(bytes_to_hex(&bytes), expected);
    }

    #[test]
    fn test_multiple_bytes() {
        let bytes = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let expected = "0x0123456789abcdef";
        assert_eq!(bytes_to_hex(&bytes), expected);
    }

    #[test]
    fn test_all_zeroes() {
        let bytes = vec![0x00, 0x00, 0x00];
        let expected = "0x000000";
        assert_eq!(bytes_to_hex(&bytes), expected);
    }

    #[test]
    fn test_mixed_bytes() {
        let bytes = vec![0xff, 0x00, 0xab, 0x12];
        let expected = "0xff00ab12";
        assert_eq!(bytes_to_hex(&bytes), expected);
    }

    #[test]
    fn test_large_bytes() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let expected = "0xdeadbeef";
        assert_eq!(bytes_to_hex(&bytes), expected);
    }

    #[test]
    fn test_extract_topic_valid_index() {
        let topics = vec![vec![0x01, 0x02, 0x03], vec![0x0a, 0x0b, 0x0c], vec![0xff, 0xfe, 0xfd]];
        assert_eq!(extract_topic(&topics, 0), "0x010203");
        assert_eq!(extract_topic(&topics, 1), "0x0a0b0c");
        assert_eq!(extract_topic(&topics, 2), "0xfffefd");
    }

    #[test]
    fn test_extract_topic_invalid_index() {
        let topics = vec![vec![0x01, 0x02, 0x03], vec![0x0a, 0x0b, 0x0c]];
        assert_eq!(extract_topic(&topics, 3), "");
        assert_eq!(extract_topic(&topics, 100), "");
    }

    #[test]
    fn test_extract_topic_empty_vector() {
        let topics: Vec<Vec<u8>> = Vec::new();
        assert_eq!(extract_topic(&topics, 0), "");
        assert_eq!(extract_topic(&topics, 1), "");
    }

    #[test]
    fn test_extract_topic_single_element() {
        let topics = vec![vec![0x0d, 0x0e, 0x0f]];
        assert_eq!(extract_topic(&topics, 0), "0x0d0e0f");
        assert_eq!(extract_topic(&topics, 1), "");
    }

    #[test]
    fn test_extract_topic_large_numbers() {
        let topics = vec![vec![0xaa, 0xbb, 0xcc], vec![0xde, 0xad, 0xbe, 0xef]];
        assert_eq!(extract_topic(&topics, 0), "0xaabbcc");
        assert_eq!(extract_topic(&topics, 1), "0xdeadbeef");
    }
}
