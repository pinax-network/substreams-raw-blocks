// Timestamp to date conversion
// ex: 2015-07-30T16:02:18Z => 2015-07-30
pub fn timestamp_to_date(timestamp: &str) -> &str {
    match timestamp.split('T').next() {
        Some(date) => date,
        None => "",
    }
}

pub fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut hex = String::new();
    hex.push_str("0x");
    for byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_date() {
        assert_eq!(timestamp_to_date("2015-07-30T16:02:18Z"), "2015-07-30");
        assert_eq!(timestamp_to_date("2020-01-01T00:00:00Z"), "2020-01-01");
        assert_eq!(timestamp_to_date("1999-12-31T23:59:59Z"), "1999-12-31");
        assert_eq!(timestamp_to_date("2000-02-29T12:34:56Z"), "2000-02-29");
    }

    #[test]
    fn test_invalid_timestamp() {
        assert_eq!(timestamp_to_date("invalid_timestamp"), "invalid_timestamp");
        assert_eq!(timestamp_to_date("2015-07-30 16:02:18"), "2015-07-30 16:02:18");
        assert_eq!(timestamp_to_date(""), "");
    }


    #[test]
    fn test_empty_vector() {
        let bytes = vec![];
        let expected = "0x";
        assert_eq!(bytes_to_hex(bytes), expected);
    }

    #[test]
    fn test_single_byte() {
        let bytes = vec![0x01];
        let expected = "0x01";
        assert_eq!(bytes_to_hex(bytes), expected);
    }

    #[test]
    fn test_multiple_bytes() {
        let bytes = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let expected = "0x0123456789abcdef";
        assert_eq!(bytes_to_hex(bytes), expected);
    }

    #[test]
    fn test_all_zeroes() {
        let bytes = vec![0x00, 0x00, 0x00];
        let expected = "0x000000";
        assert_eq!(bytes_to_hex(bytes), expected);
    }

    #[test]
    fn test_mixed_bytes() {
        let bytes = vec![0xff, 0x00, 0xab, 0x12];
        let expected = "0xff00ab12";
        assert_eq!(bytes_to_hex(bytes), expected);
    }

    #[test]
    fn test_large_bytes() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let expected = "0xdeadbeef";
        assert_eq!(bytes_to_hex(bytes), expected);
    }
}