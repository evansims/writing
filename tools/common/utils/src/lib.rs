//! # Common Utilities
//! 
//! This crate provides common utility functions used across all tools
//! in the writing tools suite.

pub mod process;
pub mod ui;
pub mod pattern;
pub mod time;

// Re-export frequently used utilities
pub use process::run_tool;
pub use ui::{print_success, print_error, print_info, print_warning};
pub use pattern::matches_pattern;
pub use time::{format_timestamp, parse_date};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_structure() {
        // Just a simple test to verify the module structure
        assert!(true);
    }
} 