//! # Publication Configuration View
//!
//! This module provides a view of the configuration specific to publication settings.

use common_errors::{Result, ResultExt};
use common_models::Config;
use std::path::Path;

use super::ConfigView;
use crate::load_config_from_path;

/// View for publication-related configuration
pub struct PublicationView {
    /// The underlying configuration
    config: Config,
}

impl PublicationView {
    /// Create a new publication view using the default configuration
    ///
    /// # Returns
    ///
    /// A new `PublicationView` instance
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    pub fn new() -> Result<Self> {
        let config = crate::load_config()?;
        Ok(Self { config })
    }

    /// Create a new publication view from a specific configuration path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// A new `PublicationView` instance
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    pub fn from_path(path: &Path) -> Result<Self> {
        let config = load_config_from_path(path)
            .with_context(|| format!("Failed to load config from path: {}", path.display()))?;
        Ok(Self { config })
    }

    /// Get the author name
    ///
    /// # Returns
    ///
    /// The author name as a string
    pub fn author(&self) -> &str {
        &self.config.publication.author
    }

    /// Get the copyright notice
    ///
    /// # Returns
    ///
    /// The copyright notice as a string
    pub fn copyright(&self) -> &str {
        &self.config.publication.copyright
    }

    /// Get the site URL
    ///
    /// # Returns
    ///
    /// The site URL as a string if available, or `None` if not set
    pub fn site_url(&self) -> Option<&str> {
        self.config.publication.site_url.as_deref()
    }

    /// Get the site URL (deprecated)
    ///
    /// # Returns
    ///
    /// The site URL as a string if available, or `None` if not set
    ///
    /// # Deprecated
    ///
    /// This method is deprecated. Use [`site_url`](Self::site_url) instead.
    #[deprecated(since = "1.1.0", note = "Use site_url() instead")]
    pub fn site(&self) -> Option<&str> {
        self.site_url()
    }
}

impl ConfigView for PublicationView {
    fn config(&self) -> &Config {
        &self.config
    }

    fn from_config(config: Config) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_config() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, r#"
content:
  base_dir: "./content"
  topics:
    blog:
      directory: "blog"
      name: "Blog"
      description: "Blog posts"
images:
  formats:
    - "webp"
    - "jpg"
  sizes:
    small:
      width: 400
      height: 300
      description: "Small thumbnail"
    medium:
      width: 800
      height: 600
      description: "Medium display"
publication:
  author: "Test Author"
  copyright: "© 2023"
  site: "https://example.com"
"#).unwrap();
        file.flush().unwrap();
        file
    }

    #[test]
    fn test_publication_view() {
        let config_file = create_test_config();
        let view = PublicationView::from_path(config_file.path()).unwrap();

        assert_eq!(view.author(), "Test Author");
        assert_eq!(view.copyright(), "© 2023");
        assert_eq!(view.site_url(), Some("https://example.com"));
    }
}