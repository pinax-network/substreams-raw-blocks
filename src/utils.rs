// Timestamp to date conversion
// ex: 2015-07-30T16:02:18Z => 2015-07-30
pub fn block_time_to_date(block_time: &str) -> String {
    match block_time.split('T').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
    }
}

pub fn add_prefix_to_hex(hex: &str) -> String {
    if hex.is_empty() {
        return "".to_string();
    } else {
        format! {"0x{}", hex}.to_string()
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
}
