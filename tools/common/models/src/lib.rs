use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration structure for the entire application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub content: ContentConfig,
    pub images: ImageConfig,
    pub publication: PublicationConfig,
}

/// Configuration structure for content settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentConfig {
    pub base_dir: String,
    pub topics: HashMap<String, TopicConfig>,
    pub tags: Option<HashMap<String, Vec<String>>>,
}

/// Configuration structure for a topic
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicConfig {
    pub name: String,
    pub description: String,
    pub path: String,
}

/// Configuration structure for images
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageConfig {
    pub formats: Vec<String>,
    pub format_descriptions: Option<HashMap<String, String>>,
    pub sizes: HashMap<String, ImageSize>,
    pub naming: Option<ImageNaming>,
    pub quality: Option<HashMap<String, HashMap<String, u32>>>,
}

/// Configuration structure for an image size
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
    pub description: String,
}

/// Configuration structure for image naming
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageNaming {
    pub pattern: String,
    pub examples: Vec<String>,
}

/// Configuration structure for publication
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicationConfig {
    pub author: String,
    pub copyright: String,
    pub site: Option<String>,
}

/// Structure for article frontmatter
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub slug: Option<String>,
    pub tagline: Option<String>,
    pub tags: Option<Vec<String>>,
    pub topics: Option<Vec<String>>,
    pub draft: Option<bool>,
    pub featured_image: Option<String>,
}

/// Structure for a complete article
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub slug: String,
    pub topic: String,
    pub path: String,
    pub word_count: Option<usize>,
    pub reading_time: Option<u32>,
} 