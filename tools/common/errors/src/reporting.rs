use crate::{WritingError, ErrorCategory};
use colored::*;

/// Display style for error messages
pub enum ErrorDisplayStyle {
    /// Simple style (just the error message)
    Simple,
    /// Detailed style (error message with category, suggestion, and details)
    Detailed,
    /// Debug style (error message with debug information)
    Debug,
}

/// Error reporter for CLI tools
///
/// This struct provides methods to format and display error messages in a
/// user-friendly way.
///
/// # Example
///
/// ```rust
/// use common_errors::{ErrorReporter, WritingError, ErrorDisplayStyle};
///
/// let error = WritingError::file_not_found("config.yaml");
/// let reporter = ErrorReporter::new();
///
/// // Display a simple error message
/// let message = reporter.format_error(&error, ErrorDisplayStyle::Simple);
/// println!("{}", message);
///
/// // Display a detailed error message
/// let message = reporter.format_error(&error, ErrorDisplayStyle::Detailed);
/// println!("{}", message);
/// ```
#[derive(Debug)]
pub struct ErrorReporter {
    /// Whether to show error codes
    pub show_error_codes: bool,
    /// Whether to show suggestions
    pub show_suggestions: bool,
    /// Whether to show debug information
    pub show_debug: bool,
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self {
            show_error_codes: true,
            show_suggestions: true,
            show_debug: false,
        }
    }
}

impl ErrorReporter {
    /// Create a new error reporter with default settings
    ///
    /// # Returns
    ///
    /// A new ErrorReporter with default settings
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorReporter;
    ///
    /// let reporter = ErrorReporter::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new error reporter with custom settings
    ///
    /// # Parameters
    ///
    /// * `show_error_codes` - Whether to show error codes
    /// * `show_suggestions` - Whether to show suggestions
    /// * `show_debug` - Whether to show debug information
    ///
    /// # Returns
    ///
    /// A new ErrorReporter with the specified settings
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorReporter;
    ///
    /// let reporter = ErrorReporter::with_settings(true, true, false);
    /// ```
    pub fn with_settings(show_error_codes: bool, show_suggestions: bool, show_debug: bool) -> Self {
        Self {
            show_error_codes,
            show_suggestions,
            show_debug,
        }
    }
    
    /// Format an error message
    ///
    /// # Parameters
    ///
    /// * `error` - The error to format
    /// * `style` - The display style to use
    ///
    /// # Returns
    ///
    /// A formatted error message string
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::{ErrorReporter, WritingError, ErrorDisplayStyle};
    ///
    /// let error = WritingError::file_not_found("config.yaml");
    /// let reporter = ErrorReporter::new();
    ///
    /// let message = reporter.format_error(&error, ErrorDisplayStyle::Simple);
    /// ```
    pub fn format_error(&self, error: &WritingError, style: ErrorDisplayStyle) -> String {
        let category = ErrorCategory::from(error);
        
        match style {
            ErrorDisplayStyle::Simple => {
                format!("{}", error)
            },
            ErrorDisplayStyle::Detailed => {
                let mut output = String::new();
                
                // Add error header
                output.push_str(&format!("{}\n", "Error".red().bold()));
                
                // Add error message
                output.push_str(&format!("  {}\n", error.to_string()));
                
                // Add category
                output.push_str(&format!("  {}: {}\n", 
                    "Category".yellow(), 
                    format!("{:?}", category).cyan()
                ));
                
                // Add suggestion if enabled
                if self.show_suggestions {
                    output.push_str(&format!("  {}: {}\n", 
                        "Suggestion".yellow(), 
                        category.user_suggestion()
                    ));
                }
                
                output
            },
            ErrorDisplayStyle::Debug => {
                let mut output = String::new();
                
                // Add error header
                output.push_str(&format!("{}\n", "Error".red().bold()));
                
                // Add error message
                output.push_str(&format!("  {}\n", error.to_string()));
                
                // Add category
                output.push_str(&format!("  {}: {}\n", 
                    "Category".yellow(), 
                    format!("{:?}", category).cyan()
                ));
                
                // Add suggestion if enabled
                if self.show_suggestions {
                    output.push_str(&format!("  {}: {}\n", 
                        "Suggestion".yellow(), 
                        category.user_suggestion()
                    ));
                }
                
                // Add debug information
                output.push_str(&format!("  {}: {:?}\n", 
                    "Debug".yellow(), 
                    error
                ));
                
                output
            },
        }
    }
    
    /// Print an error message to stderr
    ///
    /// # Parameters
    ///
    /// * `error` - The error to print
    /// * `style` - The display style to use
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::{ErrorReporter, WritingError, ErrorDisplayStyle};
    ///
    /// let error = WritingError::file_not_found("config.yaml");
    /// let reporter = ErrorReporter::new();
    ///
    /// reporter.print_error(&error, ErrorDisplayStyle::Detailed);
    /// ```
    pub fn print_error(&self, error: &WritingError, style: ErrorDisplayStyle) {
        eprintln!("{}", self.format_error(error, style));
    }
    
    /// Print an error and exit the program
    ///
    /// # Parameters
    ///
    /// * `error` - The error to print
    /// * `style` - The display style to use
    /// * `exit_code` - The exit code to use
    ///
    /// # Example
    ///
    /// ```no_run
    /// use common_errors::{ErrorReporter, WritingError, ErrorDisplayStyle};
    ///
    /// let error = WritingError::file_not_found("config.yaml");
    /// let reporter = ErrorReporter::new();
    ///
    /// reporter.print_error_and_exit(&error, ErrorDisplayStyle::Detailed, 1);
    /// ```
    pub fn print_error_and_exit(&self, error: &WritingError, style: ErrorDisplayStyle, exit_code: i32) -> ! {
        self.print_error(error, style);
        std::process::exit(exit_code);
    }
}

/// Get a default error reporter
///
/// # Returns
///
/// A default ErrorReporter
///
/// # Example
///
/// ```rust
/// use common_errors::{get_default_reporter, WritingError, ErrorDisplayStyle};
///
/// let error = WritingError::file_not_found("config.yaml");
/// let reporter = get_default_reporter();
///
/// reporter.print_error(&error, ErrorDisplayStyle::Simple);
/// ```
pub fn get_default_reporter() -> ErrorReporter {
    ErrorReporter::default()
}

/// Print an error using the default reporter and simple style
///
/// # Parameters
///
/// * `error` - The error to print
///
/// # Example
///
/// ```rust
/// use common_errors::{print_error_simple, WritingError};
///
/// let error = WritingError::file_not_found("config.yaml");
/// print_error_simple(&error);
/// ```
pub fn print_error_simple(error: &WritingError) {
    let reporter = get_default_reporter();
    reporter.print_error(error, ErrorDisplayStyle::Simple);
}

/// Print an error using the default reporter and detailed style
///
/// # Parameters
///
/// * `error` - The error to print
///
/// # Example
///
/// ```rust
/// use common_errors::{print_error_detailed, WritingError};
///
/// let error = WritingError::file_not_found("config.yaml");
/// print_error_detailed(&error);
/// ```
pub fn print_error_detailed(error: &WritingError) {
    let reporter = get_default_reporter();
    reporter.print_error(error, ErrorDisplayStyle::Detailed);
}

/// Print an error using the default reporter and debug style
///
/// # Parameters
///
/// * `error` - The error to print
///
/// # Example
///
/// ```rust
/// use common_errors::{print_error_debug, WritingError};
///
/// let error = WritingError::file_not_found("config.yaml");
/// print_error_debug(&error);
/// ```
pub fn print_error_debug(error: &WritingError) {
    let reporter = get_default_reporter();
    reporter.print_error(error, ErrorDisplayStyle::Debug);
} 