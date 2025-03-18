//! Configuration mocks for testing
//!
//! This module provides mock implementations of configuration operations for testing.

use std::path::{Path, PathBuf};
use mockall::mock;
use common_errors::Result;
use common_config::Config;

/// The ConfigLoader trait defines operations for loading configuration
#[mockall::automock]
pub trait ConfigLoader {
    /// Load configuration from the default location
    fn load_config(&self) -> Result<Config>;

    /// Load configuration from a specific path
    fn load_config_from(&self, path: &Path) -> Result<Config>;

    /// Save configuration to the default location
    fn save_config(&self, config: &Config) -> Result<()>;

    /// Save configuration to a specific path
    fn save_config_to(&self, config: &Config, path: &Path) -> Result<()>;

    /// Get the default config path
    fn get_default_config_path(&self) -> PathBuf;
}

/// A test implementation of ConfigLoader that operates in memory
pub struct InMemoryConfigLoader {
    config: Config,
    default_path: PathBuf,
    configs: std::collections::HashMap<PathBuf, Config>,
}

impl InMemoryConfigLoader {
    /// Create a new in-memory config loader with default configuration
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            default_path: PathBuf::from(".writing/config.yml"),
            configs: std::collections::HashMap::new(),
        }
    }

    /// Create a new in-memory config loader with specific configuration
    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            default_path: PathBuf::from(".writing/config.yml"),
            configs: std::collections::HashMap::new(),
        }
    }

    /// Set the default config path
    pub fn with_default_path(mut self, path: PathBuf) -> Self {
        self.default_path = path;
        self
    }

    /// Add a config at a specific path
    pub fn add_config_at(&mut self, path: PathBuf, config: Config) {
        self.configs.insert(path, config);
    }
}

impl ConfigLoader for InMemoryConfigLoader {
    fn load_config(&self) -> Result<Config> {
        match self.configs.get(&self.default_path) {
            Some(config) => Ok(config.clone()),
            None => Ok(self.config.clone()),
        }
    }

    fn load_config_from(&self, path: &Path) -> Result<Config> {
        match self.configs.get(&path.to_path_buf()) {
            Some(config) => Ok(config.clone()),
            None => Err(common_errors::WritingError::not_found(&format!(
                "Config not found at path: {}", path.display()
            ))),
        }
    }

    fn save_config(&self, config: &Config) -> Result<()> {
        let mut loader = self.clone();
        loader.config = config.clone();
        loader.configs.insert(self.default_path.clone(), config.clone());
        Ok(())
    }

    fn save_config_to(&self, config: &Config, path: &Path) -> Result<()> {
        let mut loader = self.clone();
        loader.configs.insert(path.to_path_buf(), config.clone());
        Ok(())
    }

    fn get_default_config_path(&self) -> PathBuf {
        self.default_path.clone()
    }
}

impl Clone for InMemoryConfigLoader {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            default_path: self.default_path.clone(),
            configs: self.configs.clone(),
        }
    }
}

/// Helper function to create a config loader with test configuration
pub fn create_test_config_loader() -> InMemoryConfigLoader {
    let mut config = Config::default();

    // Set up some test configuration values
    config.content_directory = PathBuf::from("content");
    config.templates_directory = PathBuf::from("templates");
    config.topics = vec![
        "blog".to_string(),
        "tutorials".to_string(),
        "guides".to_string(),
    ];

    InMemoryConfigLoader::with_config(config)
}