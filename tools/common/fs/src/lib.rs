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

    let walker = WalkDir::new(base_path).into_iter();

    for entry in walker {
        let entry = entry.with_context(|| format!("Failed to read directory entry in {}", base_path.display()))?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == extension {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_path_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        assert!(path_exists(temp_file.path()));

        let nonexistent_path = Path::new("/path/to/nonexistent/file.txt");
        assert!(!path_exists(nonexistent_path));
    }

    #[test]
    fn test_create_dir_all() {
        let temp_dir = tempdir().unwrap();
        let nested_dir = temp_dir.path().join("dir1").join("dir2").join("dir3");

        let result = create_dir_all(&nested_dir);
        assert!(result.is_ok());
        assert!(nested_dir.exists());
    }

    #[test]
    fn test_read_file_if_exists() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "Hello, world!";
        write!(temp_file, "{}", content).unwrap();

        let result = read_file_if_exists(temp_file.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(content.to_string()));

        let nonexistent_path = Path::new("/path/to/nonexistent/file.txt");
        let result = read_file_if_exists(nonexistent_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_delete_dir_all() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        // Create some nested directories and files
        let nested_dir = dir_path.join("dir1").join("dir2");
        create_dir_all(&nested_dir).unwrap();
        write_file(&nested_dir.join("test.txt"), "test").unwrap();

        let result = delete_dir_all(&dir_path);
        assert!(result.is_ok());
        assert!(!dir_path.exists());

        // Deleting a non-existent directory should not error
        let result = delete_dir_all(&dir_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_dirs_with_depth() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        // Create some nested directories
        let dir1 = dir_path.join("dir1");
        let dir2 = dir_path.join("dir1").join("dir2");
        let dir3 = dir_path.join("dir1").join("dir2").join("dir3");
        create_dir_all(&dir3).unwrap();

        // Find dirs with depth 1
        let dirs = find_dirs_with_depth(&dir_path, 1, 1).unwrap();
        assert_eq!(dirs.len(), 1);
        assert!(dirs.contains(&dir1));

        // Find dirs with depth 1-2
        let dirs = find_dirs_with_depth(&dir_path, 1, 2).unwrap();
        assert_eq!(dirs.len(), 2);
        assert!(dirs.contains(&dir1));
        assert!(dirs.contains(&dir2));

        // Find dirs with non-existent path
        let result = find_dirs_with_depth(Path::new("/path/to/nonexistent"), 1, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_files_with_extension() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        // Create some files with different extensions
        let txt_file = dir_path.join("test.txt");
        let md_file = dir_path.join("test.md");
        let subdir = dir_path.join("subdir");
        let nested_txt = subdir.join("nested.txt");

        fs::create_dir_all(&subdir).unwrap();
        write_file(&txt_file, "test").unwrap();
        write_file(&md_file, "test").unwrap();
        write_file(&nested_txt, "test").unwrap();

        // Find .txt files
        let files = find_files_with_extension(&dir_path, "txt").unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&txt_file));
        assert!(files.contains(&nested_txt));

        // Find .md files
        let files = find_files_with_extension(&dir_path, "md").unwrap();
        assert_eq!(files.len(), 1);
        assert!(files.contains(&md_file));

        // Find files with non-existent path
        let result = find_files_with_extension(Path::new("/path/to/nonexistent"), "txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_copy_file() {
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("source.txt");
        let subdir = temp_dir.path().join("subdir");
        let dest_file = subdir.join("dest.txt");
        let content = "Hello, world!";

        fs::create_dir_all(&subdir).unwrap();
        write_file(&source_file, content).unwrap();

        let result = copy_file(&source_file, &dest_file);
        assert!(result.is_ok());
        assert!(dest_file.exists());

        let dest_content = fs::read_to_string(&dest_file).unwrap();
        assert_eq!(dest_content, content);

        // Copying from non-existent source should error
        let result = copy_file(Path::new("/path/to/nonexistent"), &dest_file);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("File not found"));
    }

    #[test]
    fn test_copy_file_std() {
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("source.txt");
        let dest_file = temp_dir.path().join("dest.txt");

        let content = "Hello, world!";
        fs::write(&source_file, content).unwrap();

        copy_file_std(&source_file, &dest_file).unwrap();
        assert!(dest_file.exists());

        let dest_content = fs::read_to_string(&dest_file).unwrap();
        assert_eq!(dest_content, content);

        // Copying from non-existent source should error
        let result = copy_file_std(Path::new("/path/to/nonexistent"), &dest_file);
        assert!(result.is_err());
        // Just check that it's an error, don't check the specific message
        // as it might vary depending on the implementation
    }
}