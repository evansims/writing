#[cfg(feature = "content")]
mod content_path;

// Add the normalize module
pub mod normalize;
pub mod cleanup;
// Add the directory module for directory operations
pub mod directory;
pub mod file;
pub mod macros;  // Include the new macros module

#[cfg(feature = "content")]
pub use content_path::find_content_path;

// Re-export key directory operations for convenience
pub use directory::{move_dir, copy_dir_all, has_content, copy_content, move_content};

// Re-export from file module
pub use file::{
    read_file, write_file, create_dir, file_exists, dir_exists,
    delete_file, delete_dir, resolve_path
};

// Re-export from cleanup module
pub use cleanup::{
    copy_file, copy_file_std
};

use common_errors::{Result, WritingError, ErrorContext, IoResultExt, ResultExt};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(feature = "find")]
use walkdir::WalkDir;

// Include external test module
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

/// Check if a path exists
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Create a directory and all parent directories if they don't exist
pub fn create_dir_all(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_enhanced_context(|| {
            ErrorContext::new("create directory")
                .with_file(path)
                .with_details("Unable to create directory and parent directories")
        })
}

/// Read content from a file if it exists, returning None if it doesn't
pub fn read_file_if_exists(path: &Path) -> Result<Option<String>> {
    if !path.exists() {
        return Ok(None);
    }

    read_file(path).map(Some)
}

/// Delete a directory and all its contents
pub fn delete_dir_all(path: &Path) -> Result<()> {
    if !path.exists() {
        // Non-existent directory is already deleted, so return success
        return Ok(());
    }

    fs::remove_dir_all(path)
        .with_enhanced_context(|| {
            ErrorContext::new("delete directory")
                .with_file(path)
                .with_details("Unable to delete directory and its contents")
        })
}

/// Find all directories in a path that match a specific depth
#[cfg(feature = "find")]
pub fn find_dirs_with_depth(base_path: &Path, min_depth: usize, max_depth: usize) -> Result<Vec<PathBuf>> {
    if !base_path.exists() {
        return Err(WritingError::directory_not_found(base_path));
    }

    let mut dirs = Vec::new();

    let walker = WalkDir::new(base_path)
        .min_depth(min_depth)
        .max_depth(max_depth)
        .into_iter();

    for entry in walker {
        let entry = entry.with_context(|| format!("Failed to read directory entry in {}", base_path.display()))?;
        if entry.file_type().is_dir() {
            dirs.push(entry.path().to_path_buf());
        }
    }

    Ok(dirs)
}

/// Find all files in a path with a specific extension
#[cfg(feature = "find")]
pub fn find_files_with_extension(base_path: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    if !base_path.exists() {
        return Err(WritingError::directory_not_found(base_path));
    }

    let mut files = Vec::new();
    let dot_extension = format!(".{}", extension);

    for entry in WalkDir::new(base_path).into_iter() {
        let entry = entry.map_err(WritingError::from)?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == extension {
                    files.push(path.to_path_buf());
                }
            } else if let Some(file_name) = path.file_name() {
                if let Some(name) = file_name.to_str() {
                    if name.ends_with(&dot_extension) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    Ok(files)
}