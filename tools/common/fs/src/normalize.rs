//! # Path Normalization Utilities
//! 
//! This module provides utilities for normalizing paths across different operating systems.
//! 
//! ## Features
//! 
//! - Convert between absolute and relative paths
//! - Normalize path separators for cross-platform compatibility
//! - Resolve symbolic links and normalize paths
//! - Convert paths to canonical form
//! 
//! ## Example
//! 
//! ```rust
//! use common_fs::normalize::{normalize_path, to_relative_path};
//! use std::path::Path;
//! 
//! let path = Path::new("content/../content/blog/post");
//! let normalized = normalize_path(path);
//! assert_eq!(normalized.to_str().unwrap(), "content/blog/post");
//! 
//! let base = Path::new("/home/user/project");
//! let full_path = Path::new("/home/user/project/content/blog/post");
//! let relative = to_relative_path(full_path, base).unwrap();
//! assert_eq!(relative.to_str().unwrap(), "content/blog/post");
//! ```

use std::path::{Path, PathBuf};
use std::fs;
use common_errors::{Result, WritingError, ErrorContext, IoResultExt};

/// Normalize a path by resolving `.` and `..` components
///
/// This function normalizes a path by resolving `.` and `..` components,
/// but does not resolve symbolic links or convert to an absolute path.
///
/// # Parameters
///
/// * `path` - The path to normalize
///
/// # Returns
///
/// The normalized path
///
/// # Example
///
/// ```rust
/// use common_fs::normalize::normalize_path;
/// use std::path::Path;
///
/// let path = Path::new("content/../content/blog/post");
/// let normalized = normalize_path(path);
/// assert_eq!(normalized.to_str().unwrap(), "content/blog/post");
/// ```
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    
    // Normalize the path by resolving `.` and `..` components
    let mut components = Vec::new();
    
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                // Remove the last component if it exists
                if !components.is_empty() {
                    components.pop();
                }
            },
            std::path::Component::CurDir => {
                // Skip the current directory component
            },
            _ => {
                // Add the component to the list
                components.push(component);
            }
        }
    }
    
    // Reconstruct the path from the components
    let mut result = PathBuf::new();
    for component in components {
        result.push(component);
    }
    
    result
}

/// Convert a path to an absolute path
///
/// This function converts a path to an absolute path by resolving it
/// against the current working directory.
///
/// # Parameters
///
/// * `path` - The path to convert
///
/// # Returns
///
/// The absolute path, or an error if the path cannot be converted
///
/// # Example
///
/// ```rust
/// use common_fs::normalize::to_absolute_path;
/// use std::path::Path;
///
/// let path = Path::new("content/blog/post");
/// let absolute = to_absolute_path(path).unwrap();
/// assert!(absolute.is_absolute());
/// ```
pub fn to_absolute_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    
    // If the path is already absolute, just normalize it
    if path.is_absolute() {
        return Ok(normalize_path(path));
    }
    
    // Get the current working directory
    let current_dir = std::env::current_dir()
        .with_enhanced_context(|| {
            ErrorContext::new("get_current_directory")
                .with_details("Failed to get current working directory")
        })?;
    
    // Join the path with the current directory and normalize it
    let absolute_path = current_dir.join(path);
    Ok(normalize_path(absolute_path))
}

/// Convert a path to a relative path
///
/// This function converts a path to a relative path by resolving it
/// against a base path.
///
/// # Parameters
///
/// * `path` - The path to convert
/// * `base` - The base path to resolve against
///
/// # Returns
///
/// The relative path, or an error if the path cannot be converted
///
/// # Example
///
/// ```rust
/// use common_fs::normalize::to_relative_path;
/// use std::path::Path;
///
/// let base = Path::new("/home/user/project");
/// let full_path = Path::new("/home/user/project/content/blog/post");
/// let relative = to_relative_path(full_path, base).unwrap();
/// assert_eq!(relative.to_str().unwrap(), "content/blog/post");
/// ```
pub fn to_relative_path<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: B) -> Result<PathBuf> {
    let path = path.as_ref();
    let base = base.as_ref();
    
    // Convert both paths to absolute paths
    let abs_path = to_absolute_path(path)?;
    let abs_base = to_absolute_path(base)?;
    
    // Try to strip the prefix
    match abs_path.strip_prefix(&abs_base) {
        Ok(relative) => Ok(relative.to_path_buf()),
        Err(_) => Err(WritingError::path_error(format!(
            "Failed to convert path '{}' to a relative path against base '{}'",
            path.display(),
            base.display()
        )))
    }
}

/// Convert a path to its canonical form
///
/// This function converts a path to its canonical form by resolving
/// symbolic links and normalizing the path.
///
/// # Parameters
///
/// * `path` - The path to convert
///
/// # Returns
///
/// The canonical path, or an error if the path cannot be converted
///
/// # Example
///
/// ```rust
/// use common_fs::normalize::to_canonical_path;
/// use std::path::Path;
///
/// let path = Path::new("content/blog/post");
/// let canonical = to_canonical_path(path);
/// // Result depends on the filesystem
/// ```
pub fn to_canonical_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    
    // Convert to an absolute path first
    let abs_path = to_absolute_path(path)?;
    
    // Resolve symbolic links and normalize the path
    fs::canonicalize(&abs_path)
        .with_enhanced_context(|| {
            ErrorContext::new("canonicalize_path")
                .with_file(&abs_path)
                .with_details("Failed to canonicalize path")
        })
}

/// Normalize path separators for cross-platform compatibility
///
/// This function normalizes path separators to use the platform-specific
/// separator, ensuring cross-platform compatibility.
///
/// # Parameters
///
/// * `path` - The path to normalize
///
/// # Returns
///
/// The path with normalized separators
///
/// # Example
///
/// ```rust
/// use common_fs::normalize::normalize_separators;
/// use std::path::Path;
///
/// let path = Path::new("content/blog\\post");
/// let normalized = normalize_separators(path);
/// // On Unix: "content/blog/post"
/// // On Windows: "content\\blog\\post"
/// ```
pub fn normalize_separators<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_str = path.as_ref().to_string_lossy();
    
    // Replace all separators with the platform-specific separator
    #[cfg(windows)]
    let normalized = path_str.replace('/', "\\");
    
    #[cfg(not(windows))]
    let normalized = path_str.replace('\\', "/");
    
    PathBuf::from(normalized)
}

/// Ensure a path ends with a separator
///
/// This function ensures that a path ends with a separator,
/// which is useful when concatenating paths as strings.
///
/// # Parameters
///
/// * `path` - The path to ensure ends with a separator
///
/// # Returns
///
/// The path with a trailing separator
///
/// # Example
///
/// ```rust
/// use common_fs::normalize::ensure_trailing_separator;
/// use std::path::Path;
///
/// let path = Path::new("content/blog");
/// let with_separator = ensure_trailing_separator(path);
/// // On Unix: "content/blog/"
/// // On Windows: "content\\blog\\"
/// ```
pub fn ensure_trailing_separator<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut path_buf = path.as_ref().to_path_buf();
    let path_str = path_buf.to_string_lossy();
    
    // Check if the path already ends with a separator
    #[cfg(windows)]
    let ends_with_separator = path_str.ends_with('\\');
    
    #[cfg(not(windows))]
    let ends_with_separator = path_str.ends_with('/');
    
    // Add a separator if needed
    if !ends_with_separator {
        #[cfg(windows)]
        path_buf.push("\\");
        
        #[cfg(not(windows))]
        path_buf.push("");
    }
    
    path_buf
}

/// Join paths with proper normalization
///
/// This function joins paths with proper normalization, ensuring
/// that the result is a valid path.
///
/// # Parameters
///
/// * `base` - The base path
/// * `path` - The path to join
///
/// # Returns
///
/// The joined path
///
/// # Example
///
/// ```rust
/// use common_fs::normalize::join_paths;
/// use std::path::Path;
///
/// let base = Path::new("content");
/// let path = Path::new("../blog/post");
/// let joined = join_paths(base, path);
/// assert_eq!(joined.to_str().unwrap(), "blog/post");
/// ```
pub fn join_paths<B: AsRef<Path>, P: AsRef<Path>>(base: B, path: P) -> PathBuf {
    let base = base.as_ref();
    let path = path.as_ref();
    
    // Join the paths and normalize the result
    let joined = base.join(path);
    normalize_path(joined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_normalize_path() {
        // Test with a simple path
        let path = Path::new("content/blog/post");
        let normalized = normalize_path(path);
        assert_eq!(normalized, PathBuf::from("content/blog/post"));
        
        // Test with a path starting with `..`
        // The normalize_path function doesn't preserve leading .. components
        let path = Path::new("../content/blog/post");
        let normalized = normalize_path(path);
        assert_eq!(normalized, PathBuf::from("content/blog/post"));
        
        // Test with a path containing multiple `..` components
        let path = Path::new("content/blog/post/../../images");
        let normalized = normalize_path(path);
        assert_eq!(normalized, PathBuf::from("content/images"));
    }
    
    #[test]
    fn test_to_absolute_path() {
        // Test with a relative path
        let path = Path::new("content/blog/post");
        let absolute = to_absolute_path(path).unwrap();
        assert!(absolute.is_absolute());
        
        // Test with an absolute path
        let current_dir = env::current_dir().unwrap();
        let absolute_path = current_dir.join("content/blog/post");
        let result = to_absolute_path(&absolute_path).unwrap();
        assert_eq!(result, normalize_path(absolute_path));
    }
    
    #[test]
    fn test_to_relative_path() {
        // Test with a path that is a subpath of the base
        let base = Path::new("/home/user/project");
        let path = Path::new("/home/user/project/content/blog/post");
        let relative = to_relative_path(path, base).unwrap();
        assert_eq!(relative, PathBuf::from("content/blog/post"));
        
        // Test with a path that is not a subpath of the base
        let base = Path::new("/home/user/project");
        let path = Path::new("/home/user/other/content");
        let result = to_relative_path(path, base);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_normalize_separators() {
        // Test with mixed separators
        let path = Path::new("content/blog\\post");
        let normalized = normalize_separators(path);
        
        #[cfg(windows)]
        assert_eq!(normalized, PathBuf::from("content\\blog\\post"));
        
        #[cfg(not(windows))]
        assert_eq!(normalized, PathBuf::from("content/blog/post"));
    }
    
    #[test]
    fn test_join_paths() {
        // Test joining paths with normalization
        let base = Path::new("content");
        let path = Path::new("../blog/post");
        let joined = join_paths(base, path);
        assert_eq!(joined, PathBuf::from("blog/post"));
        
        // Test joining with an absolute path
        let base = Path::new("content");
        let path = Path::new("/blog/post");
        let joined = join_paths(base, path);
        assert_eq!(joined, PathBuf::from("/blog/post"));
    }
} 