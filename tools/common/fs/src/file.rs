use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use common_errors::{Result, WritingError};

/// Read a file's contents as a string
///
/// # Parameters
///
/// * `path` - Path to the file to read
///
/// # Returns
///
/// The file's contents as a string
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => WritingError::file_not_found(path),
            io::ErrorKind::PermissionDenied => WritingError::permission_denied(path),
            _ => WritingError::IoError(format!("Failed to read file {}: {}", path.display(), e)),
        })
}

/// Write content to a file
///
/// # Parameters
///
/// * `path` - Path to the file to write
/// * `content` - Content to write
///
/// # Returns
///
/// Result indicating success or failure
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let path = path.as_ref();
    fs::write(path, content)
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => WritingError::directory_not_found(path.parent().unwrap_or(path)),
            io::ErrorKind::PermissionDenied => WritingError::permission_denied(path),
            _ => WritingError::IoError(format!("Failed to write file {}: {}", path.display(), e)),
        })
}

/// Check if a file exists
///
/// # Parameters
///
/// * `path` - Path to the file to check
///
/// # Returns
///
/// True if the file exists, false otherwise
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Create a directory and all parent directories
///
/// # Parameters
///
/// * `path` - Path to the directory to create
///
/// # Returns
///
/// Result indicating success or failure
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    fs::create_dir_all(path)
        .map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => WritingError::permission_denied(path),
            _ => WritingError::IoError(format!("Failed to create directory {}: {}", path.display(), e)),
        })
}

/// Check if a directory exists
///
/// # Parameters
///
/// * `path` - Path to the directory to check
///
/// # Returns
///
/// True if the directory exists, false otherwise
pub fn dir_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Resolve a relative path to an absolute path
///
/// # Parameters
///
/// * `path` - Path to resolve
/// * `base` - Base path to resolve against
///
/// # Returns
///
/// The resolved absolute path
pub fn resolve_path<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: B) -> PathBuf {
    let path = path.as_ref();
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.as_ref().join(path)
    }
}

/// Delete a file
///
/// # Parameters
///
/// * `path` - Path to the file to delete
///
/// # Returns
///
/// Result indicating success or failure
pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !file_exists(path) {
        // Non-existent file is already deleted, so return success
        return Ok(());
    }

    fs::remove_file(path)
        .map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => WritingError::permission_denied(path),
            _ => WritingError::IoError(format!("Failed to delete file {}: {}", path.display(), e)),
        })
}

/// Delete a directory
///
/// # Parameters
///
/// * `path` - Path to the directory to delete
///
/// # Returns
///
/// Result indicating success or failure
pub fn delete_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !dir_exists(path) {
        return Err(WritingError::directory_not_found(path));
    }

    fs::remove_dir_all(path)
        .map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => WritingError::permission_denied(path),
            _ => WritingError::IoError(format!("Failed to delete directory {}: {}", path.display(), e)),
        })
}