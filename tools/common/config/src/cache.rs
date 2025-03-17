//! # Configuration Caching
//!
//! This module provides a caching mechanism for configuration to avoid repeated filesystem access.
//!
//! ## Features
//!
//! - Thread-safe configuration caching
//! - Automatic cache invalidation based on file modification time
//! - Lazy loading of configuration
//!
//! ## Example
//!
//! ```rust
//! use common_config::cache::ConfigCache;
//!
//! fn get_cached_config() -> common_errors::Result<()> {
//!     let cache = ConfigCache::global();
//!     let config = cache.get_config()?;
//!     
//!     println!("Author: {}", config.publication.author);
//!     
//!     // Subsequent calls will use the cached config
//!     let config2 = cache.get_config()?;
//!     
//!     Ok(())
//! }
//! ```

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use common_errors::{Result, WritingError};
use common_models::Config;
use std::fs;
use once_cell::sync::Lazy;

/// Configuration cache entry
struct CacheEntry {
    /// The cached configuration
    config: Config,
    /// The path to the configuration file
    path: PathBuf,
    /// The last modification time of the configuration file
    last_modified: SystemTime,
    /// The time when the cache entry was created
    created_at: SystemTime,
}

/// Configuration cache
pub struct ConfigCache {
    /// The cached configuration entry
    cache: Mutex<Option<CacheEntry>>,
    /// The maximum age of a cache entry before it's considered stale
    max_age: Duration,
    /// Whether to check for file modifications
    check_modifications: bool,
}

impl ConfigCache {
    /// Create a new configuration cache
    ///
    /// # Arguments
    ///
    /// * `max_age` - The maximum age of a cache entry before it's considered stale
    /// * `check_modifications` - Whether to check for file modifications
    ///
    /// # Returns
    ///
    /// A new configuration cache
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::cache::ConfigCache;
    /// use std::time::Duration;
    ///
    /// let cache = ConfigCache::new(Duration::from_secs(60), true);
    /// ```
    pub fn new(max_age: Duration, check_modifications: bool) -> Self {
        ConfigCache {
            cache: Mutex::new(None),
            max_age,
            check_modifications,
        }
    }
    
    /// Get a global instance of the config cache
    ///
    /// This method returns a reference to a global instance of the config cache.
    /// The instance is created once and reused for subsequent calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::ConfigCache;
    ///
    /// let cache = ConfigCache::global();
    /// ```
    pub fn global() -> &'static ConfigCache {
        static INSTANCE: Lazy<ConfigCache> = Lazy::new(|| {
            // Default to 5 minutes max age and check for modifications
            ConfigCache::new(Duration::from_secs(300), true)
        });
        
        &INSTANCE
    }
    
    /// Get the cached configuration
    ///
    /// This method returns the cached configuration if it exists and is still valid,
    /// otherwise it loads the configuration from disk and caches it.
    ///
    /// # Returns
    ///
    /// The cached configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::cache::ConfigCache;
    ///
    /// let cache = ConfigCache::global();
    /// let config = cache.get_config()?;
    /// ```
    pub fn get_config(&self) -> Result<Config> {
        let mut cache = self.cache.lock().unwrap();
        
        // Check if we have a cached entry
        if let Some(entry) = cache.as_ref() {
            // Check if the cache entry is still valid
            if self.is_cache_valid(entry) {
                return Ok(entry.config.clone());
            }
        }
        
        // Load the configuration from disk
        let (config, path) = self.load_config()?;
        
        // Get the last modification time of the configuration file
        let last_modified = fs::metadata(&path)
            .map(|m| m.modified().unwrap_or_else(|_| SystemTime::now()))
            .unwrap_or_else(|_| SystemTime::now());
        
        // Create a new cache entry
        let entry = CacheEntry {
            config: config.clone(),
            path,
            last_modified,
            created_at: SystemTime::now(),
        };
        
        // Update the cache
        *cache = Some(entry);
        
        Ok(config)
    }
    
    /// Get the cached configuration from a specific path
    ///
    /// This method returns the cached configuration if it exists and is still valid,
    /// otherwise it loads the configuration from the specified path and caches it.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// The cached configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::cache::ConfigCache;
    /// use std::path::Path;
    ///
    /// let cache = ConfigCache::global();
    /// let config = cache.get_config_from_path(Path::new("config.yaml"))?;
    /// ```
    pub fn get_config_from_path(&self, path: &Path) -> Result<Config> {
        let mut cache = self.cache.lock().unwrap();
        
        // Check if we have a cached entry for this path
        if let Some(entry) = cache.as_ref() {
            if entry.path == path && self.is_cache_valid(entry) {
                return Ok(entry.config.clone());
            }
        }
        
        // Load the configuration from the specified path
        let config = super::load_config_from_path(path)?;
        
        // Get the last modification time of the configuration file
        let last_modified = fs::metadata(path)
            .map(|m| m.modified().unwrap_or_else(|_| SystemTime::now()))
            .unwrap_or_else(|_| SystemTime::now());
        
        // Create a new cache entry
        let entry = CacheEntry {
            config: config.clone(),
            path: path.to_path_buf(),
            last_modified,
            created_at: SystemTime::now(),
        };
        
        // Update the cache
        *cache = Some(entry);
        
        Ok(config)
    }
    
    /// Clear the configuration cache
    ///
    /// This method clears the configuration cache, forcing the next call to
    /// `get_config` or `get_config_from_path` to load the configuration from disk.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::cache::ConfigCache;
    ///
    /// let cache = ConfigCache::global();
    /// cache.clear();
    /// ```
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        *cache = None;
    }
    
    /// Check if a cache entry is still valid
    ///
    /// This method checks if a cache entry is still valid based on its age and
    /// whether the configuration file has been modified since it was cached.
    ///
    /// # Arguments
    ///
    /// * `entry` - The cache entry to check
    ///
    /// # Returns
    ///
    /// `true` if the cache entry is still valid, `false` otherwise
    fn is_cache_valid(&self, entry: &CacheEntry) -> bool {
        // Check if the cache entry is too old
        if let Ok(age) = SystemTime::now().duration_since(entry.created_at) {
            if age > self.max_age {
                return false;
            }
        }
        
        // Check if the configuration file has been modified
        if self.check_modifications {
            if let Ok(metadata) = fs::metadata(&entry.path) {
                if let Ok(modified) = metadata.modified() {
                    if modified > entry.last_modified {
                        return false;
                    }
                }
            }
        }
        
        true
    }
    
    /// Load the configuration from disk
    ///
    /// This method loads the configuration from disk using the standard
    /// configuration loading mechanism.
    ///
    /// # Returns
    ///
    /// A tuple containing the loaded configuration and the path to the configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    fn load_config(&self) -> Result<(Config, PathBuf)> {
        // Try to find config.yaml in the current directory or parent directories
        let mut current_dir = std::env::current_dir()
            .map_err(|e| WritingError::config_error(format!("Failed to get current directory: {}", e)))?;
        let config_filename = "config.yaml";
        let mut config_path = current_dir.join(config_filename);
        
        // Keep going up the directory tree until we find config.yaml or reach the root
        while !config_path.exists() {
            if !current_dir.pop() {
                // We've reached the root directory and still haven't found config.yaml
                return Err(WritingError::config_error(
                    "Could not find config.yaml in the current directory or any parent directory"
                ));
            }
            config_path = current_dir.join(config_filename);
        }
        
        // Load the configuration from the found path
        let config = super::load_config_from_path(&config_path)?;
        
        Ok((config, config_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use tempfile::NamedTempFile;
    
    fn create_test_config() -> (NamedTempFile, Config) {
        // Create a temporary file with valid config
        let mut config_content = String::new();
        config_content.push_str("content:\n");
        config_content.push_str("  base_dir: ./content\n");
        config_content.push_str("  topics:\n");
        config_content.push_str("    blog:\n");
        config_content.push_str("      name: Blog\n");
        config_content.push_str("      description: Blog posts\n");
        config_content.push_str("      path: blog\n");
        config_content.push_str("images:\n");
        config_content.push_str("  formats: [webp, jpg]\n");
        config_content.push_str("  sizes:\n");
        config_content.push_str("    small:\n");
        config_content.push_str("      width: 480\n");
        config_content.push_str("      height: 320\n");
        config_content.push_str("      description: Small image\n");
        config_content.push_str("publication:\n");
        config_content.push_str("  author: Test Author\n");
        config_content.push_str("  copyright: Test Copyright\n");
        config_content.push_str("  site: https://example.com\n");

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), config_content).unwrap();

        // Load the config
        let config = super::super::load_config_from_path(temp_file.path()).unwrap();
        
        (temp_file, config)
    }
    
    #[test]
    fn test_config_cache_new() {
        let cache = ConfigCache::new(Duration::from_secs(60), true);
        assert!(cache.cache.lock().unwrap().is_none());
    }
    
    #[test]
    fn test_config_cache_global() {
        let cache1 = ConfigCache::global();
        let cache2 = ConfigCache::global();
        
        // Both references should point to the same instance
        assert_eq!(
            cache1 as *const ConfigCache,
            cache2 as *const ConfigCache
        );
    }
    
    #[test]
    fn test_config_cache_get_config_from_path() {
        let (temp_file, expected_config) = create_test_config();
        
        // Create a cache with a short max age
        let cache = ConfigCache::new(Duration::from_secs(1), true);
        
        // Get the config from the cache
        let config1 = cache.get_config_from_path(temp_file.path()).unwrap();
        
        // The config should match the expected config
        assert_eq!(config1.content.base_dir, expected_config.content.base_dir);
        assert_eq!(config1.publication.author, expected_config.publication.author);
        
        // Get the config again, it should be cached
        let config2 = cache.get_config_from_path(temp_file.path()).unwrap();
        
        // The configs should be equal
        assert_eq!(config1.content.base_dir, config2.content.base_dir);
        assert_eq!(config1.publication.author, config2.publication.author);
        
        // Wait for the cache to expire
        thread::sleep(Duration::from_secs(2));
        
        // Get the config again, it should be reloaded
        let config3 = cache.get_config_from_path(temp_file.path()).unwrap();
        
        // The configs should still be equal
        assert_eq!(config1.content.base_dir, config3.content.base_dir);
        assert_eq!(config1.publication.author, config3.publication.author);
    }
    
    #[test]
    fn test_config_cache_clear() {
        let (temp_file, _) = create_test_config();
        
        // Create a cache
        let cache = ConfigCache::new(Duration::from_secs(60), true);
        
        // Get the config from the cache
        let _ = cache.get_config_from_path(temp_file.path()).unwrap();
        
        // The cache should now have an entry
        assert!(cache.cache.lock().unwrap().is_some());
        
        // Clear the cache
        cache.clear();
        
        // The cache should now be empty
        assert!(cache.cache.lock().unwrap().is_none());
    }
    
    #[test]
    fn test_config_cache_file_modification() {
        let (temp_file, _) = create_test_config();
        
        // Create a cache that checks for file modifications
        let cache = ConfigCache::new(Duration::from_secs(60), true);
        
        // Get the config from the cache
        let config1 = cache.get_config_from_path(temp_file.path()).unwrap();
        
        // Modify the config file
        let mut config_content = String::new();
        config_content.push_str("content:\n");
        config_content.push_str("  base_dir: ./content\n");
        config_content.push_str("  topics:\n");
        config_content.push_str("    blog:\n");
        config_content.push_str("      name: Blog\n");
        config_content.push_str("      description: Blog posts\n");
        config_content.push_str("      path: blog\n");
        config_content.push_str("images:\n");
        config_content.push_str("  formats: [webp, jpg]\n");
        config_content.push_str("  sizes:\n");
        config_content.push_str("    small:\n");
        config_content.push_str("      width: 480\n");
        config_content.push_str("      height: 320\n");
        config_content.push_str("      description: Small image\n");
        config_content.push_str("publication:\n");
        config_content.push_str("  author: Modified Author\n");
        config_content.push_str("  copyright: Test Copyright\n");
        config_content.push_str("  site: https://example.com\n");
        
        // Wait a bit to ensure the modification time is different
        thread::sleep(Duration::from_millis(100));
        
        // Write the modified config
        fs::write(temp_file.path(), config_content).unwrap();
        
        // Get the config again, it should be reloaded
        let config2 = cache.get_config_from_path(temp_file.path()).unwrap();
        
        // The author should be different
        assert_ne!(config1.publication.author, config2.publication.author);
        assert_eq!(config2.publication.author, "Modified Author");
    }
} 