// Helper module for tests
// This module re-exports all the necessary items from the common-errors crate
// and provides utility functions for testing

// Re-export key types and traits
pub use common_errors::context::IoResultExt;
pub use common_errors::get_default_reporter;
pub use common_errors::print_error;
pub use common_errors::validation::OptionValidationExt;
pub use common_errors::ErrorCategory;
pub use common_errors::ErrorContext;
pub use common_errors::ErrorDisplayStyle;
pub use common_errors::ErrorFormatter;
pub use common_errors::ErrorFormatterExt;
pub use common_errors::ErrorReporter;
pub use common_errors::OptionExt;
pub use common_errors::Result;
pub use common_errors::ResultExt;
pub use common_errors::Verbosity;
pub use common_errors::WritingError;
pub use common_errors::*;
