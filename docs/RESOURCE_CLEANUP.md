# Resource Cleanup Utilities

This document describes the resource cleanup utilities provided by the `common_fs::cleanup` module in the Writing project.

## Overview

Resource cleanup is essential for ensuring that file handles and other resources are properly closed and cleaned up, even in the presence of errors or early returns. The `common_fs::cleanup` module provides utilities for ensuring proper resource cleanup, leveraging Rust's RAII (Resource Acquisition Is Initialization) pattern.

## Benefits

- **Automatic Resource Cleanup**: Ensures resources are properly closed when they go out of scope
- **Error-Safe File Operations**: Provides utilities for common file operations with proper error handling
- **Enhanced Context for Errors**: Adds detailed context to file operation errors
- **Simplified File I/O**: Provides high-level utilities for common file operations
- **Buffered I/O Support**: Includes support for buffered reading and writing

## Available Types and Functions

### SafeFile

A wrapper around `std::fs::File` that ensures the file is properly closed when it goes out of scope.

```rust
pub struct SafeFile {
    file: File,
    path: PathBuf,
}
```

#### Methods

- **open**: Opens a file in read-only mode
- **create**: Opens a file in write-only mode, creating it if it doesn't exist
- **with_options**: Opens a file with custom options
- **as_file**: Returns a reference to the underlying `File`
- **as_file_mut**: Returns a mutable reference to the underlying `File`
- **path**: Returns the path of the file
- **into_file**: Consumes the `SafeFile` and returns the underlying `File`
- **buf_reader**: Creates a buffered reader from the file
- **buf_writer**: Creates a buffered writer from the file

### Utility Functions

- **read_to_string**: Safely reads a file to a string
- **write_string**: Safely writes a string to a file
- **append_string**: Safely appends a string to a file
- **copy_file**: Safely copies a file

## Usage Examples

### Reading a File

```rust
use common_fs::cleanup::{SafeFile, read_to_string};
use std::io::Read;

// Using SafeFile directly
fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = SafeFile::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    // File is automatically closed when `file` goes out of scope
    Ok(())
}

// Using the utility function
fn read_file_utility_example() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_to_string("example.txt")?;
    println!("File contents: {}", contents);
    // File is automatically closed by the utility function
    Ok(())
}
```

### Writing to a File

```rust
use common_fs::cleanup::{SafeFile, write_string};
use std::io::Write;

// Using SafeFile directly
fn write_file_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = SafeFile::create("example.txt")?;
    file.write_all(b"Hello, world!")?;
    // File is automatically closed when `file` goes out of scope
    Ok(())
}

// Using the utility function
fn write_file_utility_example() -> Result<(), Box<dyn std::error::Error>> {
    write_string("example.txt", "Hello, world!")?;
    // File is automatically closed by the utility function
    Ok(())
}
```

### Appending to a File

```rust
use common_fs::cleanup::{append_string, write_string};

fn append_file_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create a file with initial content
    write_string("example.txt", "Hello, ")?;
    
    // Append to the file
    append_string("example.txt", "world!")?;
    
    // File is automatically closed by the utility functions
    Ok(())
}
```

### Copying a File

```rust
use common_fs::cleanup::{copy_file, write_string};

fn copy_file_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create a source file
    write_string("source.txt", "Hello, world!")?;
    
    // Copy the file
    copy_file("source.txt", "destination.txt")?;
    
    // Files are automatically closed by the utility functions
    Ok(())
}
```

### Using Buffered I/O

```rust
use common_fs::cleanup::SafeFile;
use std::io::{BufRead, Write};

fn buffered_io_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create a file with some content
    let file = SafeFile::create("example.txt")?;
    let mut writer = file.buf_writer();
    writeln!(writer, "Line 1")?;
    writeln!(writer, "Line 2")?;
    writeln!(writer, "Line 3")?;
    writer.flush()?;
    
    // Read the file line by line
    let file = SafeFile::open("example.txt")?;
    let reader = file.buf_reader();
    for line in reader.lines() {
        println!("Read: {}", line?);
    }
    
    // Files are automatically closed when the readers/writers go out of scope
    Ok(())
}
```

## Best Practices

1. **Use SafeFile Instead of File**: Prefer using `SafeFile` over `std::fs::File` to ensure proper resource cleanup.

2. **Use Utility Functions for Simple Operations**: For simple file operations, use the provided utility functions like `read_to_string` and `write_string`.

3. **Use Buffered I/O for Performance**: For performance-critical code, use the buffered I/O methods like `buf_reader` and `buf_writer`.

4. **Handle Errors Properly**: Always handle errors from file operations and provide appropriate context.

5. **Avoid Manual Resource Management**: Let Rust's RAII pattern handle resource cleanup automatically by ensuring resources are properly dropped.

## Implementation Details

The resource cleanup utilities are implemented in the `common_fs::cleanup` module. The implementation uses Rust's RAII pattern to ensure that resources are properly cleaned up when they go out of scope.

The `SafeFile` struct wraps a `std::fs::File` and implements the `Drop` trait to ensure that the file is properly closed when the struct is dropped. It also implements the `Read` and `Write` traits to provide the same interface as `File`.

The utility functions like `read_to_string` and `write_string` use `SafeFile` internally to ensure proper resource cleanup.

## Testing

The resource cleanup utilities are thoroughly tested to ensure they work correctly. The tests cover various scenarios, including:

- Opening and reading files
- Creating and writing to files
- Appending to files
- Copying files
- Using buffered I/O
- Error handling

## Conclusion

The resource cleanup utilities provided by the `common_fs::cleanup` module are essential for ensuring proper resource management in the Writing project. By using these utilities, you can avoid resource leaks and ensure that file handles are properly closed, even in the presence of errors or early returns. 