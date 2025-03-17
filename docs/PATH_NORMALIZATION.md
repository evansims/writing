# Path Normalization Utilities

This document describes the path normalization utilities provided by the `common_fs::normalize` module in the Writing project.

## Overview

Path normalization is essential for ensuring consistent path handling across different operating systems. The `common_fs::normalize` module provides utilities for normalizing paths, converting between absolute and relative paths, and handling path separators consistently.

## Benefits

- **Cross-Platform Compatibility**: Ensures paths work correctly on different operating systems (Windows, macOS, Linux)
- **Consistent Path Handling**: Standardizes path operations across the codebase
- **Simplified Path Manipulation**: Provides utility functions for common path operations
- **Error Prevention**: Reduces errors related to path handling and manipulation

## Available Functions

### normalize_path

Normalizes a path by resolving `.` and `..` components, but does not resolve symbolic links or convert to an absolute path.

```rust
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf
```

**Example:**

```rust
use common_fs::normalize::normalize_path;
use std::path::Path;

let path = Path::new("content/./blog/../blog/post");
let normalized = normalize_path(path);
assert_eq!(normalized.to_str().unwrap(), "content/blog/post");
```

### to_absolute_path

Converts a path to an absolute path by resolving it against the current working directory.

```rust
pub fn to_absolute_path<P: AsRef<Path>>(path: P) -> Result<PathBuf>
```

**Example:**

```rust
use common_fs::normalize::to_absolute_path;
use std::path::Path;

let path = Path::new("content/blog/post");
let absolute = to_absolute_path(path)?;
assert!(absolute.is_absolute());
```

### to_relative_path

Converts a path to a relative path by resolving it against a base path.

```rust
pub fn to_relative_path<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: B) -> Result<PathBuf>
```

**Example:**

```rust
use common_fs::normalize::to_relative_path;
use std::path::Path;

let base = Path::new("/home/user/project");
let full_path = Path::new("/home/user/project/content/blog/post");
let relative = to_relative_path(full_path, base)?;
assert_eq!(relative.to_str().unwrap(), "content/blog/post");
```

### to_canonical_path

Converts a path to its canonical form by resolving symbolic links and normalizing the path.

```rust
pub fn to_canonical_path<P: AsRef<Path>>(path: P) -> Result<PathBuf>
```

**Example:**

```rust
use common_fs::normalize::to_canonical_path;
use std::path::Path;

let path = Path::new("content/blog/post");
let canonical = to_canonical_path(path)?;
// Result depends on the filesystem
```

### normalize_separators

Normalizes path separators to use the platform-specific separator, ensuring cross-platform compatibility.

```rust
pub fn normalize_separators<P: AsRef<Path>>(path: P) -> PathBuf
```

**Example:**

```rust
use common_fs::normalize::normalize_separators;
use std::path::Path;

let path = Path::new("content/blog\\post");
let normalized = normalize_separators(path);
// On Unix: "content/blog/post"
// On Windows: "content\\blog\\post"
```

### ensure_trailing_separator

Ensures that a path ends with a separator, which is useful when concatenating paths as strings.

```rust
pub fn ensure_trailing_separator<P: AsRef<Path>>(path: P) -> PathBuf
```

**Example:**

```rust
use common_fs::normalize::ensure_trailing_separator;
use std::path::Path;

let path = Path::new("content/blog");
let with_separator = ensure_trailing_separator(path);
// On Unix: "content/blog/"
// On Windows: "content\\blog\\"
```

### join_paths

Joins paths with proper normalization, ensuring that the result is a valid path.

```rust
pub fn join_paths<B: AsRef<Path>, P: AsRef<Path>>(base: B, path: P) -> PathBuf
```

**Example:**

```rust
use common_fs::normalize::join_paths;
use std::path::Path;

let base = Path::new("content");
let path = Path::new("../blog/post");
let joined = join_paths(base, path);
assert_eq!(joined.to_str().unwrap(), "blog/post");
```

## Usage Examples

### Normalizing User Input

When accepting path input from users, it's important to normalize the path to ensure it's valid and consistent:

```rust
use common_fs::normalize::{normalize_path, normalize_separators};
use std::path::Path;

fn process_user_path(user_input: &str) -> PathBuf {
    let path = Path::new(user_input);
    let normalized = normalize_path(path);
    normalize_separators(normalized)
}
```

### Working with Relative Paths

When working with relative paths, it's often necessary to convert them to absolute paths:

```rust
use common_fs::normalize::{to_absolute_path, to_relative_path};
use std::path::Path;
use common_errors::Result;

fn process_relative_path(relative_path: &str, base_dir: &str) -> Result<()> {
    let path = Path::new(relative_path);
    let base = Path::new(base_dir);
    
    // Convert to absolute path
    let absolute = to_absolute_path(path)?;
    println!("Absolute path: {}", absolute.display());
    
    // Convert back to relative path
    let relative = to_relative_path(&absolute, base)?;
    println!("Relative path: {}", relative.display());
    
    Ok(())
}
```

### Joining Paths Safely

When joining paths, it's important to handle edge cases like `..` components:

```rust
use common_fs::normalize::join_paths;
use std::path::Path;

fn join_path_safely(base: &str, path: &str) -> PathBuf {
    let base_path = Path::new(base);
    let path_to_join = Path::new(path);
    join_paths(base_path, path_to_join)
}
```

## Best Practices

1. **Always Normalize User Input**: User-provided paths should always be normalized to prevent path traversal attacks and ensure consistency.

2. **Use Absolute Paths for File Operations**: When performing file operations, it's often safer to use absolute paths to avoid confusion about the current working directory.

3. **Handle Path Separators Consistently**: Use the `normalize_separators` function to ensure path separators are consistent with the current platform.

4. **Prefer Path Manipulation Functions**: Use the provided path manipulation functions instead of string manipulation to ensure paths are handled correctly.

5. **Handle Errors Gracefully**: Path operations can fail for various reasons, so always handle errors gracefully and provide meaningful error messages.

## Implementation Details

The path normalization utilities are implemented in the `common_fs::normalize` module. The implementation uses the standard library's `Path` and `PathBuf` types, along with custom logic for handling path components and separators.

The module is designed to be platform-independent, with specific handling for Windows and Unix-like systems where necessary. This ensures that paths are handled consistently across different operating systems.

## Testing

The path normalization utilities are thoroughly tested to ensure they work correctly on different platforms. The tests cover various edge cases, including:

- Paths with `.` and `..` components
- Paths with mixed separators
- Absolute and relative paths
- Paths with symbolic links
- Paths with non-existent components

## Conclusion

The path normalization utilities provided by the `common_fs::normalize` module are essential for ensuring consistent path handling across different operating systems. By using these utilities, you can avoid common pitfalls and ensure that your code works correctly on all platforms. 