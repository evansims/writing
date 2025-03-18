use std::borrow::Cow;
use std::fmt;
use colored::Colorize;

use crate::{Result, WritingError, ErrorKind};

/// Formats errors with detailed context and user-friendly suggestions
pub struct ErrorFormatter {
    /// Whether to use colors in error output
    use_colors: bool,
    /// The verbosity level of error output
    verbosity: Verbosity,
    /// The indentation level for error details
    indent_level: usize,
    /// Whether to include suggestions in error output
    include_suggestions: bool,
}

/// The verbosity level of error output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Minimal error output (just the error message)
    Minimal,
    /// Standard error output (error message and context)
    Standard,
    /// Detailed error output (error message, context, and details)
    Detailed,
    /// Debug error output (all information, including debug information)
    Debug,
}

impl Default for ErrorFormatter {
    fn default() -> Self {
        Self {
            use_colors: true,
            verbosity: Verbosity::Standard,
            indent_level: 0,
            include_suggestions: true,
        }
    }
}

impl ErrorFormatter {
    /// Creates a new error formatter with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to use colors in error output
    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }

    /// Sets the verbosity level of error output
    pub fn with_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.verbosity = verbosity;
        self
    }

    /// Sets the indentation level for error details
    pub fn with_indent(mut self, indent_level: usize) -> Self {
        self.indent_level = indent_level;
        self
    }

    /// Sets whether to include suggestions in error output
    pub fn with_suggestions(mut self, include_suggestions: bool) -> Self {
        self.include_suggestions = include_suggestions;
        self
    }

    /// Formats an error with detailed context and user-friendly suggestions
    pub fn format<'a>(&self, error: &'a WritingError) -> Cow<'a, str> {
        match self.verbosity {
            Verbosity::Minimal => self.format_minimal(error),
            Verbosity::Standard => self.format_standard(error),
            Verbosity::Detailed => self.format_detailed(error),
            Verbosity::Debug => self.format_debug(error),
        }
    }

    /// Formats an error with minimal information
    fn format_minimal<'a>(&self, error: &'a WritingError) -> Cow<'a, str> {
        Cow::Borrowed(error.message())
    }

    /// Formats an error with standard information
    fn format_standard<'a>(&self, error: &'a WritingError) -> Cow<'a, str> {
        let mut output = String::new();
        let indent = " ".repeat(self.indent_level);

        // Add the error header
        let error_label = if self.use_colors {
            "Error:".red().bold().to_string()
        } else {
            "Error:".to_string()
        };
        output.push_str(&format!("{}{} {}\n", indent, error_label, error.message()));

        // Add the error context if available
        if let Some(context) = error.context() {
            let context_label = if self.use_colors {
                "Context:".yellow().to_string()
            } else {
                "Context:".to_string()
            };
            output.push_str(&format!("{}{} {}\n", indent, context_label, context));
        }

        // Add a suggested solution if available and suggestions are enabled
        if self.include_suggestions {
            if let Some(suggestion) = self.get_suggestion(error) {
                let suggestion_label = if self.use_colors {
                    "Suggestion:".green().to_string()
                } else {
                    "Suggestion:".to_string()
                };
                output.push_str(&format!("{}{} {}\n", indent, suggestion_label, suggestion));
            }
        }

        Cow::Owned(output)
    }

    /// Formats an error with detailed information
    fn format_detailed<'a>(&self, error: &'a WritingError) -> Cow<'a, str> {
        let mut output = String::new();
        let indent = " ".repeat(self.indent_level);

        // Add the error header with error kind
        let error_kind = self.format_error_kind(error.kind());
        let error_label = if self.use_colors {
            "Error:".red().bold().to_string()
        } else {
            "Error:".to_string()
        };
        output.push_str(&format!("{}{} [{}] {}\n", indent, error_label, error_kind, error.message()));

        // Add the error context if available
        if let Some(context) = error.context() {
            let context_label = if self.use_colors {
                "Context:".yellow().to_string()
            } else {
                "Context:".to_string()
            };
            output.push_str(&format!("{}{} {}\n", indent, context_label, context));
        }

        // Add error source chain
        let mut current_error = error.source();
        let mut depth = 0;

        if current_error.is_some() {
            let caused_by_label = if self.use_colors {
                "Caused by:".blue().to_string()
            } else {
                "Caused by:".to_string()
            };
            output.push_str(&format!("{}{}\n", indent, caused_by_label));
        }

        while let Some(source) = current_error {
            let source_indent = " ".repeat(self.indent_level + 2 + depth * 2);
            output.push_str(&format!("{}{}\n", source_indent, source));
            current_error = source.source();
            depth += 1;
        }

        // Add a suggested solution if available and suggestions are enabled
        if self.include_suggestions {
            if let Some(suggestion) = self.get_suggestion(error) {
                let suggestion_label = if self.use_colors {
                    "Suggestion:".green().to_string()
                } else {
                    "Suggestion:".to_string()
                };
                output.push_str(&format!("{}{} {}\n", indent, suggestion_label, suggestion));
            }
        }

        Cow::Owned(output)
    }

    /// Formats an error with debug information
    fn format_debug<'a>(&self, error: &'a WritingError) -> Cow<'a, str> {
        let mut output = String::new();
        let indent = " ".repeat(self.indent_level);

        // Add the error header with error kind
        let error_kind = self.format_error_kind(error.kind());
        let error_label = if self.use_colors {
            "Error:".red().bold().to_string()
        } else {
            "Error:".to_string()
        };
        output.push_str(&format!("{}{} [{}] {}\n", indent, error_label, error_kind, error.message()));

        // Add the error context if available
        if let Some(context) = error.context() {
            let context_label = if self.use_colors {
                "Context:".yellow().to_string()
            } else {
                "Context:".to_string()
            };
            output.push_str(&format!("{}{} {}\n", indent, context_label, context));
        }

        // Add error source chain
        let mut current_error = error.source();
        let mut depth = 0;

        if current_error.is_some() {
            let caused_by_label = if self.use_colors {
                "Caused by:".blue().to_string()
            } else {
                "Caused by:".to_string()
            };
            output.push_str(&format!("{}{}\n", indent, caused_by_label));
        }

        while let Some(source) = current_error {
            let source_indent = " ".repeat(self.indent_level + 2 + depth * 2);
            output.push_str(&format!("{}{}\n", source_indent, source));
            current_error = source.source();
            depth += 1;
        }

        // Add debug information
        let debug_label = if self.use_colors {
            "Debug Info:".magenta().to_string()
        } else {
            "Debug Info:".to_string()
        };
        output.push_str(&format!("{}{}\n", indent, debug_label));
        output.push_str(&format!("{}  Error kind: {:?}\n", indent, error.kind()));
        if let Some(backtrace) = error.backtrace() {
            output.push_str(&format!("{}  Backtrace:\n{}\n", indent, backtrace));
        }

        // Add a suggested solution if available and suggestions are enabled
        if self.include_suggestions {
            if let Some(suggestion) = self.get_suggestion(error) {
                let suggestion_label = if self.use_colors {
                    "Suggestion:".green().to_string()
                } else {
                    "Suggestion:".to_string()
                };
                output.push_str(&format!("{}{} {}\n", indent, suggestion_label, suggestion));
            }
        }

        Cow::Owned(output)
    }

    /// Gets a user-friendly suggestion for an error
    fn get_suggestion(&self, error: &WritingError) -> Option<&'static str> {
        match error.kind() {
            ErrorKind::IoError => match error.message() {
                message if message.contains("permission denied") => {
                    Some("Check file permissions and ensure you have the necessary access rights.")
                }
                message if message.contains("no such file or directory") => {
                    Some("Verify that the file path is correct and the file exists.")
                }
                message if message.contains("already exists") => {
                    Some("Use a different file name or remove the existing file.")
                }
                _ => Some("Check disk space, file permissions, and ensure paths are valid."),
            },
            ErrorKind::SerializationError => {
                Some("Verify that the data format is correct and matches the expected schema.")
            }
            ErrorKind::DeserializationError => {
                Some("Check the file format and ensure it matches the expected schema.")
            }
            ErrorKind::ValidationError => {
                Some("Review the input data and ensure it meets the required constraints.")
            }
            ErrorKind::ConfigurationError => {
                Some("Check your configuration file for errors and ensure it's properly formatted.")
            }
            ErrorKind::PluginError => {
                Some("Verify that the plugin is compatible with the current version of the application.")
            }
            ErrorKind::ExecutionError => {
                Some("Check that all required dependencies are installed and properly configured.")
            }
            ErrorKind::ParsingError => {
                Some("Review the syntax of the input file and ensure it's properly formatted.")
            }
            ErrorKind::NetworkError => {
                Some("Check your internet connection and ensure the target server is accessible.")
            }
            ErrorKind::TimeoutError => {
                Some("The operation timed out. Try again, or increase the timeout duration.")
            }
            ErrorKind::NotFoundError => {
                Some("Verify that the resource exists and that the path is correct.")
            }
            ErrorKind::InvalidInputError => {
                Some("Review the input data and ensure it meets the expected format and constraints.")
            }
            ErrorKind::UnauthorizedError => {
                Some("Check your credentials and ensure you have the necessary permissions.")
            }
            ErrorKind::LockError => {
                Some("Wait for other processes to release the lock, or force unlock if safe to do so.")
            }
            ErrorKind::UnsupportedOperationError => {
                Some("This operation is not supported in the current context or configuration.")
            }
            ErrorKind::UnknownError => {
                Some("An unexpected error occurred. Check the logs for more information.")
            }
        }
    }

    /// Formats an error kind as a user-friendly string
    fn format_error_kind(&self, kind: ErrorKind) -> &'static str {
        match kind {
            ErrorKind::IoError => "IO",
            ErrorKind::SerializationError => "Serialization",
            ErrorKind::DeserializationError => "Deserialization",
            ErrorKind::ValidationError => "Validation",
            ErrorKind::ConfigurationError => "Configuration",
            ErrorKind::PluginError => "Plugin",
            ErrorKind::ExecutionError => "Execution",
            ErrorKind::ParsingError => "Parsing",
            ErrorKind::NetworkError => "Network",
            ErrorKind::TimeoutError => "Timeout",
            ErrorKind::NotFoundError => "Not Found",
            ErrorKind::InvalidInputError => "Invalid Input",
            ErrorKind::UnauthorizedError => "Unauthorized",
            ErrorKind::LockError => "Lock",
            ErrorKind::UnsupportedOperationError => "Unsupported Operation",
            ErrorKind::UnknownError => "Unknown",
        }
    }
}

/// Extension trait to format WritingError with ErrorFormatter
pub trait ErrorFormatterExt {
    /// Formats an error with detailed context and user-friendly suggestions
    fn format(&self, formatter: &ErrorFormatter) -> Cow<'_, str>;

    /// Formats an error with default formatting settings
    fn format_default(&self) -> String;

    /// Formats an error for CLI output with colors
    fn format_cli(&self) -> String;

    /// Formats an error for logging (no colors)
    fn format_log(&self) -> String;

    /// Formats an error for debugging (detailed, with colors)
    fn format_debug(&self) -> String;
}

impl ErrorFormatterExt for WritingError {
    fn format(&self, formatter: &ErrorFormatter) -> Cow<'_, str> {
        formatter.format(self)
    }

    fn format_default(&self) -> String {
        self.format(&ErrorFormatter::default()).to_string()
    }

    fn format_cli(&self) -> String {
        self.format(&ErrorFormatter::new().with_colors(true)).to_string()
    }

    fn format_log(&self) -> String {
        self.format(&ErrorFormatter::new().with_colors(false)).to_string()
    }

    fn format_debug(&self) -> String {
        self.format(&ErrorFormatter::new()
            .with_colors(true)
            .with_verbosity(Verbosity::Debug)
        ).to_string()
    }
}

/// Display implementation for WritingError using standard formatting
impl fmt::Display for WritingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Use detailed formatting for alternate format specifier
            write!(f, "{}", self.format_debug())
        } else {
            // Use standard formatting
            write!(f, "{}", self.format_default())
        }
    }
}

/// A utility function to print an error to stderr with standard formatting
pub fn print_error(error: &WritingError) {
    eprintln!("{}", error.format_cli());
}

/// A utility function to print an error to stderr with detailed formatting
pub fn print_error_detailed(error: &WritingError) {
    eprintln!("{}", error.format(&ErrorFormatter::new()
        .with_colors(true)
        .with_verbosity(Verbosity::Detailed)
    ));
}

/// A utility function to print an error to stderr with debug formatting
pub fn print_error_debug(error: &WritingError) {
    eprintln!("{}", error.format_debug());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WritingError;

    #[test]
    fn test_minimal_formatting() {
        let error = WritingError::io_error(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"), "Failed to open config file");
        let formatter = ErrorFormatter::new().with_verbosity(Verbosity::Minimal);
        let formatted = error.format(&formatter);
        assert_eq!(formatted, "Failed to open config file");
    }

    #[test]
    fn test_standard_formatting() {
        let error = WritingError::io_error(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"), "Failed to open config file");
        let formatter = ErrorFormatter::new()
            .with_verbosity(Verbosity::Standard)
            .with_colors(false);
        let formatted = error.format(&formatter);
        assert!(formatted.contains("Error: Failed to open config file"));
        assert!(formatted.contains("Context: File not found"));
        assert!(formatted.contains("Suggestion:"));
    }

    #[test]
    fn test_detailed_formatting() {
        let error = WritingError::io_error(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"), "Failed to open config file");
        let formatter = ErrorFormatter::new()
            .with_verbosity(Verbosity::Detailed)
            .with_colors(false);
        let formatted = error.format(&formatter);
        assert!(formatted.contains("Error: [IO] Failed to open config file"));
        assert!(formatted.contains("Context: File not found"));
        assert!(formatted.contains("Caused by:"));
        assert!(formatted.contains("Suggestion:"));
    }

    #[test]
    fn test_without_suggestions() {
        let error = WritingError::io_error(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"), "Failed to open config file");
        let formatter = ErrorFormatter::new()
            .with_colors(false)
            .with_suggestions(false);
        let formatted = error.format(&formatter);
        assert!(!formatted.contains("Suggestion:"));
    }

    #[test]
    fn test_with_indentation() {
        let error = WritingError::io_error(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"), "Failed to open config file");
        let formatter = ErrorFormatter::new()
            .with_colors(false)
            .with_indent(4);
        let formatted = error.format(&formatter);
        assert!(formatted.starts_with("    Error:"));
    }
}