# Module Structure Standards

This document outlines the standardized module structure for the Writing toolkit. Following these patterns helps maintain consistency, testability, and separation of concerns throughout the codebase.

## Overview

Each module in the codebase should follow a consistent structure with clear separation between:

- Public API
- Implementation details
- Unit tests

This approach improves readability, maintainability, and testability of the code.

## Standard Module Structure

Every module should follow this general structure:

```
module_name/
├── mod.rs                 # Public API and re-exports
├── impl.rs                # Implementation details
├── models.rs              # Data structures (if needed)
├── errors.rs              # Module-specific errors (if needed)
└── tests/                 # Unit tests
    ├── mod.rs             # Test module setup
    ├── impl_tests.rs      # Implementation tests
    └── integration_tests.rs  # Integration tests
```

### mod.rs

The `mod.rs` file serves as the public API for the module. It should:

- Re-export public types and functions
- Document the module's purpose and usage
- Avoid containing implementation details
- Keep implementation dependencies private

Example:

```rust
//! Module for managing content.
//!
//! This module provides functionality for creating, editing, and managing content.

mod impl;
mod models;
mod errors;

#[cfg(test)]
mod tests;

// Re-export public items
pub use errors::ContentError;
pub use models::{Content, ContentMetadata};

// Public API functions
pub use impl::{create_content, edit_content, delete_content, move_content};

// Only expose implementation details when necessary
#[doc(hidden)]
pub use impl::internal_function_needed_externally;
```

### impl.rs

The `impl.rs` file contains the implementation details of the module. It should:

- Implement the functionality exposed by the public API
- Keep implementation details private
- Follow a clear organization pattern with related functions grouped together

Example:

```rust
//! Implementation details for content management.

use super::errors::ContentError;
use super::models::{Content, ContentMetadata};
use common::fs::{FileOperations, PathUtils};
use common::validation::SlugValidator;
use std::path::Path;

/// Creates new content with the given parameters.
pub fn create_content(
    title: &str,
    topic: &str,
    slug: Option<&str>,
    tags: Option<&str>,
) -> Result<Content, ContentError> {
    // Implementation details...
}

/// Edits existing content.
pub fn edit_content(
    slug: &str,
    topic: &str,
    field: &str,
    value: &str,
) -> Result<Content, ContentError> {
    // Implementation details...
}

// More functions...
```

### models.rs

The `models.rs` file defines data structures used by the module. It should:

- Define clean, well-documented data structures
- Implement appropriate traits (Debug, Clone, etc.)
- Include validation logic or constructors if needed

Example:

```rust
//! Data models for content management.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a content item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    /// The unique slug identifier
    pub slug: String,
    /// The topic this content belongs to
    pub topic: String,
    /// Path to the content directory
    pub path: PathBuf,
    /// Metadata for the content
    pub metadata: ContentMetadata,
}

/// Metadata for a content item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    /// Title of the content
    pub title: String,
    /// Optional tagline for the content
    pub tagline: Option<String>,
    /// Tags associated with the content
    pub tags: Vec<String>,
    /// Publication date
    pub date: chrono::DateTime<chrono::Utc>,
    // More fields...
}

// Implementations, constructors, etc.
```

### errors.rs

The `errors.rs` file defines module-specific errors. It should:

- Define a module-specific error enum
- Implement `std::error::Error` and `Display` traits
- Provide conversion from underlying errors

Example:

```rust
//! Error types for content operations.

use std::fmt;
use thiserror::Error;

/// Errors that can occur during content operations.
#[derive(Debug, Error)]
pub enum ContentError {
    /// Content with the given slug was not found
    #[error("Content with slug '{0}' was not found")]
    NotFound(String),
    
    /// Topic does not exist
    #[error("Topic '{0}' does not exist")]
    TopicNotFound(String),
    
    /// Invalid slug format
    #[error("Invalid slug format: {0}")]
    InvalidSlug(String),
    
    /// File system error
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    /// Other error
    #[error("Error: {0}")]
    Other(String),
}
```

### tests/

The `tests/` directory contains unit and integration tests for the module. It should:

- Test both public API and internal functions
- Use mocks/fixtures where appropriate
- Cover both success and failure cases
- Follow the same organization as the code being tested

Example test file:

```rust
//! Tests for content management implementation.

use super::super::*;
use common_test_utils::TestFixture;
use std::path::Path;

#[test]
fn test_create_content_with_valid_parameters() {
    let fixture = TestFixture::new().unwrap();
    
    // Set up test environment
    fixture.create_topic("blog").unwrap();
    
    // Test function
    let result = create_content(
        "Test Article",
        "blog",
        None, // Generate slug from title
        Some("test,article"),
    );
    
    // Assertions
    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.slug, "test-article");
    assert_eq!(content.topic, "blog");
    assert!(Path::new(&fixture.path()).join("content/blog/test-article").exists());
}

#[test]
fn test_create_content_with_invalid_topic() {
    let fixture = TestFixture::new().unwrap();
    
    // Test function with invalid topic
    let result = create_content(
        "Test Article",
        "non-existent-topic",
        None,
        None,
    );
    
    // Assertions
    assert!(result.is_err());
    match result.unwrap_err() {
        ContentError::TopicNotFound(topic) => assert_eq!(topic, "non-existent-topic"),
        err => panic!("Unexpected error: {:?}", err),
    }
}
```

## Integration with Tools

Each tool command should follow this structure:

1. Parse command-line arguments
2. Call the appropriate module function
3. Handle errors and format output

Example:

```rust
// In tools/content-new/src/main.rs
fn main() -> Result<()> {
    let args = cli::parse_args();
    
    // Call module function
    let content = content::create_content(
        &args.title,
        &args.topic,
        args.slug.as_deref(),
        args.tags.as_deref(),
    )?;
    
    // Format output
    println!("Created content: {}", content.slug);
    
    Ok(())
}
```

## Conversion Process

When converting existing code to follow this structure:

1. Identify module boundaries
2. Split implementation details from public API
3. Move data structures to models.rs
4. Create appropriate error types
5. Ensure tests follow the same structure

## Benefits

Following this standardized module structure provides several benefits:

- **Maintainability**: Clear separation of concerns makes code easier to understand
- **Testability**: Well-defined interfaces are easier to test
- **Discoverability**: Consistent structure makes it easier to find code
- **Reusability**: Clean APIs make code more reusable
- **Documentation**: Public APIs are better documented 