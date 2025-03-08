use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Input, Select, Confirm};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about = "Rename a topic and move its content")]
struct Args {
    /// Current topic key
    #[arg(short, long)]
    key: Option<String>,

    /// New topic key
    #[arg(short, long)]
    new_key: Option<String>,

    /// New topic name
    #[arg(short, long)]
    new_name: Option<String>,

    /// New topic path
    #[arg(short, long)]
    new_path: Option<String>,
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

fn has_content(base_dir: &str, path: &str) -> bool {
    let dir_path = format!("{}/{}", base_dir, path);
    let path = Path::new(&dir_path);
    
    if !path.exists() {
        return false;
    }
    
    // Check if there are any subdirectories (articles)
    WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .next()
        .is_some()
}

fn move_content(base_dir: &str, old_path: &str, new_path: &str) -> Result<()> {
    let old_dir = format!("{}/{}", base_dir, old_path);
    let new_dir = format!("{}/{}", base_dir, new_path);
    
    // Create the new directory if it doesn't exist
    if !Path::new(&new_dir).exists() {
        fs::create_dir_all(&new_dir)
            .context(format!("Failed to create directory: {}", new_dir))?;
    }
    
    // Check if the old directory exists and has content
    if !Path::new(&old_dir).exists() {
        return Ok(());
    }
    
    // Move each article directory from old path to new path
    for entry in WalkDir::new(&old_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let article_name = entry.file_name().to_string_lossy().to_string();
        let old_article_path = format!("{}/{}", old_dir, article_name);
        let new_article_path = format!("{}/{}", new_dir, article_name);
        
        // Move the article directory
        println!("{} Moving {} to {}", "INFO:".blue().bold(), old_article_path, new_article_path);
        fs::rename(&old_article_path, &new_article_path)
            .context(format!("Failed to move {} to {}", old_article_path, new_article_path))?;
    }
    
    // Remove the old directory if it's empty
    if let Ok(entries) = fs::read_dir(&old_dir) {
        if entries.count() == 0 {
            fs::remove_dir(&old_dir)
                .context(format!("Failed to remove directory: {}", old_dir))?;
        }
    }
    
    Ok(())
}

fn update_index_md(old_topic_key: &str, new_topic_key: &str, old_path: &str, new_path: &str) -> Result<()> {
    let index_path = "index.md";
    if !Path::new(index_path).exists() {
        return Ok(());
    }
    
    let content = fs::read_to_string(index_path)
        .context(format!("Failed to read file: {}", index_path))?;
    
    // Replace paths in links
    let old_content_path = format!("content/{}/", old_path);
    let new_content_path = format!("content/{}/", new_path);
    let updated_content = content.replace(&old_content_path, &new_content_path);
    
    fs::write(index_path, updated_content)
        .context(format!("Failed to write file: {}", index_path))?;
    
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read the current configuration
    let mut config = read_config()?;
    
    // Get the current topic key
    let current_key = match args.key {
        Some(k) => k,
        None => {
            // Get a list of available topics
            let topics: Vec<String> = config.content.topics.keys().cloned().collect();
            if topics.is_empty() {
                return Err(anyhow::anyhow!("No topics found in the configuration"));
            }
            
            let selection = Select::new()
                .with_prompt("Select a topic to rename")
                .items(&topics)
                .interact()?;
            
            topics[selection].clone()
        }
    };
    
    // Check if the topic exists
    if !config.content.topics.contains_key(&current_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", current_key));
    }
    
    // Get the current topic configuration
    let current_topic = config.content.topics.get(&current_key).unwrap().clone();
    
    // Get the new topic key
    let new_key = match args.new_key {
        Some(k) => k,
        None => {
            let suggested_key = if let Some(new_name) = &args.new_name {
                slugify(&new_name.to_lowercase())
            } else {
                current_key.clone()
            };
            
            let prompt = format!("Enter new topic key (default: '{}')", suggested_key);
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
    
    // Check if the new key already exists and is different from the current key
    if new_key != current_key && config.content.topics.contains_key(&new_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' already exists", new_key));
    }
    
    // Get the new topic name
    let new_name = match args.new_name {
        Some(n) => n,
        None => {
            let prompt = format!("Enter new topic name (current: '{}')", current_topic.name);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() {
                current_topic.name.clone()
            } else {
                input
            }
        }
    };
    
    // Get the new topic path
    let new_path = match args.new_path {
        Some(p) => p,
        None => {
            let suggested_path = if new_key != current_key {
                new_key.clone()
            } else {
                current_topic.path.clone()
            };
            
            let prompt = format!("Enter new topic path (default: '{}')", suggested_path);
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
    
    // Check if the topic has content
    let has_content = has_content(&config.content.base_dir, &current_topic.path);
    
    // If the path is changing and there is content, confirm the move
    if current_topic.path != new_path && has_content {
        let prompt = format!(
            "Topic '{}' has content. Move content from '{}' to '{}'?",
            current_key, current_topic.path, new_path
        );
        
        if !Confirm::new().with_prompt(prompt).interact()? {
            return Err(anyhow::anyhow!("Operation cancelled by user"));
        }
        
        // Move the content
        move_content(&config.content.base_dir, &current_topic.path, &new_path)?;
        
        // Update index.md
        update_index_md(&current_key, &new_key, &current_topic.path, &new_path)?;
    }
    
    // Create the new topic configuration
    let new_topic = TopicConfig {
        name: new_name,
        description: current_topic.description.clone(),
        path: new_path,
    };
    
    // Update the configuration
    if current_key != new_key {
        // Remove the old topic
        config.content.topics.remove(&current_key);
        
        // Move tags if they exist
        if let Some(tags) = config.content.tags.remove(&current_key) {
            config.content.tags.insert(new_key.clone(), tags);
        }
    }
    
    // Add the new topic
    config.content.topics.insert(new_key.clone(), new_topic);
    
    // Write the updated configuration
    write_config(&config)?;
    
    println!("{} Topic renamed from '{}' to '{}'", "SUCCESS:".green().bold(), current_key, new_key);
    
    Ok(())
} 