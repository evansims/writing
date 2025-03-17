//! # Directory Operations Utilities
//! 
//! This module provides utilities for working with directories, including copying, moving,
//! and checking directory contents.
//! 
//! ## Features
//! 
//! - Move directories with fallback mechanism for cross-filesystem moves
//! - Copy directories recursively with proper error handling
//! - Check if directories have content
//! - Convenience wrappers for directory operations with string paths
//! 
//! ## Examples
//! 
//! ```rust,no_run
//! use common_fs::directory::{copy_dir_all, move_dir, has_content};
//! use std::path::Path;
//! use common_errors::Result;
//! 
//! # fn main() -> Result<()> {
//! // Copy a directory recursively
//! copy_dir_all(Path::new("source_dir"), Path::new("target_dir"))?;
//! 
//! // Move a directory
//! move_dir(Path::new("old_location"), Path::new("new_location"))?;
//! 
//! // Check if a directory has content
//! if has_content(Path::new("some_dir")) {
//!     println!("Directory has content");
//! }
//! 
//! // Copy content using string paths
//! common_fs::copy_content("base_dir", "source_path", "target_path")?;
//! 
//! // Move content using string paths
//! common_fs::move_content("base_dir", "source_path", "target_path")?;
//! # Ok(())
//! # }
//! ```
//! 
//! ## Feature Flags
//! 
//! - `fs_extra`: Enables the `copy_dir_with_fs_extra` function for more efficient directory copying
//! - `find`: Required for finding functionality in other modules
//! - `directory_ops`: Combines both `fs_extra` and `find` for full directory operations

use std::fs;
use std::path::Path;
use common_errors::{Result, ResultExt};

#[cfg(feature = "fs_extra")]
use fs_extra::dir::{copy as fs_extra_copy, CopyOptions};

/// Move a directory from one location to another
///
/// This function tries to use fs::rename first (fast path), but falls back
/// to copy and remove if rename fails (e.g., across filesystems).
///
/// # Parameters
///
/// * `from` - Source directory
/// * `to` - Target directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the directory cannot be moved
pub fn move_dir(from: &Path, to: &Path) -> Result<()> {
    // Try to use fs::rename first (fast path)
    match fs::rename(from, to) {
        Ok(_) => Ok(()),
        Err(_) => {
            // If rename fails (e.g., across filesystems), fall back to copy and remove
            copy_dir_all(from, to)?;
            fs::remove_dir_all(from)
                .with_context(|| {
                    format!("Unable to remove directory after copying: {}", from.display())
                })
        }
    }
}

/// Copy a directory recursively without external dependencies
///
/// This function copies a directory recursively from one location to another
/// using only standard library functions.
///
/// # Parameters
///
/// * `src` - Source directory
/// * `dst` - Target directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the directory cannot be copied
pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)
            .with_context(|| {
                format!("Unable to create target directory during copy: {}", dst.display())
            })?;
    }
    
    for entry in fs::read_dir(src)
        .with_context(|| {
            format!("Unable to read source directory during copy: {}", src.display())
        })?
    {
        let entry = entry
            .with_context(|| {
                "Unable to read directory entry during copy".to_string()
            })?;
        
        let ty = entry.file_type()
            .with_context(|| {
                format!("Unable to get file type during copy: {}", entry.path().display())
            })?;
        
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            crate::copy_file_std(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Copy a directory using fs_extra if available
///
/// This function uses the fs_extra crate to copy a directory recursively.
/// It requires the "fs_extra" feature to be enabled.
///
/// # Parameters
///
/// * `from` - Source directory
/// * `to` - Target directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the directory cannot be copied
#[cfg(feature = "fs_extra")]
pub fn copy_dir_with_fs_extra(from: &Path, to: &Path) -> Result<()> {
    // Create target directory if it doesn't exist
    if !to.exists() {
        crate::create_dir_all(to)?;
    }
    
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    
    // Copy directory
    fs_extra_copy(from, to, &options)
        .map(|_| ())
        .with_context(|| {
            format!("Unable to copy directory to {}", to.display())
        })
}

/// Check if a directory has any content
///
/// This function checks if a directory has any content.
///
/// # Parameters
///
/// * `dir` - Directory to check
///
/// # Returns
///
/// Returns true if the directory has content, false otherwise
pub fn has_content(dir: &Path) -> bool {
    if !dir.exists() || !dir.is_dir() {
        return false;
    }
    
    // Check if the directory has any entries
    match fs::read_dir(dir) {
        Ok(entries) => entries.count() > 0,
        Err(_) => false,
    }
}

/// Copy directory and all its contents to another directory
///
/// This function is a convenience wrapper around copy_dir_all that
/// takes strings as input and creates parent directories as needed.
///
/// # Parameters
///
/// * `base_dir` - Base directory
/// * `source_path` - Source directory (relative to base_dir)
/// * `target_path` - Target directory (relative to base_dir)
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the directory cannot be copied
pub fn copy_content(base_dir: &str, source_path: &str, target_path: &str) -> Result<()> {
    let source_dir = Path::new(base_dir).join(source_path);
    let target_dir = Path::new(base_dir).join(target_path);
    
    // Create the target directory if it doesn't exist
    if !target_dir.exists() {
        crate::create_dir_all(&target_dir)?;
    }
    
    copy_dir_all(&source_dir, &target_dir)
}

/// Move directory and all its contents to another directory
///
/// This function is a convenience wrapper around move_dir that
/// takes strings as input and creates parent directories as needed.
///
/// # Parameters
///
/// * `base_dir` - Base directory
/// * `source_path` - Source directory (relative to base_dir)
/// * `target_path` - Target directory (relative to base_dir)
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the directory cannot be moved
pub fn move_content(base_dir: &str, source_path: &str, target_path: &str) -> Result<()> {
    let source_dir = Path::new(base_dir).join(source_path);
    let target_dir = Path::new(base_dir).join(target_path);
    
    // Create the target directory if it doesn't exist
    if !target_dir.exists() {
        crate::create_dir_all(&target_dir)?;
    }
    
    move_dir(&source_dir, &target_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn test_copy_dir_all() {
        let temp_dir = tempdir().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");
        
        fs::create_dir_all(&src_dir).unwrap();
        
        // Create a test file
        let file_path = src_dir.join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "Hello, world!").unwrap();
        
        // Create a nested directory
        let nested_dir = src_dir.join("nested");
        fs::create_dir_all(&nested_dir).unwrap();
        let nested_file = nested_dir.join("nested.txt");
        let mut file = File::create(&nested_file).unwrap();
        write!(file, "Nested file").unwrap();
        
        // Copy the directory
        let result = copy_dir_all(&src_dir, &dst_dir);
        assert!(result.is_ok());
        
        // Check that the target files exist
        assert!(dst_dir.exists());
        assert!(dst_dir.join("test.txt").exists());
        assert!(dst_dir.join("nested").exists());
        assert!(dst_dir.join("nested").join("nested.txt").exists());
        
        // Check file contents
        let content = fs::read_to_string(dst_dir.join("test.txt")).unwrap();
        assert_eq!(content, "Hello, world!");
        let content = fs::read_to_string(dst_dir.join("nested").join("nested.txt")).unwrap();
        assert_eq!(content, "Nested file");
    }

    #[test]
    fn test_move_dir() {
        let temp_dir = tempdir().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");
        
        fs::create_dir_all(&src_dir).unwrap();
        
        // Create a test file
        let file_path = src_dir.join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "Hello, world!").unwrap();
        
        // Move the directory
        let result = move_dir(&src_dir, &dst_dir);
        assert!(result.is_ok());
        
        // Check that the source directory no longer exists
        assert!(!src_dir.exists());
        
        // Check that the target files exist
        assert!(dst_dir.exists());
        assert!(dst_dir.join("test.txt").exists());
        
        // Check file contents
        let content = fs::read_to_string(dst_dir.join("test.txt")).unwrap();
        assert_eq!(content, "Hello, world!");
    }

    #[test]
    fn test_has_content() {
        let temp_dir = tempdir().unwrap();
        
        // Empty directory
        assert!(!has_content(temp_dir.path()));
        
        // Directory with content
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "Hello, world!").unwrap();
        
        assert!(has_content(temp_dir.path()));
        
        // Non-existent directory
        let non_existent = temp_dir.path().join("non_existent");
        assert!(!has_content(&non_existent));
    }
} 