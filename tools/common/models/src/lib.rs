//! # Models Module
//!
//! This module provides data structures for configuration and content.
//!
//! ## Features
//!
//! - Configuration structures for the application
//! - Content structures for articles and frontmatter
//! - Serialization and deserialization support
//!
//! ## Example
//!
//! ```rust
//! use common_models::{Config, ContentConfig, TopicConfig};
//! use std::collections::HashMap;
//!
//! // Create a simple configuration
//! let mut topics = HashMap::new();
//! topics.insert(
//!     "blog".to_string(),
//!     TopicConfig {
//!         name: "Blog".to_string(),
//!         description: "Blog posts".to_string(),
//!         path: "blog".to_string(),
//!     },
//! );
//!
//! let content_config = ContentConfig {
//!     base_dir: "/content".to_string(),
//!     topics,
//!     tags: None,
//! };
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Include external test module
#[cfg(test)]
mod tests;

/// Configuration structure for the entire application
///
/// This struct contains all configuration for the application,
/// including content, images, and publication settings.
///
/// # Example
///
/// ```rust
/// use common_models::{Config, ContentConfig, ImageConfig, PublicationConfig};
/// use std::collections::HashMap;
///
/// let config = Config {
///     content: ContentConfig {
///         base_dir: "/content".to_string(),
///         topics: HashMap::new(),
///         tags: None,
///     },
///     images: ImageConfig {
///         formats: vec!["jpg".to_string()],
///         format_descriptions: None,
///         sizes: HashMap::new(),
///         naming: None,
///         quality: None,
///     },
///     publication: PublicationConfig {
///         author: "Author".to_string(),
///         copyright: "Copyright".to_string(),
///         site_url: None,
///     },
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Content configuration settings
    pub content: ContentConfig,
    /// Image configuration settings
    pub images: ImageConfig,
    /// Publication configuration settings
    pub publication: PublicationConfig,
}

/// Configuration structure for content settings
///
/// This struct contains all configuration related to content,
/// including topics, tags, and base directory.
///
/// # Example
///
/// ```rust
/// use common_models::{ContentConfig, TopicConfig};
/// use std::collections::HashMap;
///
/// let mut topics = HashMap::new();
/// topics.insert(
///     "blog".to_string(),
///     TopicConfig {
///         name: "Blog".to_string(),
///         description: "Blog posts".to_string(),
///         path: "blog".to_string(),
///     },
/// );
///
/// let content_config = ContentConfig {
///     base_dir: "/content".to_string(),
///     topics,
///     tags: None,
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentConfig {
    /// Base directory for content files
    pub base_dir: String,
    /// Map of topic IDs to topic configurations
    pub topics: HashMap<String, TopicConfig>,
    /// Optional map of tag categories to tags
    pub tags: Option<HashMap<String, Vec<String>>>,
}

/// Configuration structure for a topic
///
/// This struct contains configuration for a single topic,
/// including its name, description, and directory.
///
/// # Example
///
/// ```rust
/// use common_models::TopicConfig;
///
/// let topic = TopicConfig {
///     name: "Blog".to_string(),
///     description: "Blog posts".to_string(),
///     directory: "blog".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicConfig {
    /// Display name of the topic
    pub name: String,
    /// Description of the topic
    pub description: String,
    /// Path to the topic directory
    pub directory: String,
}

/// Configuration structure for images
///
/// This struct contains all configuration related to images,
/// including formats, sizes, naming, and quality settings.
///
/// # Example
///
/// ```rust
/// use common_models::{ImageConfig, ImageSize};
/// use std::collections::HashMap;
///
/// let mut sizes = HashMap::new();
/// sizes.insert(
///     "small".to_string(),
///     ImageSize {
///         width_px: 480,
///         height_px: 320,
///         description: "Small image".to_string(),
///     },
/// );
///
/// let image_config = ImageConfig {
///     formats: vec!["jpg".to_string(), "webp".to_string()],
///     format_descriptions: None,
///     sizes,
///     naming: None,
///     quality: None,
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageConfig {
    /// List of supported image formats
    pub formats: Vec<String>,
    /// Optional map of format IDs to descriptions
    pub format_descriptions: Option<HashMap<String, String>>,
    /// Map of size IDs to size configurations
    pub sizes: HashMap<String, ImageSize>,
    /// Optional naming configuration
    pub naming: Option<ImageNaming>,
    /// Optional quality settings for different formats and sizes
    pub quality: Option<HashMap<String, HashMap<String, u32>>>,
}

/// Configuration structure for image size
///
/// This struct contains the width, height, and description of an image size.
///
/// # Example
///
/// ```rust
/// use common_models::ImageSize;
///
/// let size = ImageSize {
///     width: 1200,
///     height: 630,
///     description: "Featured image".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageSize {
    /// Width of the image in pixels
    pub width: u32,
    /// Height of the image in pixels
    pub height: u32,
    /// Description of the image size
    pub description: String,
}

/// Configuration structure for image naming
///
/// This struct contains configuration for image naming,
/// including the pattern and examples.
///
/// # Example
///
/// ```rust
/// use common_models::ImageNaming;
///
/// let naming = ImageNaming {
///     pattern: "{slug}-{size}.{format}".to_string(),
///     examples: vec!["post-small.jpg".to_string()],
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageNaming {
    /// Pattern for image filenames
    pub pattern: String,
    /// Examples of image filenames
    pub examples: Vec<String>,
}

/// Configuration structure for publication settings
///
/// This struct contains the author, copyright, and site URL for the publication.
///
/// # Example
///
/// ```rust
/// use common_models::PublicationConfig;
///
/// let publication = PublicationConfig {
///     author: "Author".to_string(),
///     copyright: "Copyright".to_string(),
///     site_url: None,
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicationConfig {
    /// Author name
    pub author: String,
    /// Copyright notice
    pub copyright: String,
    /// Optional site URL
    #[serde(rename = "site")]
    pub site_url: Option<String>,
}

/// Frontmatter metadata for articles
///
/// This struct contains metadata for an article, such as title,
/// publication date, tags, etc.
///
/// # Example
///
/// ```rust
/// use common_models::Frontmatter;
///
/// let frontmatter = Frontmatter {
///     title: "Article Title".to_string(),
///     published_at: Some("2023-01-01".to_string()),
///     updated_at: None,
///     slug: Some("article-slug".to_string()),
///     tagline: Some("Article tagline".to_string()),
///     tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
///     topics: Some(vec!["topic1".to_string()]),
///     is_draft: Some(false),
///     featured_image_path: Some("images/article.jpg".to_string()),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frontmatter {
    /// Title of the article
    pub title: String,
    /// Optional publication date
    #[serde(rename = "published")]
    pub published_at: Option<String>,
    /// Optional last updated date
    #[serde(rename = "updated")]
    pub updated_at: Option<String>,
    /// Optional slug for the article
    pub slug: Option<String>,
    /// Optional tagline or subtitle
    pub tagline: Option<String>,
    /// Optional list of tags
    pub tags: Option<Vec<String>>,
    /// Optional list of topics
    pub topics: Option<Vec<String>>,
    /// Optional draft status
    #[serde(rename = "draft")]
    pub is_draft: Option<bool>,
    /// Optional featured image path
    #[serde(rename = "featured_image")]
    pub featured_image_path: Option<String>,
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            published_at: None,
            updated_at: None,
            slug: None,
            tagline: None,
            tags: None,
            topics: None,
            is_draft: Some(true),
            featured_image_path: None,
        }
    }
}

/// Structure for a complete article
///
/// This struct contains all information about an article,
/// including frontmatter, content, and metadata.
///
/// # Example
///
/// ```rust
/// use common_models::{Article, Frontmatter};
///
/// let article = Article {
///     frontmatter: Frontmatter {
///         title: "My First Post".to_string(),
///         published_at: Some("2023-01-01".to_string()),
///         updated_at: None,
///         slug: Some("my-first-post".to_string()),
///         tagline: None,
///         tags: Some(vec!["intro".to_string()]),
///         topics: Some(vec!["blog".to_string()]),
///         is_draft: Some(false),
///         featured_image_path: None,
///     },
///     content: "# My First Post\n\nThis is my first blog post.".to_string(),
///     slug: "my-first-post".to_string(),
///     topic: "blog".to_string(),
///     path: "/content/blog/my-first-post/index.mdx".to_string(),
///     word_count: Some(7),
///     reading_time: Some(1),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    /// Frontmatter metadata
    pub frontmatter: Frontmatter,
    /// Article content in Markdown format
    pub content: String,
    /// Slug for the article
    pub slug: String,
    /// Topic the article belongs to
    pub topic: String,
    /// Path to the article file
    pub path: String,
    /// Optional word count
    pub word_count: Option<usize>,
    /// Optional reading time in minutes
    pub reading_time: Option<u32>,
}