//! Unit tests for date formatting functionality

use content_stats::format_date;

#[cfg(test)]
mod date_format_tests {
    use super::*;

    #[test]
    fn test_format_date_iso_date() {
        // Test formatting an ISO date (YYYY-MM-DD)
        let formatted = format_date("2023-01-15");
        assert_eq!(formatted, "January 15, 2023");
    }

    #[test]
    fn test_format_date_draft() {
        // Test formatting a DRAFT date
        let formatted = format_date("DRAFT");
        assert_eq!(formatted, "DRAFT");
    }

    #[test]
    fn test_format_date_with_time() {
        // Test formatting a date with time
        let formatted = format_date("2023-01-15T14:30:00");
        assert_eq!(formatted, "January 15, 2023");
    }

    #[test]
    fn test_format_date_invalid() {
        // Test formatting an invalid date
        let formatted = format_date("invalid-date");
        assert_eq!(formatted, "invalid-date");
    }

    #[test]
    fn test_format_date_empty() {
        // Test formatting an empty date
        let formatted = format_date("");
        assert_eq!(formatted, "");
    }

    #[test]
    fn test_format_date_future() {
        // Test formatting a future date
        let formatted = format_date("2050-12-31");
        assert_eq!(formatted, "December 31, 2050");
    }

    #[test]
    fn test_format_date_past() {
        // Test formatting a past date
        let formatted = format_date("1999-12-31");
        assert_eq!(formatted, "December 31, 1999");
    }
}