//! # Lazy Build Cache Module
//!
//! This module provides a lazy-loading build cache implementation.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::Result;

/// Lazy build cache structure
#[derive(Debug, Default)]
pub struct LazyBuildCache {
    /// Map of file paths to their last modified timestamps
    files: Arc<Mutex<HashMap<PathBuf, u64>>>,
    /// Map of file paths to their content hashes
    hashes: Arc<Mutex<HashMap<PathBuf, String>>>,
}

impl LazyBuildCache {
    /// Create a new empty lazy build cache
    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(HashMap::new())),
            hashes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a file to the cache
    pub fn add_file(&self, path: PathBuf, timestamp: u64, hash: String) -> Result<()> {
        let mut files = self.files.lock().map_err(|e| anyhow::anyhow!("Failed to lock files: {}", e))?;
        let mut hashes = self.hashes.lock().map_err(|e| anyhow::anyhow!("Failed to lock hashes: {}", e))?;

        files.insert(path.clone(), timestamp);
        hashes.insert(path, hash);

        Ok(())
    }

    /// Check if a file needs rebuilding
    pub fn needs_rebuild(&self, path: &PathBuf, timestamp: u64, hash: &str) -> Result<bool> {
        let files = self.files.lock().map_err(|e| anyhow::anyhow!("Failed to lock files: {}", e))?;
        let hashes = self.hashes.lock().map_err(|e| anyhow::anyhow!("Failed to lock hashes: {}", e))?;

        match (files.get(path), hashes.get(path)) {
            (Some(cached_time), Some(cached_hash)) => {
                Ok(*cached_time != timestamp || cached_hash != hash)
            }
            _ => Ok(true),
        }
    }

    /// Clear the cache
    pub fn clear(&self) -> Result<()> {
        let mut files = self.files.lock().map_err(|e| anyhow::anyhow!("Failed to lock files: {}", e))?;
        let mut hashes = self.hashes.lock().map_err(|e| anyhow::anyhow!("Failed to lock hashes: {}", e))?;

        files.clear();
        hashes.clear();

        Ok(())
    }
}