//! Integration tests for the Write CLI
//!
//! This module contains integration tests that test how the Write CLI
//! interacts with its components and external tools.

// Include all test modules
mod build_tests;
mod content_tests;
mod cross_tool_tests;
mod error_handling_tests;
mod image_tests;
mod stats_tests;
mod topic_tests;
mod configuration_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn it_integrates() {
        assert!(true);
    }
}
