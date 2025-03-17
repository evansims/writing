use crate::{Result, WritingError};
use std::path::{Path, PathBuf};
use std::io;

/// Enhanced error context information
///
/// This struct provides detailed context for errors, including the operation
/// that failed, the file path involved, and additional details.
///
/// # Example
///
/// ```rust
/// use common_errors::{ErrorContext, IoResultExt};
/// use std::fs;
/// use std::path::Path;
///
/// fn read_config_file(path: &Path) -> common_errors::Result<String> {
///     fs::read_to_string(path)
///         .with_enhanced_context(|| {
///             ErrorContext::new("read_config_file")
///                 .with_file(path)
///                 .with_details("Configuration file could not be read")
///         })
/// }
/// ```
#[derive(Debug)]
pub struct ErrorContext {
    /// The name of the operation that failed
    pub operation: String,
    /// Optional path to the file involved in the operation
    pub file_path: Option<PathBuf>,
    /// Optional additional details about the error
    pub details: Option<String>,
}

impl ErrorContext {
    /// Create a new error context with operation name
    ///
    /// # Parameters
    ///
    /// * `operation` - The name of the operation that failed
    ///
    /// # Returns
    ///
    /// A new ErrorContext with the specified operation name
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorContext;
    ///
    /// let context = ErrorContext::new("read_file");
    /// ```
    pub fn new<S: AsRef<str>>(operation: S) -> Self {
        ErrorContext {
            operation: operation.as_ref().to_string(),
            file_path: None,
            details: None,
        }
    }
    
    /// Add a file path to the context
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the file involved in the operation
    ///
    /// # Returns
    ///
    /// The ErrorContext with the file path added
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorContext;
    /// use std::path::Path;
    ///
    /// let context = ErrorContext::new("read_file")
    ///     .with_file(Path::new("config.yaml"));
    /// ```
    pub fn with_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.file_path = Some(path.as_ref().to_path_buf());
        self
    }
    
    /// Add details to the context
    ///
    /// # Parameters
    ///
    /// * `details` - Additional details about the error
    ///
    /// # Returns
    ///
    /// The ErrorContext with the details added
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorContext;
    ///
    /// let context = ErrorContext::new("read_file")
    ///     .with_details("File could not be read due to permissions");
    /// ```
    pub fn with_details<S: AsRef<str>>(mut self, details: S) -> Self {
        self.details = Some(details.as_ref().to_string());
        self
    }
    
    /// Format the context into a string message
    ///
    /// # Returns
    ///
    /// A formatted string containing the operation name, file path (if any),
    /// and details (if any)
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::ErrorContext;
    /// use std::path::Path;
    ///
    /// let context = ErrorContext::new("read_file")
    ///     .with_file(Path::new("config.yaml"))
    ///     .with_details("File could not be read");
    ///
    /// let message = context.format();
    /// // message will be something like:
    /// // "Operation 'read_file' failed: path 'config.yaml': File could not be read"
    /// ```
    pub fn format(&self) -> String {
        let mut msg = format!("Operation '{}' failed", self.operation);
        
        if let Some(path) = &self.file_path {
            msg.push_str(&format!(": path '{}'", path.display()));
        }
        
        if let Some(details) = &self.details {
            msg.push_str(&format!(": {}", details));
        }
        
        msg
    }
}

/// Extension trait for IO error results to add enhanced context
///
/// This trait extends IO error results with methods to add enhanced context
/// information, making it easier to create detailed error messages.
///
/// # Example
///
/// ```rust
/// use common_errors::{ErrorContext, IoResultExt};
/// use std::fs;
/// use std::path::Path;
///
/// fn read_file(path: &Path) -> common_errors::Result<String> {
///     fs::read_to_string(path)
///         .with_enhanced_context(|| {
///             ErrorContext::new("read_file")
///                 .with_file(path)
///         })
/// }
/// ```
pub trait IoResultExt<T> {
    /// Add enhanced context to an IO error
    ///
    /// # Parameters
    ///
    /// * `f` - Function that returns an ErrorContext
    ///
    /// # Returns
    ///
    /// A Result with the original value or a contextualized error
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::{ErrorContext, IoResultExt};
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// fn read_file(path: &Path) -> common_errors::Result<String> {
    ///     fs::read_to_string(path)
    ///         .with_enhanced_context(|| {
    ///             ErrorContext::new("read_file")
    ///                 .with_file(path)
    ///         })
    /// }
    /// ```
    fn with_enhanced_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> ErrorContext;
}

// Implementation for IO errors, with special handling
impl<T> IoResultExt<T> for std::result::Result<T, io::Error> {
    fn with_enhanced_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> ErrorContext,
    {
        self.map_err(|e| {
            let context = f();
            
            // Map IO errors to specific WritingError types
            match e.kind() {
                io::ErrorKind::NotFound => {
                    if let Some(path) = &context.file_path {
                        WritingError::file_not_found(path)
                    } else {
                        WritingError::content_not_found(context.format())
                    }
                },
                io::ErrorKind::PermissionDenied => {
                    if let Some(path) = &context.file_path {
                        WritingError::permission_denied(path)
                    } else {
                        WritingError::other(context.format())
                    }
                },
                _ => WritingError::other(format!("{}: {}", context.format(), e)),
            }
        })
    }
} 