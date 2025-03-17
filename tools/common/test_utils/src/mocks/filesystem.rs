//! # Mock FileSystem Implementation
//! 
//! This module provides a mock implementation of filesystem operations for testing.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use common_errors::{Result, WritingError};

/// A mock implementation of filesystem operations
#[derive(Debug, Clone, Default)]
pub struct MockFileSystem {
    files: Arc<Mutex<HashMap<PathBuf, String>>>,
    directories: Arc<Mutex<HashMap<PathBuf, bool>>>,
}

impl MockFileSystem {
    /// Create a new mock filesystem implementation
    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(HashMap::new())),
            directories: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Add a file to the mock filesystem
    pub fn add_file(&mut self, path: &str, content: &str) {
        let path_buf = normalize_path_to_pathbuf(path);
        self.files.lock().unwrap().insert(path_buf.clone(), content.to_string());
        
        // Create parent directories
        if let Some(parent) = path_buf.parent() {
            self.create_directory(parent);
        }
    }
    
    /// Create a directory in the mock filesystem
    pub fn create_directory(&mut self, path: &Path) {
        let path_buf = normalize_path_to_pathbuf(&path.to_string_lossy());
        self.directories.lock().unwrap().insert(path_buf.clone(), true);
        
        // Create parent directories
        if let Some(parent) = path_buf.parent() {
            if parent.to_string_lossy() != "/" {
                self.create_directory(parent);
            }
        }
    }
    
    /// Check if a file exists in the mock filesystem
    pub fn file_exists(&self, path: &str) -> bool {
        let path_buf = normalize_path_to_pathbuf(path);
        self.files.lock().unwrap().contains_key(&path_buf)
    }
    
    /// Check if a directory exists in the mock filesystem
    pub fn dir_exists(&self, path: &str) -> bool {
        let path_buf = normalize_path_to_pathbuf(path);
        self.directories.lock().unwrap().contains_key(&path_buf)
    }
    
    /// Read a file from the mock filesystem
    pub fn read_file(&self, path: &Path) -> Result<String> {
        let path_buf = normalize_path_to_pathbuf(&path.to_string_lossy());
        let files = self.files.lock().unwrap();
        
        if let Some(content) = files.get(&path_buf) {
            Ok(content.clone())
        } else {
            Err(WritingError::file_not_found(path.to_string_lossy().to_string()))
        }
    }
    
    /// Write a file to the mock filesystem
    pub fn write_file(&mut self, path: &str, content: &str) -> Result<()> {
        self.add_file(path, content);
        Ok(())
    }
    
    /// Delete a file from the mock filesystem
    pub fn delete_file(&mut self, path: &str) -> Result<()> {
        let path_buf = normalize_path_to_pathbuf(path);
        let mut files = self.files.lock().unwrap();
        
        if files.remove(&path_buf).is_some() {
            Ok(())
        } else {
            Err(WritingError::file_not_found(path))
        }
    }
    
    /// List files in a directory
    pub fn list_files(&self, dir_path: &str) -> Result<Vec<String>> {
        let dir_path_buf = normalize_path_to_pathbuf(dir_path);
        let files = self.files.lock().unwrap();
        
        let mut result = Vec::new();
        for path in files.keys() {
            if path.starts_with(&dir_path_buf) && path != &dir_path_buf {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        result.push(file_name_str.to_string());
                    }
                }
            }
        }
        
        Ok(result)
    }
}

/// Trait for filesystem operations
pub trait FileSystem {
    /// Check if a file exists
    fn file_exists(&self, path: &Path) -> bool;
    
    /// Check if a directory exists
    fn dir_exists(&self, path: &Path) -> bool;
    
    /// Read a file
    fn read_file(&self, path: &Path) -> Result<String>;
    
    /// Write a file
    fn write_file(&self, path: &Path, content: String) -> Result<()>;
    
    /// Delete a file
    fn delete_file(&self, path: &Path) -> Result<()>;
    
    /// List files in a directory
    fn list_files(&self, dir_path: &Path) -> Result<Vec<String>>;
}

// Implement the trait for the mock
impl FileSystem for MockFileSystem {
    fn file_exists(&self, path: &Path) -> bool {
        let path_buf = normalize_path_to_pathbuf(&path.to_string_lossy());
        self.files.lock().unwrap().contains_key(&path_buf)
    }
    
    fn dir_exists(&self, path: &Path) -> bool {
        let path_buf = normalize_path_to_pathbuf(&path.to_string_lossy());
        self.directories.lock().unwrap().contains_key(&path_buf)
    }
    
    fn read_file(&self, path: &Path) -> Result<String> {
        let path_buf = normalize_path_to_pathbuf(&path.to_string_lossy());
        let files = self.files.lock().unwrap();
        
        if let Some(content) = files.get(&path_buf) {
            Ok(content.clone())
        } else {
            Err(WritingError::file_not_found(path.to_string_lossy().to_string()))
        }
    }
    
    fn write_file(&self, path: &Path, content: String) -> Result<()> {
        let path_buf = normalize_path_to_pathbuf(&path.to_string_lossy());
        let mut files = self.files.lock().unwrap();
        
        // Create parent directories
        if let Some(parent) = path_buf.parent() {
            let mut dirs = self.directories.lock().unwrap();
            dirs.insert(parent.to_path_buf(), true);
        }
        
        files.insert(path_buf, content);
        Ok(())
    }
    
    fn delete_file(&self, path: &Path) -> Result<()> {
        let path_buf = normalize_path_to_pathbuf(&path.to_string_lossy());
        let mut files = self.files.lock().unwrap();
        
        if files.remove(&path_buf).is_some() {
            Ok(())
        } else {
            Err(WritingError::file_not_found(path))
        }
    }
    
    fn list_files(&self, dir_path: &Path) -> Result<Vec<String>> {
        let dir_path_buf = normalize_path_to_pathbuf(&dir_path.to_string_lossy());
        let files = self.files.lock().unwrap();
        
        let mut result = Vec::new();
        for path in files.keys() {
            if path.starts_with(&dir_path_buf) && path != &dir_path_buf {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        result.push(file_name_str.to_string());
                    }
                }
            }
        }
        
        Ok(result)
    }
}

// Helper function to normalize paths to PathBuf
fn normalize_path_to_pathbuf(path: &str) -> PathBuf {
    let path = path.trim();
    if path.starts_with('/') {
        PathBuf::from(path)
    } else {
        let mut path_buf = PathBuf::from("/");
        path_buf.push(path);
        path_buf
    }
} 