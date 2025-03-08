use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Input, Confirm};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about = "Add a new topic to the writing collection")]
struct Args {
    /// Topic key (optional, will be derived from name if not provided)
    #[arg(short, long)]
    key: Option<String>,

    /// Topic name
    #[arg(short, long)]
    name: Option<String>,

    /// Topic description
    #[arg(short, long)]
    description: Option<String>,

    /// Topic path (optional, will be derived from key if not provided)
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TopicConfig {
    name: String,
    description: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContentConfig {
    base_dir: String,
    topics: HashMap<String, TopicConfig>,
    tags: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImagesConfig {
    formats: Vec<HashMap<String, String>>,
    quality: HashMap<String, u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublicationConfig {
    author: String,
    copyright: String,
    site: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    content: ContentConfig,
    images: ImagesConfig,
    publication: PublicationConfig,
}

fn read_config() -> Result<Config> {
    let config_path = "config.yaml";
    let config_content = fs::read_to_string(config_path)
        .context(format!("Failed to read config file: {}", config_path))?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse config.yaml")?;
    
    Ok(config)
}

fn write_config(config: &Config) -> Result<()> {
    let config_path = "config.yaml";
    let config_content = serde_yaml::to_string(config)
        .context("Failed to serialize config")?;
    
    // Add a comment at the top of the file
    let config_content = format!("# Writing Repository Configuration\n\n{}", config_content);
    
    fs::write(config_path, config_content)
        .context(format!("Failed to write config file: {}", config_path))?;
    
    Ok(())
}

fn create_topic_directory(base_dir: &str, path: &str) -> Result<()> {
    let dir_path = format!("{}/{}", base_dir, path);
    if !Path::new(&dir_path).exists() {
        fs::create_dir_all(&dir_path)
            .context(format!("Failed to create topic directory: {}", dir_path))?;
        println!("{} Created topic directory: {}", "INFO:".blue().bold(), dir_path);
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read the current configuration
    let mut config = read_config()?;
    
    // Get topic key (either from args or prompt)
    let name = match args.name {
        Some(n) => n,
        None => {
            Input::<String>::new()
                .with_prompt("Enter topic name (e.g., 'Mindset')")
                .interact_text()?
        }
    };
    
    // Generate key from name if not provided
    let key = match args.key {
        Some(k) => k,
        None => {
            let suggested_key = slugify(&name.to_lowercase());
            let prompt = format!("Enter topic key (default: '{}')", suggested_key);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() {
                suggested_key
            } else {
                input
            }
        }
    };
    
    // Check if topic already exists
    if config.content.topics.contains_key(&key) {
        return Err(anyhow::anyhow!("Topic with key '{}' already exists", key));
    }
    
    // Get description
    let description = match args.description {
        Some(d) => d,
        None => {
            Input::<String>::new()
                .with_prompt("Enter topic description")
                .interact_text()?
        }
    };
    
    // Get path
    let path = match args.path {
        Some(p) => p,
        None => {
            let suggested_path = key.clone();
            let prompt = format!("Enter topic path (default: '{}')", suggested_path);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() {
                suggested_path
            } else {
                input
            }
        }
    };
    
    // Create the topic configuration
    let topic_config = TopicConfig {
        name,
        description,
        path: path.clone(),
    };
    
    // Add the topic to the configuration
    config.content.topics.insert(key.clone(), topic_config);
    
    // Create the topic directory
    create_topic_directory(&config.content.base_dir, &path)?;
    
    // Write the updated configuration
    write_config(&config)?;
    
    println!("{} Topic '{}' added successfully", "SUCCESS:".green().bold(), key);
    
    // Ask if the user wants to add tags for this topic
    if Confirm::new()
        .with_prompt("Do you want to add tags for this topic?")
        .interact()?
    {
        let tags_input = Input::<String>::new()
            .with_prompt("Enter comma-separated tags")
            .interact_text()?;
        
        let tags: Vec<String> = tags_input
            .split(',')
            .map(|t| t.trim().to_string())
            .collect();
        
        if !tags.is_empty() {
            config.content.tags.insert(key.clone(), tags);
            write_config(&config)?;
            println!("{} Tags added for topic '{}'", "SUCCESS:".green().bold(), key);
        }
    }
    
    Ok(())
} 