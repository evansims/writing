//! # Mock Config Loader Implementation
//! 
//! This module provides a mock implementation of config loading operations for testing.

use std::path::Path;
use std::sync::{Arc, Mutex};
use common_errors::Result;
use common_models::Config;

/// A mock implementation of config loading operations
#[derive(Debug, Clone)]
pub struct MockConfigLoader {
    config: Arc<Mutex<Config>>,
}

impl MockConfigLoader {
    /// Create a new mock config loader implementation
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
        }
    }
    
    /// Set the config to be returned by the loader
    pub fn set_config(&mut self, config: Config) {
        let mut current_config = self.config.lock().unwrap();
        *current_config = config;
    }
    
    /// Load the config from a path (path is ignored in the mock)
    pub fn load_config(&self, _path: &str) -> Result<Config> {
        let config = self.config.lock().unwrap();
        Ok(config.clone())
    }
}

/// Trait for config loading operations
pub trait ConfigLoader {
    /// Load the config from a path
    fn load_config(&self, path: &Path) -> Result<Config>;
}

// Implement the trait for the mock
impl ConfigLoader for MockConfigLoader {
    fn load_config(&self, _path: &Path) -> Result<Config> {
        let config = self.config.lock().unwrap();
        Ok(config.clone())
    }
} 