//! # File System Operation Macros
//!
//! This module provides macros for common file system operations with proper error handling.

/// Read a file with proper error context
///
/// # Examples
///
/// ```
/// use common_fs::read_file;
/// use std::path::Path;
///
/// fn read_config() -> common_errors::Result<String> {
///     let path = Path::new("config.yaml");
///     let content = read_file!(path);
///     Ok(content)
/// }
/// ```
#[macro_export]
macro_rules! read_file {
    ($path:expr) => {
        match std::fs::read_to_string($path) {
            Ok(content) => content,
            Err(err) => {
                return Err(common_errors::WritingError::io_error(&format!(
                    "Failed to read file {}: {}",
                    $path.display(),
                    err
                )));
            }
        }
    };
    ($path:expr, $err_msg:expr) => {
        match std::fs::read_to_string($path) {
            Ok(content) => content,
            Err(err) => {
                return Err(common_errors::WritingError::io_error(&format!(
                    "{}: {}",
                    $err_msg,
                    err
                )));
            }
        }
    };
}

/// Write to a file with proper error context
///
/// # Examples
///
/// ```
/// use common_fs::write_file;
/// use std::path::Path;
///
/// fn write_config(content: &str) -> common_errors::Result<()> {
///     let path = Path::new("config.yaml");
///     write_file!(path, content);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! write_file {
    ($path:expr, $content:expr) => {
        match std::fs::write($path, $content) {
            Ok(_) => (),
            Err(err) => {
                return Err(common_errors::WritingError::io_error(&format!(
                    "Failed to write to file {}: {}",
                    $path.display(),
                    err
                )));
            }
        }
    };
    ($path:expr, $content:expr, $err_msg:expr) => {
        match std::fs::write($path, $content) {
            Ok(_) => (),
            Err(err) => {
                return Err(common_errors::WritingError::io_error(&format!(
                    "{}: {}",
                    $err_msg,
                    err
                )));
            }
        }
    };
}

/// Create a directory with proper error context
///
/// # Examples
///
/// ```
/// use common_fs::create_dir;
/// use std::path::Path;
///
/// fn setup_content_dir() -> common_errors::Result<()> {
///     let path = Path::new("content");
///     create_dir!(path);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! create_dir {
    ($path:expr) => {
        match std::fs::create_dir_all($path) {
            Ok(_) => (),
            Err(err) => {
                return Err(common_errors::WritingError::io_error(&format!(
                    "Failed to create directory {}: {}",
                    $path.display(),
                    err
                )));
            }
        }
    };
    ($path:expr, $err_msg:expr) => {
        match std::fs::create_dir_all($path) {
            Ok(_) => (),
            Err(err) => {
                return Err(common_errors::WritingError::io_error(&format!(
                    "{}: {}",
                    $err_msg,
                    err
                )));
            }
        }
    };
}

/// Check if a file exists with proper error handling
///
/// # Examples
///
/// ```
/// use common_fs::file_exists;
/// use std::path::Path;
///
/// fn check_config() -> bool {
///     let path = Path::new("config.yaml");
///     file_exists!(path)
/// }
/// ```
#[macro_export]
macro_rules! file_exists {
    ($path:expr) => {
        std::path::Path::new($path).exists() && std::path::Path::new($path).is_file()
    };
}

/// Check if a directory exists with proper error handling
///
/// # Examples
///
/// ```
/// use common_fs::dir_exists;
/// use std::path::Path;
///
/// fn check_content_dir() -> bool {
///     let path = Path::new("content");
///     dir_exists!(path)
/// }
/// ```
#[macro_export]
macro_rules! dir_exists {
    ($path:expr) => {
        std::path::Path::new($path).exists() && std::path::Path::new($path).is_dir()
    };
}