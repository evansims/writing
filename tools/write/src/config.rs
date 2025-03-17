use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub site: SiteConfig,
    pub topics: Vec<TopicConfig>,
    pub images: ImageConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub author: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicConfig {
    pub key: String,
    pub name: String,
    pub description: String,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageConfig {
    pub formats: Vec<String>,
    pub format_descriptions: Option<HashMap<String, String>>,
    pub sizes: Vec<u32>,
}

pub fn load_config() -> Result<Config> {
    // Try to find config.yaml in the current directory or parent directories
    let mut current_dir = std::env::current_dir()?;
    let config_filename = "config.yaml";
    let mut config_path = current_dir.join(config_filename);
    
    // Keep going up the directory tree until we find config.yaml or reach the root
    while !config_path.exists() {
        if !current_dir.pop() {
            // We've reached the root directory and still haven't found config.yaml
            return Err(anyhow::anyhow!("Could not find config.yaml in the current directory or any parent directory"));
        }
        config_path = current_dir.join(config_filename);
    }
    
    let config_content = fs::read_to_string(&config_path)?;
    
    // Parse the YAML content
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&config_content)?;
    
    // Extract site information
    let site = SiteConfig {
        title: yaml_value["publication"]["author"].as_str().unwrap_or("").to_string(),
        description: "".to_string(), // Default empty description
        author: yaml_value["publication"]["author"].as_str().unwrap_or("").to_string(),
        url: yaml_value["publication"]["site"].as_str().map(|s| s.to_string()),
    };
    
    // Extract topics
    let mut topics = Vec::new();
    if let Some(topics_map) = yaml_value["content"]["topics"].as_mapping() {
        for (key, value) in topics_map {
            let key_str = key.as_str().unwrap_or("").to_string();
            let name = value["name"].as_str().unwrap_or("").to_string();
            let description = value["description"].as_str().unwrap_or("").to_string();
            let path = value["path"].as_str().map(|s| s.to_string());
            
            topics.push(TopicConfig {
                key: key_str,
                name,
                description,
                path,
            });
        }
    }
    
    // Extract image formats
    let mut formats = Vec::new();
    if let Some(formats_array) = yaml_value["images"]["formats"].as_sequence() {
        for format in formats_array {
            if let Some(format_str) = format.as_str() {
                formats.push(format_str.to_string());
            }
        }
    }
    
    // Extract format descriptions if available
    let mut format_descriptions = None;
    if let Some(desc_map) = yaml_value["images"]["format_descriptions"].as_mapping() {
        let mut descriptions = HashMap::new();
        for (key, value) in desc_map {
            if let (Some(key_str), Some(value_str)) = (key.as_str(), value.as_str()) {
                descriptions.insert(key_str.to_string(), value_str.to_string());
            }
        }
        if !descriptions.is_empty() {
            format_descriptions = Some(descriptions);
        }
    }
    
    // Extract image sizes (just using a placeholder for now)
    let sizes = vec![1200, 800, 400]; // Default sizes
    
    let images = ImageConfig {
        formats,
        format_descriptions,
        sizes,
    };
    
    Ok(Config {
        site,
        topics,
        images,
    })
}

pub fn get_topics() -> Result<Vec<String>> {
    let config = load_config()?;
    let topics = config.topics.iter().map(|t| t.key.clone()).collect();
    Ok(topics)
}

#[allow(dead_code)]
pub fn get_topic_names() -> Result<Vec<String>> {
    let config = load_config()?;
    let topic_names = config.topics.iter().map(|t| t.name.clone()).collect();
    Ok(topic_names)
}

#[allow(dead_code)]
pub fn get_topic_by_key(key: &str) -> Result<Option<TopicConfig>> {
    let config = load_config()?;
    let topic = config.topics.iter().find(|t| t.key == key).cloned();
    Ok(topic)
}

#[allow(dead_code)]
pub fn get_site_url() -> Result<Option<String>> {
    let config = load_config()?;
    Ok(config.site.url)
} 