# Macros for Code Reuse

This document describes the macros available in the Writing project, which have been created to reduce code duplication and promote consistent patterns across the codebase.

## Overview

Macros in Rust allow us to define reusable code patterns that get expanded at compile time. The Writing project uses macros to standardize common operations such as error handling, file operations, and trait implementations.

## Error Handling Macros

Error handling macros are provided by the `common_errors` crate and help standardize error context addition, propagation, and creation.

### `with_context!`

The `with_context!` macro simplifies adding context to errors, making them more informative and helpful.

```rust
use common_errors::{Result, with_context};
use std::fs;
use std::path::Path;

fn read_config(path: &Path) -> Result<String> {
    // Before:
    // fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))

    // After:
    with_context!(fs::read_to_string(path), "Failed to read config file: {}", path.display())
}
```

### `try_with_context!`

The `try_with_context!` macro combines matching on a `Result` with adding context to any error, streamlining the common pattern of checking a result and propagating errors with context.

```rust
use common_errors::{Result, try_with_context};
use std::fs;
use std::path::Path;

fn read_config(path: &Path) -> Result<String> {
    // Before:
    // let content = fs::read_to_string(path)
    //     .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    // After:
    let content = try_with_context!(fs::read_to_string(path), "Failed to read config file: {}", path.display());
    Ok(content)
}
```

### `error!`

The `error!` macro simplifies creating errors with the appropriate category, ensuring consistency in error creation throughout the codebase.

```rust
use common_errors::{Result, error};

fn validate_input(input: &str) -> Result<()> {
    // Before:
    // if input.is_empty() {
    //     return Err(WritingError::validation_error("Input cannot be empty"));
    // }

    // After:
    if input.is_empty() {
        return Err(error!(validation, "Input cannot be empty"));
    }
    Ok(())
}
```

## File System Macros

File system macros are provided by the `common_fs` crate and standardize file and directory operations across the codebase.

### `read_file!`

The `read_file!` macro simplifies reading files with proper error handling.

```rust
use common_errors::Result;
use common_fs::read_file;
use std::path::Path;

fn read_config(path: &Path) -> Result<String> {
    // Before:
    // let content = fs::read_to_string(path)
    //     .with_context(|| format!("Failed to read file: {}", path.display()))?;

    // After:
    let content = read_file!(path);
    Ok(content)
}
```

### `write_file!`

The `write_file!` macro simplifies writing content to files with proper error handling.

```rust
use common_errors::Result;
use common_fs::write_file;
use std::path::Path;

fn write_config(path: &Path, content: &str) -> Result<()> {
    // Before:
    // fs::write(path, content)
    //     .with_context(|| format!("Failed to write file: {}", path.display()))?;

    // After:
    write_file!(path, content);
    Ok(())
}
```

### `create_dir!`

The `create_dir!` macro simplifies creating directories with proper error handling.

```rust
use common_errors::Result;
use common_fs::create_dir;
use std::path::Path;

fn setup_content_dir(path: &Path) -> Result<()> {
    // Before:
    // fs::create_dir_all(path)
    //     .with_context(|| format!("Failed to create directory: {}", path.display()))?;

    // After:
    create_dir!(path);
    Ok(())
}
```

### `file_exists!` and `dir_exists!`

The `file_exists!` and `dir_exists!` macros provide a clean way to check if a file or directory exists.

```rust
use common_fs::{file_exists, dir_exists};
use std::path::Path;

fn check_paths(file_path: &Path, dir_path: &Path) -> (bool, bool) {
    // Before:
    // let file_exists = path.exists() && path.is_file();
    // let dir_exists = path.exists() && path.is_dir();

    // After:
    let file_exists = file_exists!(file_path);
    let dir_exists = dir_exists!(dir_path);
    (file_exists, dir_exists)
}
```

## Trait Implementation Macros

Trait implementation macros are provided by the `common_macros` crate and simplify common trait implementations across the codebase.

### `impl_default_options!`

The `impl_default_options!` macro implements the `Default` trait for option structs with a builder pattern, reducing boilerplate code for command-line option handling.

```rust
use common_macros::impl_default_options;

// Before:
pub struct BuildOptions {
    pub output_dir: Option<String>,
    pub topic: Option<String>,
    pub include_drafts: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            output_dir: None,
            topic: None,
            include_drafts: false,
        }
    }
}

// After:
pub struct BuildOptions {
    pub output_dir: Option<String>,
    pub topic: Option<String>,
    pub include_drafts: bool,
}

impl_default_options! {
    BuildOptions {
        output_dir: Option<String> = None,
        topic: Option<String> = None,
        include_drafts: bool = false,
    }
}

// This also automatically adds builder methods:
// let options = BuildOptions::builder()
//     .output_dir(Some("build".to_string()))
//     .include_drafts(true)
//     .build();
```

### `impl_debug_logging!`

The `impl_debug_logging!` macro adds debug logging methods to a struct, ensuring consistent logging patterns across the codebase.

```rust
use common_macros::impl_debug_logging;

pub struct BuildProcess {
    name: String,
}

impl BuildProcess {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl_debug_logging! { BuildProcess }

// This adds methods like:
// process.debug_log("Starting build");
// process.info_log("Build completed");
// process.error_log("Build failed");
// process.debug_state();
```

### `impl_from_str_enum!`

The `impl_from_str_enum!` macro implements string-to-enum conversion for command-line arguments and configuration values.

```rust
use common_macros::impl_from_str_enum;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum BuildMode {
    Development,
    Production,
    Test,
}

impl_from_str_enum! {
    BuildMode {
        "dev" | "development" => BuildMode::Development,
        "prod" | "production" => BuildMode::Production,
        "test" => BuildMode::Test,
    }
}

// This allows:
// let mode = "dev".parse::<BuildMode>().unwrap();
// assert_eq!(mode, BuildMode::Development);
//
// Also adds:
// let mode = BuildMode::from_str_or_default("unknown", BuildMode::Development);
// let values = BuildMode::valid_values(); // Returns ["dev", "development", "prod", "production", "test"]
```

## Best Practices

When using these macros, follow these guidelines:

1. **Prefer Macros for Common Patterns**: Use these macros for common patterns to ensure consistency across the codebase.

2. **Documentation**: When using these macros, include documentation explaining what the macro does for maintainability.

3. **Error Handling**: Always use the error handling macros to ensure consistent error reporting.

4. **File Operations**: Always use the file system macros for file and directory operations to ensure consistent error handling.

5. **Trait Implementations**: Use the trait implementation macros to reduce boilerplate and ensure consistent trait implementations.

## Implementation Details

The macros are implemented in the following files:

- `tools/common/errors/src/macros.rs` - Error handling macros
- `tools/common/fs/src/macros.rs` - File system macros
- `tools/common/macros/src/lib.rs` - Trait implementation macros

These macros are re-exported by their respective crates for ease of use.

## Conclusion

By using these macros, we can significantly reduce code duplication and ensure consistent patterns across the codebase. This makes the code more maintainable, readable, and less error-prone.

When extending the codebase, consider adding new macros for patterns that are repeated frequently.
