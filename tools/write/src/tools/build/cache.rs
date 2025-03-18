//! # Build Cache Module
//!
//! This module provides caching functionality for the build process.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// Build cache structure
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildCache {
    /// Map of file paths to their last modified timestamps
    pub files: HashMap<PathBuf, u64>,
    /// Map of file paths to their content hashes
    pub hashes: HashMap<PathBuf, String>,
}

impl BuildCache {
    /// Create a new empty build cache
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            hashes: HashMap::new(),
        }
    }

    /// Add a file to the cache
    pub fn add_file(&mut self, path: PathBuf, timestamp: u64, hash: String) {
        self.files.insert(path.clone(), timestamp);
        self.hashes.insert(path, hash);
    }

    /// Check if a file needs rebuilding
    pub fn needs_rebuild(&self, path: &PathBuf, timestamp: u64, hash: &str) -> bool {
        match (self.files.get(path), self.hashes.get(path)) {
            (Some(cached_time), Some(cached_hash)) => {
                *cached_time != timestamp || cached_hash != hash
            }
            _ => true,
        }
    }
}

impl Default for BuildCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the build cache from disk
pub fn get_build_cache() -> Result<BuildCache> {
    let cache_file = PathBuf::from(".write-cache/build.json");
    if cache_file.exists() {
        let contents = fs::read_to_string(cache_file)?;
        Ok(serde_json::from_str(&contents)?)
    } else {
        Ok(BuildCache::new())
    }
}

/// Save the build cache to disk
pub fn save_build_cache(cache: &BuildCache) -> Result<()> {
    let cache_dir = PathBuf::from(".write-cache");
    fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join("build.json");
    let contents = serde_json::to_string_pretty(cache)?;
    fs::write(cache_file, contents)?;
    Ok(())
}

/// Clear the build cache from disk
pub fn clear_build_cache() -> Result<()> {
    let cache_file = PathBuf::from(".write-cache/build.json");
    if cache_file.exists() {
        fs::remove_file(cache_file)?;
    }
    Ok(())
}