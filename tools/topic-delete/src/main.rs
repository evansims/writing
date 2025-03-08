use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about = "Delete a topic and migrate its content")]
struct Args {
    /// Topic key to delete
    #[arg(short, long)]
    key: Option<String>,

    /// Target topic key for content migration
    #[arg(short, long)]
    target: Option<String>,

    /// Force deletion without confirmation
    #[arg(short, long)]
    force: bool,
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

fn move_content(base_dir: &str, source_path: &str, target_path: &str) -> Result<()> {
    let source_dir = format!("{}/{}", base_dir, source_path);
    let target_dir = format!("{}/{}", base_dir, target_path);
    
    // Create the target directory if it doesn't exist
    if !Path::new(&target_dir).exists() {
        fs::create_dir_all(&target_dir)
            .context(format!("Failed to create directory: {}", target_dir))?;
    }
    
    // Check if the source directory exists and has content
    if !Path::new(&source_dir).exists() {
        return Ok(());
    }
    
    // Move each article directory from source path to target path
    for entry in WalkDir::new(&source_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let article_name = entry.file_name().to_string_lossy().to_string();
        let source_article_path = format!("{}/{}", source_dir, article_name);
        let target_article_path = format!("{}/{}", target_dir, article_name);
        
        // Check if the target article already exists
        if Path::new(&target_article_path).exists() {
            let new_article_name = format!("{}-migrated", article_name);
            let new_target_path = format!("{}/{}", target_dir, new_article_name);
            
            println!("{} Target article already exists, renaming to {}", "WARNING:".yellow().bold(), new_article_name);
            
            // Move the article directory with a new name
            println!("{} Moving {} to {}", "INFO:".blue().bold(), source_article_path, new_target_path);
            fs::rename(&source_article_path, &new_target_path)
                .context(format!("Failed to move {} to {}", source_article_path, new_target_path))?;
        } else {
            // Move the article directory
            println!("{} Moving {} to {}", "INFO:".blue().bold(), source_article_path, target_article_path);
            fs::rename(&source_article_path, &target_article_path)
                .context(format!("Failed to move {} to {}", source_article_path, target_article_path))?;
        }
    }
    
    // Remove the source directory if it's empty
    if let Ok(entries) = fs::read_dir(&source_dir) {
        if entries.count() == 0 {
            fs::remove_dir(&source_dir)
                .context(format!("Failed to remove directory: {}", source_dir))?;
        }
    }
    
    Ok(())
}

fn update_index_md(source_path: &str, target_path: &str) -> Result<()> {
    let index_path = "index.md";
    if !Path::new(index_path).exists() {
        return Ok(());
    }
    
    let content = fs::read_to_string(index_path)
        .context(format!("Failed to read file: {}", index_path))?;
    
    // Replace paths in links
    let source_content_path = format!("content/{}/", source_path);
    let target_content_path = format!("content/{}/", target_path);
    let updated_content = content.replace(&source_content_path, &target_content_path);
    
    fs::write(index_path, updated_content)
        .context(format!("Failed to write file: {}", index_path))?;
    
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read the current configuration
    let mut config = read_config()?;
    
    // Get the topic key to delete
    let topic_key = match args.key {
        Some(k) => k,
        None => {
            // Get a list of available topics
            let topics: Vec<String> = config.content.topics.keys().cloned().collect();
            if topics.is_empty() {
                return Err(anyhow::anyhow!("No topics found in the configuration"));
            }
            
            let selection = Select::new()
                .with_prompt("Select a topic to delete")
                .items(&topics)
                .interact()?;
            
            topics[selection].clone()
        }
    };
    
    // Check if the topic exists
    if !config.content.topics.contains_key(&topic_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", topic_key));
    }
    
    // Get the topic configuration
    let topic_config = config.content.topics.get(&topic_key).unwrap().clone();
    
    // Check if the topic has content
    let topic_has_content = has_content(&config.content.base_dir, &topic_config.path);
    
    // If the topic has content, we need to migrate it
    if topic_has_content {
        println!("{} Topic '{}' has content that needs to be migrated", "WARNING:".yellow().bold(), topic_key);
        
        // Get the target topic for migration
        let target_key = match args.target {
            Some(t) => {
                // Check if the target topic exists
                if !config.content.topics.contains_key(&t) {
                    return Err(anyhow::anyhow!("Target topic with key '{}' not found", t));
                }
                t
            },
            None => {
                // Get a list of available topics (excluding the one being deleted)
                let topics: Vec<String> = config.content.topics.keys()
                    .filter(|&k| k != &topic_key)
                    .cloned()
                    .collect();
                
                if topics.is_empty() {
                    return Err(anyhow::anyhow!("No other topics available for migration"));
                }
                
                let selection = Select::new()
                    .with_prompt("Select a target topic for content migration")
                    .items(&topics)
                    .interact()?;
                
                topics[selection].clone()
            }
        };
        
        // Get the target topic configuration
        let target_config = config.content.topics.get(&target_key).unwrap().clone();
        
        // Confirm the migration
        let prompt = format!(
            "Migrate content from topic '{}' to '{}'?",
            topic_key, target_key
        );
        
        if !args.force && !Confirm::new().with_prompt(prompt).interact()? {
            return Err(anyhow::anyhow!("Operation cancelled by user"));
        }
        
        // Move the content
        move_content(&config.content.base_dir, &topic_config.path, &target_config.path)?;
        
        // Update index.md
        update_index_md(&topic_config.path, &target_config.path)?;
    } else if !args.force {
        // Confirm deletion
        let prompt = format!("Delete topic '{}'?", topic_key);
        if !Confirm::new().with_prompt(prompt).interact()? {
            return Err(anyhow::anyhow!("Operation cancelled by user"));
        }
    }
    
    // Remove the topic directory if it exists
    let topic_dir = format!("{}/{}", config.content.base_dir, topic_config.path);
    if Path::new(&topic_dir).exists() {
        if let Ok(entries) = fs::read_dir(&topic_dir) {
            if entries.count() == 0 {
                fs::remove_dir(&topic_dir)
                    .context(format!("Failed to remove directory: {}", topic_dir))?;
            }
        }
    }
    
    // Remove the topic from the configuration
    config.content.topics.remove(&topic_key);
    
    // Remove the topic's tags if they exist
    config.content.tags.remove(&topic_key);
    
    // Write the updated configuration
    write_config(&config)?;
    
    println!("{} Topic '{}' deleted successfully", "SUCCESS:".green().bold(), topic_key);
    
    Ok(())
} 