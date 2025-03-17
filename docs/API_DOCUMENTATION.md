# API Documentation Standards

This document outlines the standards for documenting public APIs in the Writing project.

## Overview

Consistent documentation is essential for maintainability and usability of the codebase. All public APIs in the Writing project should follow these documentation standards.

## Documentation Format

### Module Documentation

Each module should have a top-level documentation comment that includes:

1. **Module Purpose**: A brief description of what the module does
2. **Features**: A list of key features provided by the module
3. **Example**: A simple example of how to use the module

Example:

```rust
//! # Config Module
//! 
//! This module provides functionality for loading and managing configuration.
//! 
//! ## Features
//! 
//! - Loading configuration from YAML files
//! - Validating configuration values
//! - Accessing configuration through a typed interface
//! 
//! ## Example
//! 
//! ```rust
//! use common_config::Config;
//! 
//! let config = Config::load("config.yaml").unwrap();
//! let author = config.publication.author;
//! ```
```

### Struct/Enum Documentation

Each struct or enum should have a documentation comment that includes:

1. **Purpose**: A brief description of what the struct/enum represents
2. **Fields/Variants**: Description of important fields or variants
3. **Example**: A simple example of how to use the struct/enum (if applicable)

Example:

```rust
/// Configuration for content settings
///
/// This struct contains all configuration related to content,
/// including topics, tags, and base directory.
///
/// # Example
///
/// ```rust
/// use common_models::ContentConfig;
///
/// let content_config = ContentConfig {
///     base_dir: "/content".to_string(),
///     topics: HashMap::new(),
///     tags: None,
/// };
/// ```
pub struct ContentConfig {
    /// Base directory for content files
    pub base_dir: String,
    /// Map of topic IDs to topic configurations
    pub topics: HashMap<String, TopicConfig>,
    /// Optional map of tag categories to tags
    pub tags: Option<HashMap<String, Vec<String>>>,
}
```

### Function/Method Documentation

Each function or method should have a documentation comment that includes:

1. **Purpose**: A brief description of what the function does
2. **Parameters**: Description of each parameter
3. **Returns**: Description of the return value
4. **Errors**: Description of possible errors (if applicable)
5. **Example**: A simple example of how to use the function (if applicable)

Example:

```rust
/// Loads configuration from a file
///
/// # Parameters
///
/// * `path` - Path to the configuration file
///
/// # Returns
///
/// Returns the loaded configuration if successful
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed
///
/// # Example
///
/// ```rust
/// use common_config::load_config;
///
/// let config = load_config("config.yaml").unwrap();
/// ```
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    // Implementation
}
```

### Trait Documentation

Each trait should have a documentation comment that includes:

1. **Purpose**: A brief description of what the trait represents
2. **Methods**: Description of required and provided methods
3. **Example**: A simple example of how to implement or use the trait

Example:

```rust
/// Trait for filesystem operations
///
/// This trait defines the interface for filesystem operations
/// that can be implemented by real or mock filesystems.
///
/// # Example
///
/// ```rust
/// use common_fs::FileSystem;
///
/// struct MyFileSystem;
///
/// impl FileSystem for MyFileSystem {
///     fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
///         // Implementation
///     }
///     
///     // Other required methods
/// }
/// ```
pub trait FileSystem {
    /// Check if a file exists
    fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool;
    
    /// Read a file
    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<String>;
    
    /// Write a file
    fn write_file<P: AsRef<Path>, C: Into<String>>(&mut self, path: P, content: C) -> Result<()>;
    
    /// Delete a file
    fn delete_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
}
```

## Documentation Best Practices

1. **Be Concise**: Keep documentation clear and to the point
2. **Use Complete Sentences**: Start sentences with capital letters and end with periods
3. **Use Code Examples**: Provide examples for complex functionality
4. **Document Errors**: Clearly document possible error conditions
5. **Keep Examples Simple**: Examples should be simple and focused on the functionality being documented
6. **Use Markdown**: Use Markdown formatting for better readability
7. **Document Public APIs**: All public items should be documented
8. **Update Documentation**: Keep documentation up to date with code changes

## Documentation Tools

- Use `cargo doc` to generate documentation
- Use `cargo doc --open` to view the generated documentation
- Use `#[doc(hidden)]` to hide items from documentation
- Use `#[deprecated]` to mark deprecated items

## Example

See the `common/models` crate for examples of well-documented APIs. 