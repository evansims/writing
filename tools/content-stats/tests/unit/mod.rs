//! Unit tests for content-stats
//!
//! This module contains unit tests for the content-stats tool.

pub mod stats_options_tests;
pub mod calculate_stats_tests;
pub mod generate_stats_tests;
pub mod date_format_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}