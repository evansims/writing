//! # Configuration Views
//!
//! This module provides specialized views of the configuration,
//! allowing access to specific sections of the configuration
//! in a type-safe and convenient way.
//!
//! ## Available Views
//!
//! - `ContentView`: Access content-related configuration
//! - `ImageView`: Access image-related configuration
//! - `PublicationView`: Access publication-related configuration
//!
//! ## Example
//!
//! ```rust
//! use common_config::views::ContentView;
//!
//! fn get_content_config() -> common_errors::Result<()> {
//!     let view = ContentView::new()?;
//!
//!     // Access content-specific configuration
//!     let base_dir = view.base_dir();
//!     let topics = view.topics();
//!
//!     Ok(())
//! }
//! ```

use common_models::Config;

mod content;
mod image;
mod publication;

pub use content::ContentView;
pub use image::ImageView;
pub use publication::PublicationView;

/// Trait for configuration views
///
/// This trait defines common methods for all configuration views,
/// allowing access to the underlying configuration and creation
/// from an existing configuration.
pub trait ConfigView {
    /// Get the underlying configuration
    ///
    /// # Returns
    ///
    /// The underlying configuration
    fn config(&self) -> &Config;

    /// Create a view from an existing configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use
    ///
    /// # Returns
    ///
    /// A new view instance
    fn from_config(config: Config) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_errors::Result;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_config() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, r#"
content:
  base_dir: "./content"
  topics:
    blog:
      path: "blog"
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
publication:
  author: "Test Author"
  copyright: "© 2023"
  site: "https://example.com"
"#).unwrap();
        file.flush().unwrap();
        file
    }

    #[test]
    fn test_content_view() -> Result<()> {
        let config_file = create_test_config();
        let view = ContentView::from_path(config_file.path())?;

        assert_eq!(view.base_dir(), "./content");
        assert!(view.topics().contains_key("blog"));

        Ok(())
    }

    #[test]
    fn test_image_view() -> Result<()> {
        let config_file = create_test_config();
        let view = ImageView::from_path(config_file.path())?;

        assert!(view.formats().contains(&"webp".to_string()));
        assert!(view.sizes().contains_key("small"));

        Ok(())
    }

    #[test]
    fn test_publication_view() -> Result<()> {
        let config_file = create_test_config();
        let view = PublicationView::from_path(config_file.path())?;

        assert_eq!(view.author(), "Test Author");
        assert_eq!(view.copyright(), "© 2023");

        Ok(())
    }
}