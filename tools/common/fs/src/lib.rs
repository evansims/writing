use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Check if a path exists
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Create a directory and all parent directories if they don't exist
pub fn create_dir_all(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    Ok(())
}

/// Write content to a file, creating the parent directory if it doesn't exist
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    
    fs::write(path, content)
        .with_context(|| format!("Failed to write to file: {}", path.display()))?;
    
    Ok(())
}

/// Read content from a file
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

/// Delete a file
pub fn delete_file(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path)
            .with_context(|| format!("Failed to delete file: {}", path.display()))?;
    }
    Ok(())
}

/// Delete a directory and all its contents
pub fn delete_dir_all(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)
            .with_context(|| format!("Failed to delete directory: {}", path.display()))?;
    }
    Ok(())
}

/// Find all directories in a path that match a specific depth
pub fn find_dirs_with_depth(base_path: &Path, min_depth: usize, max_depth: usize) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    
    for entry in WalkDir::new(base_path)
        .min_depth(min_depth)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            dirs.push(entry.path().to_path_buf());
        }
    }
    
    Ok(dirs)
}

/// Find all files in a path with a specific extension
pub fn find_files_with_extension(base_path: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
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

/// Copy a file from source to destination, creating parent directories if needed
pub fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        create_dir_all(parent)?;
    }
    
    fs::copy(source, destination)
        .with_context(|| format!("Failed to copy from {} to {}", source.display(), destination.display()))?;
    
    Ok(())
} 