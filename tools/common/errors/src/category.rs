use crate::WritingError;

/// Error category for user feedback
///
/// This enum categorizes errors to provide more user-friendly feedback.
/// Different categories can be handled differently in the UI.
///
/// # Example
///
/// ```rust
/// use common_errors::{WritingError, ErrorCategory};
///
/// let error = WritingError::file_not_found("config.yaml");
/// let category = ErrorCategory::from(&error);
///
/// match category {
///     ErrorCategory::NotFound => println!("File not found. Please check the path."),
///     ErrorCategory::Permission => println!("Permission denied. Please check your permissions."),
///     _ => println!("An error occurred: {}", error),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Configuration-related errors
    Configuration,
    /// Validation errors (e.g., invalid input)
    Validation,
    /// Resource not found errors
    NotFound,
    /// Permission-related errors
    Permission,
    /// Format errors (e.g., invalid YAML)
    Format,
    /// I/O errors
    Io,
    /// Command execution errors
    Command,
    /// Templating errors
    Template,
    /// Parsing errors
    Parsing,
    /// Unexpected errors
    Unexpected,
}

impl ErrorCategory {
    /// Get a user-friendly message for this error category
    ///
    /// # Returns
    ///
    /// A string with a user-friendly message for this error category
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorCategory;
    ///
    /// let category = ErrorCategory::NotFound;
    /// let message = category.user_message();
    /// // message will be something like:
    /// // "The requested resource was not found."
    /// ```
    pub fn user_message(&self) -> &'static str {
        match self {
            ErrorCategory::Configuration => "There is a problem with your configuration.",
            ErrorCategory::Validation => "The provided input is invalid.",
            ErrorCategory::NotFound => "The requested resource was not found.",
            ErrorCategory::Permission => "You don't have permission to access this resource.",
            ErrorCategory::Format => "The format of the input is invalid.",
            ErrorCategory::Io => "There was an I/O error.",
            ErrorCategory::Command => "The command execution failed.",
            ErrorCategory::Template => "There was an error processing the template.",
            ErrorCategory::Parsing => "There was an error parsing the content.",
            ErrorCategory::Unexpected => "An unexpected error occurred.",
        }
    }
    
    /// Get a user-friendly suggestion for this error category
    ///
    /// # Returns
    ///
    /// A string with a user-friendly suggestion for this error category
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorCategory;
    ///
    /// let category = ErrorCategory::NotFound;
    /// let suggestion = category.user_suggestion();
    /// // suggestion will be something like:
    /// // "Check if the file exists and the path is correct."
    /// ```
    pub fn user_suggestion(&self) -> &'static str {
        match self {
            ErrorCategory::Configuration => "Check your configuration file for errors.",
            ErrorCategory::Validation => "Make sure your input meets the requirements.",
            ErrorCategory::NotFound => "Check if the file exists and the path is correct.",
            ErrorCategory::Permission => "Check your file permissions or run with elevated privileges.",
            ErrorCategory::Format => "Check the format of your input and make sure it's valid.",
            ErrorCategory::Io => "Check disk space, file permissions, and make sure the file is not in use.",
            ErrorCategory::Command => "Check if the command exists and you have permission to run it.",
            ErrorCategory::Template => "Check your template syntax and make sure all variables are defined.",
            ErrorCategory::Parsing => "Check the syntax of your content and make sure it's valid.",
            ErrorCategory::Unexpected => "This is a bug. Please report it to the developers.",
        }
    }
}

impl From<&WritingError> for ErrorCategory {
    fn from(error: &WritingError) -> Self {
        match error {
            WritingError::ConfigError(_) => ErrorCategory::Configuration,
            WritingError::ContentNotFound(_) => ErrorCategory::NotFound,
            WritingError::TopicError(_) => ErrorCategory::NotFound,
            WritingError::IoError(_) => ErrorCategory::Io,
            WritingError::YamlError(_) => ErrorCategory::Format,
            WritingError::FormatError(_) => ErrorCategory::Format,
            WritingError::FileNotFound(_) => ErrorCategory::NotFound,
            WritingError::DirectoryNotFound(_) => ErrorCategory::NotFound,
            WritingError::ValidationError(_) => ErrorCategory::Validation,
            WritingError::PermissionDenied(_) => ErrorCategory::Permission,
            WritingError::ContentAlreadyExists(_) => ErrorCategory::Validation,
            WritingError::InvalidArgument(_) => ErrorCategory::Validation,
            WritingError::CommandError(_) => ErrorCategory::Command,
            WritingError::TemplateError(_) => ErrorCategory::Template,
            WritingError::ContentParsingError(_) => ErrorCategory::Parsing,
            WritingError::Other(_) => ErrorCategory::Unexpected,
        }
    }
} 