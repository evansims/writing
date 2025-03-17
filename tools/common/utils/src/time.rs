//! # Time Utilities
//! 
//! This module provides utilities for working with dates and times.

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

/// Format a timestamp as a human-readable string
pub fn format_timestamp(timestamp: i64) -> String {
    let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
    let local_time = datetime.with_timezone(&Local);
    
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Format a timestamp as a human-readable relative time
pub fn format_relative_time(timestamp: i64) -> String {
    let now = Utc::now().timestamp();
    let diff = now - timestamp;
    
    match diff {
        d if d < 60 => format!("just now"),
        d if d < 3600 => format!("{} minutes ago", d / 60),
        d if d < 86400 => format!("{} hours ago", d / 3600),
        d if d < 604800 => format!("{} days ago", d / 86400),
        d if d < 2592000 => format!("{} weeks ago", d / 604800),
        d if d < 31536000 => format!("{} months ago", d / 2592000),
        d => format!("{} years ago", d / 31536000),
    }
}

/// Parse a date string in YYYY-MM-DD format
pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("Failed to parse date '{}': {}", date_str, e))
}

/// Parse a datetime string in various formats
pub fn parse_datetime(datetime_str: &str) -> Result<DateTime<Utc>> {
    // Try parsing with different formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d",
        "%d/%m/%Y %H:%M:%S",
        "%d/%m/%Y %H:%M",
        "%d/%m/%Y",
    ];
    
    for format in &formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, format) {
            return Ok(DateTime::from_utc(dt, Utc));
        }
        
        // For date-only formats, try to add a default time
        if format.ends_with("%Y") || format.ends_with("%d") {
            if let Ok(date) = NaiveDate::parse_from_str(datetime_str, format) {
                let default_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
                let datetime = NaiveDateTime::new(date, default_time);
                return Ok(DateTime::from_utc(datetime, Utc));
            }
        }
    }
    
    Err(anyhow::anyhow!("Failed to parse datetime: {}", datetime_str))
}

/// Get the current timestamp
pub fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}

/// Get the current date as a string in YYYY-MM-DD format
pub fn current_date_string() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

/// Get the current datetime as a string in YYYY-MM-DD HH:MM:SS format
pub fn current_datetime_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Calculate the difference between two dates in days
pub fn days_between(date1: &NaiveDate, date2: &NaiveDate) -> i64 {
    let duration = date2.signed_duration_since(*date1);
    duration.num_days()
}

/// Check if a date is in the future
pub fn is_future_date(date: &NaiveDate) -> bool {
    let today = Local::now().date_naive();
    date > &today
}

/// Check if a date is in the past
pub fn is_past_date(date: &NaiveDate) -> bool {
    let today = Local::now().date_naive();
    date < &today
}

/// Add days to a date
pub fn add_days(date: &NaiveDate, days: i64) -> NaiveDate {
    *date + chrono::Duration::days(days)
}

/// Get the start of the month for a given date
pub fn start_of_month(date: &NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()
}

/// Get the end of the month for a given date
pub fn end_of_month(date: &NaiveDate) -> NaiveDate {
    // Get the first day of the next month and subtract one day
    let next_month = if date.month() == 12 {
        NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).unwrap()
    };
    
    next_month.pred_opt().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_timestamp() {
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let formatted = format_timestamp(timestamp);
        
        // Since this depends on the local timezone, we just check if it contains the date
        assert!(formatted.contains("2021-01-01"));
    }
    
    #[test]
    fn test_parse_date() {
        let date_str = "2021-01-01";
        let date = parse_date(date_str).unwrap();
        
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }
    
    #[test]
    fn test_parse_datetime() {
        let datetime_str = "2021-01-01 12:30:45";
        let datetime = parse_datetime(datetime_str).unwrap();
        
        assert_eq!(datetime.naive_utc().date().year(), 2021);
        assert_eq!(datetime.naive_utc().date().month(), 1);
        assert_eq!(datetime.naive_utc().date().day(), 1);
        assert_eq!(datetime.naive_utc().time().hour(), 12);
        assert_eq!(datetime.naive_utc().time().minute(), 30);
        assert_eq!(datetime.naive_utc().time().second(), 45);
    }
    
    #[test]
    fn test_days_between() {
        let date1 = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2021, 1, 10).unwrap();
        
        assert_eq!(days_between(&date1, &date2), 9);
        assert_eq!(days_between(&date2, &date1), -9);
    }
    
    #[test]
    fn test_add_days() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let new_date = add_days(&date, 10);
        
        assert_eq!(new_date.year(), 2021);
        assert_eq!(new_date.month(), 1);
        assert_eq!(new_date.day(), 11);
    }
    
    #[test]
    fn test_start_and_end_of_month() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 15).unwrap();
        
        let start = start_of_month(&date);
        assert_eq!(start.year(), 2021);
        assert_eq!(start.month(), 1);
        assert_eq!(start.day(), 1);
        
        let end = end_of_month(&date);
        assert_eq!(end.year(), 2021);
        assert_eq!(end.month(), 1);
        assert_eq!(end.day(), 31);
    }
} 