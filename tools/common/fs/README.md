# File System Operations Module

This module provides a comprehensive set of utilities for performing file system operations in a safe, consistent manner with proper error handling.

## Features

- File operations: read, write, copy, delete
- Directory operations: create, copy, move, delete
- Path normalization and cleanup
- Safe file handling with proper resource cleanup
- Content path finding and management

## Usage

### Basic File Operations

```rust
use common_fs::{read_file, write_file, copy_file_std, delete_file};
use std::path::Path;

// Read from a file
let content = read_file(Path::new("some_file.txt"))?;

// Write to a file
write_file(Path::new("output.txt"), "Hello, world!")?;

// Copy a file
copy_file_std(Path::new("source.txt"), Path::new("destination.txt"))?;

// Delete a file
delete_file(Path::new("to_delete.txt"))?;
```

### Directory Operations

```rust
use common_fs::{create_dir_all, delete_dir_all, move_dir, copy_dir_all};
use std::path::Path;

// Create a directory structure
create_dir_all(Path::new("path/to/new/dir"))?;

// Copy a directory and all its contents
copy_dir_all(Path::new("source_dir"), Path::new("destination_dir"))?;

// Move a directory
move_dir(Path::new("old_location"), Path::new("new_location"))?;

// Delete a directory and all its contents
delete_dir_all(Path::new("to_delete"))?;
```

### Convenience Wrappers

```rust
use common_fs::{copy_content, move_content};

// Copy content using string paths
copy_content("base_dir", "source_path", "target_path")?;

// Move content using string paths
move_content("base_dir", "source_path", "target_path")?;
```

### Path Utilities

```rust
use common_fs::normalize::{normalize_path, clean_path};
use std::path::Path;

// Normalize a path (resolve relative parts)
let normalized = normalize_path(Path::new("path/with/../relative/parts"))?;

// Clean a path (remove extra slashes, etc.)
let cleaned = clean_path("path//with//extra///slashes");
```

### Safe File Handling

```rust
use common_fs::cleanup::{read_to_string, write_string};
use std::path::Path;

// Read using a safe file handle that cleans up resources
let content = read_to_string(Path::new("file.txt"))?;

// Write using a safe file handle
write_string(Path::new("output.txt"), "content")?;
```

## Features and Conditional Compilation

This module uses feature flags to control dependency inclusion:

- `content`: Enables content path finding utilities
- `copy`: Enables additional file copying utilities (requires `fs_extra`)
- `find`: Enables file and directory finding utilities (requires `walkdir`)
- `directory_ops`: Combines `copy` and `find` for comprehensive directory operations

## Error Handling

All functions return a `Result` type with detailed error information, using the `common_errors` module.

## Testing

All functions have comprehensive tests to ensure they work correctly in various scenarios. 