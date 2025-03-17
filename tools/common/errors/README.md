# Error Handling Module

This module provides standardized error handling mechanisms for the writing project, ensuring consistent error reporting and handling across all tools.

## Features

- Custom error types with context information
- Error categorization (IO, Validation, Configuration, etc.)
- Error context enhancement with file paths and line numbers
- Result extensions for better error handling
- Automatic error propagation with context

## Usage

### Basic Error Handling

```rust
use common_errors::{Result, WritingError};

fn some_function() -> Result<()> {
    // Return an IO error
    if condition_not_met {
        return Err(WritingError::io_error("Failed to read file"));
    }
    
    // Return a validation error
    if invalid_input {
        return Err(WritingError::validation_error("Invalid input"));
    }
    
    Ok(())
}
```

### Adding Context to Errors

```rust
use common_errors::{Result, WritingError, ErrorContext, ResultExt};
use std::path::Path;

fn process_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .with_enhanced_context(|| {
            ErrorContext::new("read file")
                .with_file(path)
                .with_details("Unable to read file contents")
        })
}
```

### Using the Result Extension Traits

```rust
use common_errors::{Result, ResultExt, IoResultExt};
use std::path::Path;

fn read_config(path: &Path) -> Result<String> {
    // For standard results
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    // For IO results specifically
    std::fs::write(path, content)
        .with_io_context(|| format!("Failed to write to file: {}", path.display()))
}
```

### Error Propagation with the `?` Operator

```rust
use common_errors::{Result, WritingError};
use std::path::Path;

fn process_files(paths: &[&Path]) -> Result<Vec<String>> {
    let mut results = Vec::new();
    
    for path in paths {
        // The ? operator will propagate errors automatically
        let content = std::fs::read_to_string(path)
            .map_err(|e| WritingError::io_error(format!("Failed to read {}: {}", path.display(), e)))?;
        
        results.push(content);
    }
    
    Ok(results)
}
```

## Error Categories

The module provides several error categories:

- **IO Errors**: File operations, directory operations
- **Validation Errors**: Input validation, schema validation
- **Configuration Errors**: Missing or invalid configuration
- **Template Errors**: Template rendering issues
- **Command Errors**: Command execution failures
- **Parsing Errors**: Content or data parsing issues
- **Serialization Errors**: Data serialization/deserialization issues
- **External Errors**: Errors from external dependencies

Each category provides specific context information and formatting.

## Integration with Other Modules

The `common_errors` module integrates seamlessly with other modules:

- `common_fs` uses it for file operation errors
- `common_cli` uses it for command line argument processing
- `common_markdown` uses it for markdown parsing and processing
- All tools use it for consistent error handling and reporting

## Design Philosophy

The error handling is designed with the following principles:

1. **Consistency**: All errors follow the same pattern
2. **Context**: Errors include context information
3. **Propagation**: Errors can be easily propagated and enhanced
4. **User-friendly**: Error messages are clear and actionable
5. **Debugging**: Error contexts include file paths and line numbers for easier debugging 