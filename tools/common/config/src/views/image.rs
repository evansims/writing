//! # Image Configuration View
//!
//! This module provides a view of the configuration specific to image management.

use common_errors::{Result, ResultExt};
use common_models::{Config, ImageSize};
use std::collections::HashMap;
use std::path::Path;

use super::ConfigView;
use crate::load_config_from_path;

/// View for image-related configuration
pub struct ImageView {
    /// The underlying configuration
    config: Config,
}

impl ImageView {
    /// Create a new image view using the default configuration
    ///
    /// # Returns
    ///
    /// A new `ImageView` instance
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    pub fn new() -> Result<Self> {
        let config = crate::load_config()?;
        Ok(Self { config })
    }

    /// Create a new image view from a specific configuration path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// A new `ImageView` instance
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    pub fn from_path(path: &Path) -> Result<Self> {
        let config = load_config_from_path(path)
            .with_context(|| format!("Failed to load config from path: {}", path.display()))?;
        Ok(Self { config })
    }

    /// Get the supported image formats
    ///
    /// # Returns
    ///
    /// A vector of supported image formats
    pub fn formats(&self) -> &Vec<String> {
        &self.config.images.formats
    }

    /// Get the description for a specific format
    ///
    /// # Arguments
    ///
    /// * `format` - The format name
    ///
    /// # Returns
    ///
    /// The format description if found, or `None` if not found
    pub fn format_description(&self, format: &str) -> Option<&String> {
        self.config.images.format_descriptions.as_ref()
            .and_then(|d| d.get(format))
    }

    /// Get the image sizes
    ///
    /// # Returns
    ///
    /// A map of size keys to size configurations
    pub fn sizes(&self) -> &HashMap<String, ImageSize> {
        &self.config.images.sizes
    }

    /// Get a specific image size configuration
    ///
    /// # Arguments
    ///
    /// * `key` - The size key
    ///
    /// # Returns
    ///
    /// The size configuration if found, or `None` if not found
    pub fn size(&self, key: &str) -> Option<&ImageSize> {
        self.config.images.sizes.get(key)
    }

    /// Get the quality setting for a specific format and size
    ///
    /// # Arguments
    ///
    /// * `format` - The format name
    /// * `size` - The size key
    ///
    /// # Returns
    ///
    /// The quality setting if found, or `None` if not found
    pub fn quality(&self, format: &str, size: &str) -> Option<u32> {
        self.config.images.quality.as_ref()
            .and_then(|q| q.get(format))
            .and_then(|s| s.get(size))
            .copied()
    }
}

impl ConfigView for ImageView {
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
  format_descriptions:
    webp: "WebP format (modern)"
    jpg: "JPEG format (compatible)"
  sizes:
    small:
      width: 400
      height: 300
      description: "Small thumbnail"
    medium:
      width: 800
      height: 600
      description: "Medium display"
  quality:
    webp:
      small: 80
      medium: 85
    jpg:
      small: 70
      medium: 75
publication:
  author: "Test Author"
  copyright: "© 2023"
  site: "https://example.com"
"#).unwrap();
        file.flush().unwrap();
        file
    }

    #[test]
    fn test_image_view() {
        let config_file = create_test_config();
        let view = ImageView::from_path(config_file.path()).unwrap();

        let formats = view.formats();
        assert_eq!(formats.len(), 2);
        assert!(formats.contains(&"webp".to_string()));
        assert!(formats.contains(&"jpg".to_string()));

        let desc = view.format_description("webp").unwrap();
        assert_eq!(desc, "WebP format (modern)");

        let sizes = view.sizes();
        assert_eq!(sizes.len(), 2);
        assert!(sizes.contains_key("small"));
        assert!(sizes.contains_key("medium"));

        let small = view.size("small").unwrap();
        assert_eq!(small.width, 400);
        assert_eq!(small.height, 300);
        assert_eq!(small.description, "Small thumbnail");

        let quality = view.quality("webp", "small").unwrap();
        assert_eq!(quality, 80);
    }
}