// Timestamp to date conversion
// ex: 2015-07-30T16:02:18Z => 2015-07-30
pub fn block_time_to_date(block_time: &str) -> String {
    match block_time.split('T').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
    }
}

// Timestamp to date conversion
// ex: 2015-07-30 => 2015-07
pub fn block_date_to_month(block_date: &str) -> String {
    match block_date.split('-').take(2).collect::<Vec<&str>>().join("-").as_str() {
        date => date.to_string(),
    }
}

pub fn block_date_to_year(block_date: &str) -> String {
    match block_date.split('-').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
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
    fn test_block_date_to_month() {
        assert_eq!(block_date_to_month("2015-07-30"), "2015-07");
        assert_eq!(block_date_to_month("2020-01-01"), "2020-01");
        assert_eq!(block_date_to_month("1999-12-31"), "1999-12");
        assert_eq!(block_date_to_month("2000-02-29"), "2000-02");
    }

    #[test]
    fn test_block_date_to_year() {
        assert_eq!(block_date_to_year("2015-07-30"), "2015");
        assert_eq!(block_date_to_year("2020-01-01"), "2020");
        assert_eq!(block_date_to_year("1999-12-31"), "1999");
        assert_eq!(block_date_to_year("2000-02-29"), "2000");
    }

    #[test]
    fn test_invalid_timestamp() {
        assert_eq!(block_time_to_date("invalid_timestamp"), "invalid_timestamp");
        assert_eq!(block_time_to_date("2015-07-30 16:02:18"), "2015-07-30 16:02:18");
        assert_eq!(block_time_to_date(""), "");
    }
}
