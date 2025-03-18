//! Error types for the content-edit module.
//!
//! This file defines the error types used by the content-edit module.

use std::fmt;
use std::error::Error;
use std::path::PathBuf;
use common_errors::WritingError;

/// Errors that can occur in the content-edit module.
#[derive(Debug)]
pub enum ContentEditError {
    /// Content was not found
    ContentNotFound {
        /// The slug of the content that was not found
        slug: String,
        /// The topic where the content was expected, if specified
        topic: Option<String>,
    },

    /// Invalid content format
    InvalidFormat {
        /// The reason the format is invalid
        reason: String,
    },

    /// Error in the content path
    InvalidPath {
        /// The path that was invalid
        path: PathBuf,
        /// The reason the path is invalid
        reason: String,
    },

    /// Error accessing the file system
    FileSystem {
        /// The underlying IO error
        error: std::io::Error,
    },

    /// Error in configuration
    Configuration {
        /// The reason the configuration is invalid
        reason: String,
    },

    /// Error in validation
    Validation {
        /// The reason validation failed
        reason: String,
    },

    /// A generic error
    Other {
        /// The error message
        message: String,
    },
}

impl fmt::Display for ContentEditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentNotFound { slug, topic } => {
                if let Some(topic_name) = topic {
                    write!(f, "Content '{}' not found in topic '{}'", slug, topic_name)
                } else {
                    write!(f, "Content '{}' not found in any topic", slug)
                }
            },
            Self::InvalidFormat { reason } => {
                write!(f, "Invalid content format: {}", reason)
            },
            Self::InvalidPath { path, reason } => {
                write!(f, "Invalid path {}: {}", path.display(), reason)
            },
            Self::FileSystem { error } => {
                write!(f, "File system error: {}", error)
            },
            Self::Configuration { reason } => {
                write!(f, "Configuration error: {}", reason)
            },
            Self::Validation { reason } => {
                write!(f, "Validation error: {}", reason)
            },
            Self::Other { message } => {
                write!(f, "Error: {}", message)
            }
        }
    }
}

impl Error for ContentEditError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FileSystem { error } => Some(error),
            _ => None,
        }
    }
}

// Implement conversions from other error types

impl From<std::io::Error> for ContentEditError {
    fn from(error: std::io::Error) -> Self {
        Self::FileSystem { error }
    }
}

impl From<&str> for ContentEditError {
    fn from(message: &str) -> Self {
        Self::Other { message: message.to_string() }
    }
}

impl From<String> for ContentEditError {
    fn from(message: String) -> Self {
        Self::Other { message }
    }
}

impl From<ContentEditError> for WritingError {
    fn from(error: ContentEditError) -> Self {
        match error {
            ContentEditError::ContentNotFound { slug, topic } => {
                if let Some(topic_name) = topic {
                    WritingError::content_not_found(format!("Content '{}' not found in topic '{}'", slug, topic_name))
                } else {
                    WritingError::content_not_found(format!("Content '{}' not found in any topic", slug))
                }
            },
            ContentEditError::InvalidFormat { reason } => {
                WritingError::validation_error(format!("Invalid content format: {}", reason))
            },
            ContentEditError::InvalidPath { path, reason } => {
                WritingError::validation_error(format!("Invalid path {}: {}", path.display(), reason))
            },
            ContentEditError::FileSystem { error } => {
                WritingError::validation_error(format!("File system error: {}", error))
            },
            ContentEditError::Configuration { reason } => {
                WritingError::config_error(reason)
            },
            ContentEditError::Validation { reason } => {
                WritingError::validation_error(reason)
            },
            ContentEditError::Other { message } => {
                WritingError::other(message)
            }
        }
    }
}

impl From<WritingError> for ContentEditError {
    fn from(error: WritingError) -> Self {
        match error {
            WritingError::ContentNotFound(message) => {
                ContentEditError::ContentNotFound {
                    slug: message,
                    topic: None,
                }
            },
            WritingError::IoError(source) => {
                ContentEditError::FileSystem {
                    error: std::io::Error::new(std::io::ErrorKind::Other, source.to_string())
                }
            },
            WritingError::ConfigError(message) => {
                ContentEditError::Configuration { reason: message }
            },
            WritingError::ValidationError(message) => {
                ContentEditError::Validation { reason: message }
            },
            _ => ContentEditError::Other { message: error.to_string() }
        }
    }
}