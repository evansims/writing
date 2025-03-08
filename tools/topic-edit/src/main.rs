use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[command(author, version, about = "Update an existing topic in the writing collection")]
struct Args {
    /// Topic key to update
    #[arg(short, long)]
    key: Option<String>,

    /// New topic name
    #[arg(short, long)]
    name: Option<String>,

    /// New topic description
    #[arg(short, long)]
    description: Option<String>,
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

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read the current configuration
    let mut config = read_config()?;
    
    // Get the topic key to update
    let key = match args.key {
        Some(k) => k,
        None => {
            // Get a list of available topics
            let topics: Vec<String> = config.content.topics.keys().cloned().collect();
            if topics.is_empty() {
                return Err(anyhow::anyhow!("No topics found in the configuration"));
            }
            
            let selection = Select::new()
                .with_prompt("Select a topic to update")
                .items(&topics)
                .interact()?;
            
            topics[selection].clone()
        }
    };
    
    // Check if the topic exists
    if !config.content.topics.contains_key(&key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", key));
    }
    
    // Get the current topic configuration
    let mut topic_config = config.content.topics.get(&key).unwrap().clone();
    
    // Update the name if provided
    if let Some(name) = args.name {
        topic_config.name = name;
    } else {
        let current_name = &topic_config.name;
        let prompt = format!("Enter new name (current: '{}')", current_name);
        let input = Input::<String>::new()
            .with_prompt(&prompt)
            .allow_empty(true)
            .interact_text()?;
        
        if !input.is_empty() {
            topic_config.name = input;
        }
    }
    
    // Update the description if provided
    if let Some(description) = args.description {
        topic_config.description = description;
    } else {
        let current_description = &topic_config.description;
        let prompt = format!("Enter new description (current: '{}')", current_description);
        let input = Input::<String>::new()
            .with_prompt(&prompt)
            .allow_empty(true)
            .interact_text()?;
        
        if !input.is_empty() {
            topic_config.description = input;
        }
    }
    
    // Update the topic configuration
    config.content.topics.insert(key.clone(), topic_config);
    
    // Write the updated configuration
    write_config(&config)?;
    
    println!("{} Topic '{}' updated successfully", "SUCCESS:".green().bold(), key);
    
    Ok(())
} 