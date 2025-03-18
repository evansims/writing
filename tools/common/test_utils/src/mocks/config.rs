//! Configuration mocks for testing
//!
//! This module provides mock implementations of configuration operations for testing.

use std::path::{Path, PathBuf};
use mockall::mock;
use common_errors::Result;
use common_models::{Config, TopicConfig};
use mockall::predicate::*;
use std::collections::HashMap;

/// The ConfigLoader trait defines operations for loading configuration
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

/// Mock implementation of ConfigLoader for testing
mock! {
    pub ConfigLoader {}
    impl ConfigLoader for ConfigLoader {
        fn load_config(&self) -> Result<Config>;
        fn load_config_from(&self, path: &Path) -> Result<Config>;
        fn save_config(&self, config: &Config) -> Result<()>;
        fn save_config_to(&self, config: &Config, path: &Path) -> Result<()>;
        fn get_default_config_path(&self) -> PathBuf;
    }
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
            None => Err(common_errors::WritingError::config_error(
                format!("Config not found at path: {}", path.display())
            )),
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
    // Add basic config values that match the fields in the Config struct
    config.title = "Test Site".to_string();
    config.email = "test@example.com".to_string();
    config.url = "https://example.com".to_string();

    // Add topic configuration to content
    let mut topics = std::collections::HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        directory: "blog".to_string(),
        description: "Blog posts".to_string(),
    });
    topics.insert("tutorials".to_string(), TopicConfig {
        name: "Tutorials".to_string(),
        directory: "tutorials".to_string(),
        description: "Tutorial articles".to_string(),
    });

    config.content.topics = topics;

    InMemoryConfigLoader::with_config(config)
}

/// Set up a default test configuration
fn default_test_config() -> Config {
    let mut config = Config::default();

    // Set basic configuration values
    config.title = "Test Site".to_string();
    config.email = "test@example.com".to_string();
    config.url = "https://example.com".to_string();

    // Create topics
    let mut topics = HashMap::new();

    // Blog topic
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        directory: "blog".to_string(),
        description: "Blog posts".to_string(),
    });

    // Tutorials topic
    topics.insert("tutorials".to_string(), TopicConfig {
        name: "Tutorials".to_string(),
        directory: "tutorials".to_string(),
        description: "Tutorial articles".to_string(),
    });

    // Set topics in content config
    config.content.topics = topics;

    config
}