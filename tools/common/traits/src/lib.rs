//! # Common Trait Implementations
//!
//! This module provides shared trait implementations for common behaviors
//! across the codebase.

use std::fmt;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use common_errors::{Result, WritingError};

/// Trait for file operations
///
/// This trait provides common file operations for types that need
/// to read from or write to files.
pub trait FileIO {
    /// Read the content from a file
    fn read_from_file(&mut self, path: &Path) -> Result<()>;

    /// Write the content to a file
    fn write_to_file(&self, path: &Path) -> Result<()>;

    /// Check if the content exists in a file
    fn exists_in_file(&self, path: &Path) -> Result<bool>;
}

/// Trait for configuration loading
///
/// This trait provides a common interface for types that need to
/// load configuration from various sources.
pub trait ConfigLoading {
    /// Load configuration from a YAML file
    fn load_from_yaml(&mut self, path: &Path) -> Result<()>;

    /// Load configuration from a JSON file
    fn load_from_json(&mut self, path: &Path) -> Result<()>;

    /// Load configuration from environment variables
    fn load_from_env(&mut self) -> Result<()>;
}

/// Trait for content processing
///
/// This trait provides a common interface for types that need to
/// process content in various formats.
pub trait ContentProcessing {
    /// Process markdown content
    fn process_markdown(&self, content: &str) -> Result<String>;

    /// Extract frontmatter from content
    fn extract_frontmatter(&self, content: &str) -> Result<serde_yaml::Value>;

    /// Generate HTML from markdown
    fn generate_html(&self, markdown: &str) -> Result<String>;
}

/// Trait for validation operations
///
/// This trait provides a common interface for types that need to
/// validate their state or configuration.
pub trait Validation {
    /// Validate the current state
    fn validate(&self) -> Result<()>;

    /// Check if the current state is valid
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Get validation errors
    fn validation_errors(&self) -> Vec<String>;
}

/// Trait for error conversion
///
/// This trait provides common error conversion functions for types
/// that need to convert between different error types.
pub trait ErrorConversion<E> {
    /// Convert from an external error type to a WritingError
    fn to_writing_error(error: E, context: &str) -> WritingError;

    /// Convert from an external error type to a Result
    fn to_result<T>(result: std::result::Result<T, E>, context: &str) -> Result<T>;
}

/// Implementation of ErrorConversion for IO errors
impl ErrorConversion<io::Error> for io::Error {
    fn to_writing_error(error: io::Error, context: &str) -> WritingError {
        WritingError::io_error(&format!("{}: {}", context, error))
    }

    fn to_result<T>(result: std::result::Result<T, io::Error>, context: &str) -> Result<T> {
        result.map_err(|e| Self::to_writing_error(e, context))
    }
}

/// Trait for progress reporting
///
/// This trait provides a common interface for types that need to
/// report progress during long-running operations.
pub trait ProgressReporting {
    /// Start a new progress operation
    fn start_progress(&self, operation: &str, total: usize);

    /// Update the progress
    fn update_progress(&self, completed: usize, message: Option<&str>);

    /// Complete the progress operation
    fn complete_progress(&self);

    /// Report a failed progress operation
    fn fail_progress(&self, error: &str);
}

/// Trait for serialization operations
///
/// This trait provides a common interface for types that need to
/// serialize and deserialize to/from various formats.
pub trait SerializationOps {
    /// Serialize to JSON
    fn to_json(&self) -> Result<String> where Self: Serialize;

    /// Deserialize from JSON
    fn from_json(json: &str) -> Result<Self> where Self: Sized + for<'de> Deserialize<'de>;

    /// Serialize to YAML
    fn to_yaml(&self) -> Result<String> where Self: Serialize;

    /// Deserialize from YAML
    fn from_yaml(yaml: &str) -> Result<Self> where Self: Sized + for<'de> Deserialize<'de>;
}

/// Implementation of SerializationOps for any type that implements Serialize and Deserialize
impl<T> SerializationOps for T
where
    T: Serialize + for<'de> Deserialize<'de>
{
    fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| WritingError::format_error(&format!("Failed to serialize to JSON: {}", e)))
    }

    fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| WritingError::format_error(&format!("Failed to deserialize from JSON: {}", e)))
    }

    fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .map_err(|e| WritingError::format_error(&format!("Failed to serialize to YAML: {}", e)))
    }

    fn from_yaml(yaml: &str) -> Result<Self> {
        serde_yaml::from_str(yaml)
            .map_err(|e| WritingError::format_error(&format!("Failed to deserialize from YAML: {}", e)))
    }
}

/// Trait for temporary file operations
///
/// This trait provides a common interface for types that need to
/// work with temporary files.
pub trait TempFileOps {
    /// Create a temporary file with the given content
    fn create_temp_file(&self, content: &str) -> Result<PathBuf>;

    /// Create a temporary directory
    fn create_temp_dir(&self) -> Result<PathBuf>;

    /// Clean up temporary files and directories
    fn cleanup_temp(&self) -> Result<()>;
}

/// Default implementation of TempFileOps
impl TempFileOps for PathBuf {
    fn create_temp_file(&self, content: &str) -> Result<PathBuf> {
        let temp_dir = tempfile::Builder::new()
            .prefix("writing-")
            .tempdir()
            .map_err(|e| WritingError::io_error(&format!("Failed to create temp dir: {}", e)))?;

        let file_path = temp_dir.path().join(self.file_name().unwrap_or_default());

        std::fs::write(&file_path, content)
            .map_err(|e| WritingError::io_error(&format!("Failed to write temp file: {}", e)))?;

        Ok(file_path)
    }

    fn create_temp_dir(&self) -> Result<PathBuf> {
        let temp_dir = tempfile::Builder::new()
            .prefix("writing-")
            .tempdir()
            .map_err(|e| WritingError::io_error(&format!("Failed to create temp dir: {}", e)))?;

        Ok(temp_dir.path().to_path_buf())
    }

    fn cleanup_temp(&self) -> Result<()> {
        if self.starts_with(std::env::temp_dir()) && self.exists() {
            if self.is_dir() {
                std::fs::remove_dir_all(self)
                    .map_err(|e| WritingError::io_error(&format!("Failed to remove temp dir: {}", e)))?;
            } else {
                std::fs::remove_file(self)
                    .map_err(|e| WritingError::io_error(&format!("Failed to remove temp file: {}", e)))?;
            }
        }

        Ok(())
    }
}

/// Module with trait implementations for content utilities
pub mod content {
    use super::*;
    use common_errors::Result;

    /// Trait for content metadata extraction
    pub trait MetadataExtraction {
        /// Extract word count from content
        fn word_count(&self, content: &str) -> usize;

        /// Extract reading time from content (in minutes)
        fn reading_time(&self, content: &str) -> usize;

        /// Extract character count from content
        fn character_count(&self, content: &str) -> usize;

        /// Extract paragraph count from content
        fn paragraph_count(&self, content: &str) -> usize;

        /// Extract sentence count from content
        fn sentence_count(&self, content: &str) -> usize;
    }

    /// Default implementation of MetadataExtraction
    pub struct ContentMetadataExtractor;

    impl MetadataExtraction for ContentMetadataExtractor {
        fn word_count(&self, content: &str) -> usize {
            content
                .split_whitespace()
                .filter(|word| !word.is_empty())
                .count()
        }

        fn reading_time(&self, content: &str) -> usize {
            let words = self.word_count(content);
            let minutes = (words as f64 / 200.0).ceil() as usize;
            std::cmp::max(1, minutes)
        }

        fn character_count(&self, content: &str) -> usize {
            content.chars().count()
        }

        fn paragraph_count(&self, content: &str) -> usize {
            content
                .split("\n\n")
                .filter(|p| !p.trim().is_empty())
                .count()
        }

        fn sentence_count(&self, content: &str) -> usize {
            let sentence_endings = [".", "!", "?"];

            content
                .split(|c| sentence_endings.contains(&c.to_string().as_str()))
                .filter(|s| !s.trim().is_empty())
                .count()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_ops() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestStruct {
            name: String,
            value: i32,
        }

        let test = TestStruct {
            name: "Test".to_string(),
            value: 42,
        };

        // Test JSON serialization and deserialization
        let json = test.to_json().unwrap();
        let deserialized = TestStruct::from_json(&json).unwrap();
        assert_eq!(test, deserialized);

        // Test YAML serialization and deserialization
        let yaml = test.to_yaml().unwrap();
        let deserialized = TestStruct::from_yaml(&yaml).unwrap();
        assert_eq!(test, deserialized);
    }

    #[test]
    fn test_content_metadata() {
        use content::{ContentMetadataExtractor, MetadataExtraction};

        let extractor = ContentMetadataExtractor;
        let content = "This is a test paragraph.\n\nThis is another paragraph with some more words!";

        assert_eq!(extractor.word_count(content), 12);
        assert_eq!(extractor.paragraph_count(content), 2);
        assert_eq!(extractor.sentence_count(content), 2);
        assert_eq!(extractor.character_count(content), 65);
        assert_eq!(extractor.reading_time(content), 1);

        let long_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(40);
        assert_eq!(extractor.reading_time(&long_content), 2);
    }
}