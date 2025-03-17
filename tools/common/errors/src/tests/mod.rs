//! Tests for the common errors module
//!
//! This module contains comprehensive tests for the common errors module.

mod error_conversion_tests;
mod context_tests;
mod validation_tests;
mod reporting_tests;
mod category_tests;

// Re-export all test modules
pub use error_conversion_tests::*;
pub use context_tests::*;
pub use validation_tests::*;
pub use reporting_tests::*;
pub use category_tests::*;

// Re-export any utility functions if needed 