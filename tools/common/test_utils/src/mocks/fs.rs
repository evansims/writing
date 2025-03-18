//! Filesystem mocks for testing
//!
//! This module provides mock implementations of filesystem operations for testing.

use std::path::{Path, PathBuf};
use mockall::mock;
use common_errors::Result;

/// The FileSystem trait defines operations for interacting with the filesystem
#[mockall::automock]
pub trait FileSystem {
    /// Check if a file exists at the given path
    fn file_exists(&self, path: &Path) -> Result<bool>;

    /// Check if a directory exists at the given path
    fn dir_exists(&self, path: &Path) -> Result<bool>;

    /// Read the contents of a file
    fn read_file(&self, path: &Path) -> Result<String>;

    /// Write contents to a file
    fn write_file(&self, path: &Path, contents: &str) -> Result<()>;

    /// Append contents to a file
    fn append_file(&self, path: &Path, contents: &str) -> Result<()>;

    /// Create a directory and any parent directories
    fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// List files in a directory
    fn list_files(&self, path: &Path) -> Result<Vec<PathBuf>>;

    /// List subdirectories in a directory
    fn list_dirs(&self, path: &Path) -> Result<Vec<PathBuf>>;

    /// Remove a file
    fn remove_file(&self, path: &Path) -> Result<()>;

    /// Remove a directory and all its contents
    fn remove_dir_all(&self, path: &Path) -> Result<()>;

    /// Copy a file from source to destination
    fn copy_file(&self, from: &Path, to: &Path) -> Result<()>;

    /// Move a file from source to destination
    fn move_file(&self, from: &Path, to: &Path) -> Result<()>;
}

/// A test implementation of FileSystem that operates on an in-memory filesystem
pub struct InMemoryFileSystem {
    files: std::collections::HashMap<PathBuf, String>,
    dirs: std::collections::HashSet<PathBuf>,
}

impl InMemoryFileSystem {
    /// Create a new in-memory filesystem
    pub fn new() -> Self {
        Self {
            files: std::collections::HashMap::new(),
            dirs: std::collections::HashSet::new(),
        }
    }

    /// Add a file to the in-memory filesystem
    pub fn add_file(&mut self, path: PathBuf, contents: String) {
        // Create parent directories
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        self.ensure_dir(parent);

        // Add the file
        self.files.insert(path, contents);
    }

    /// Add a directory to the in-memory filesystem
    pub fn add_dir(&mut self, path: PathBuf) {
        self.ensure_dir(&path);
    }

    /// Ensure a directory exists, creating parent directories if needed
    fn ensure_dir(&mut self, path: &Path) {
        if path.as_os_str().is_empty() {
            return;
        }

        self.dirs.insert(path.to_path_buf());

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                self.ensure_dir(parent);
            }
        }
    }
}

impl FileSystem for InMemoryFileSystem {
    fn file_exists(&self, path: &Path) -> Result<bool> {
        Ok(self.files.contains_key(&path.to_path_buf()))
    }

    fn dir_exists(&self, path: &Path) -> Result<bool> {
        Ok(self.dirs.contains(&path.to_path_buf()))
    }

    fn read_file(&self, path: &Path) -> Result<String> {
        match self.files.get(&path.to_path_buf()) {
            Some(contents) => Ok(contents.clone()),
            None => Err(common_errors::WritingError::file_not_found(path)),
        }
    }

    fn write_file(&self, path: &Path, contents: &str) -> Result<()> {
        let mut fs = self.clone();
        fs.add_file(path.to_path_buf(), contents.to_string());
        Ok(())
    }

    fn append_file(&self, path: &Path, contents: &str) -> Result<()> {
        let mut fs = self.clone();
        let existing = fs.read_file(path).unwrap_or_default();
        fs.add_file(path.to_path_buf(), format!("{}{}", existing, contents));
        Ok(())
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        let mut fs = self.clone();
        fs.ensure_dir(path);
        Ok(())
    }

    fn list_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        if !self.dir_exists(path)? {
            return Err(common_errors::WritingError::directory_not_found(path));
        }

        let mut files = Vec::new();
        let path_str = path.to_string_lossy();

        for file_path in self.files.keys() {
            if let Some(parent) = file_path.parent() {
                if parent == path {
                    files.push(file_path.clone());
                }
            }
        }

        Ok(files)
    }

    fn list_dirs(&self, path: &Path) -> Result<Vec<PathBuf>> {
        if !self.dir_exists(path)? {
            return Err(common_errors::WritingError::directory_not_found(path));
        }

        let mut dirs = Vec::new();
        let path_str = path.to_string_lossy();

        for dir_path in self.dirs.iter() {
            if let Some(parent) = dir_path.parent() {
                if parent == path {
                    dirs.push(dir_path.clone());
                }
            }
        }

        Ok(dirs)
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        if !self.file_exists(path)? {
            return Err(common_errors::WritingError::file_not_found(path));
        }

        let mut fs = self.clone();
        fs.files.remove(&path.to_path_buf());
        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        if !self.dir_exists(path)? {
            return Err(common_errors::WritingError::directory_not_found(path));
        }

        let mut fs = self.clone();
        let path_str = path.to_string_lossy();

        // Remove all files in or under this directory
        let files_to_remove: Vec<PathBuf> = fs.files.keys()
            .filter(|file_path| {
                file_path.to_string_lossy().starts_with(&*path_str)
            })
            .cloned()
            .collect();

        // Remove all directories under this directory
        let dirs_to_remove: Vec<PathBuf> = fs.dirs.iter()
            .filter(|dir_path| {
                dir_path.to_string_lossy().starts_with(&*path_str)
            })
            .cloned()
            .collect();

        for file_path in files_to_remove {
            fs.files.remove(&file_path);
        }

        for dir_path in dirs_to_remove {
            fs.dirs.remove(&dir_path);
        }

        Ok(())
    }

    fn copy_file(&self, from: &Path, to: &Path) -> Result<()> {
        let contents = self.read_file(from)?;
        self.write_file(to, &contents)?;
        Ok(())
    }

    fn move_file(&self, from: &Path, to: &Path) -> Result<()> {
        let contents = self.read_file(from)?;
        self.write_file(to, &contents)?;
        self.remove_file(from)?;
        Ok(())
    }
}

impl Clone for InMemoryFileSystem {
    fn clone(&self) -> Self {
        Self {
            files: self.files.clone(),
            dirs: self.dirs.clone(),
        }
    }
}

/// Helper function to create a file system with test files
pub fn create_test_fs() -> InMemoryFileSystem {
    let mut fs = InMemoryFileSystem::new();

    // Create some test directories
    fs.add_dir(PathBuf::from("content"));
    fs.add_dir(PathBuf::from("content/blog"));
    fs.add_dir(PathBuf::from("content/tutorials"));

    // Add some test files
    fs.add_file(
        PathBuf::from("content/blog/test-article.md"),
        "---\ntitle: Test Article\ntopic: blog\n---\nThis is test content.".to_string(),
    );

    fs.add_file(
        PathBuf::from("content/tutorials/getting-started.md"),
        "---\ntitle: Getting Started\ntopic: tutorials\n---\nFollow these steps to get started.".to_string(),
    );

    fs
}