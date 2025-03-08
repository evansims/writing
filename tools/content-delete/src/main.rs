use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Select};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about = "Delete existing content")]
struct Args {
    /// Content slug to delete
    #[arg(short, long)]
    slug: Option<String>,

    /// Topic (optional, will search if not provided)
    #[arg(short, long)]
    topic: Option<String>,

    /// Force delete without confirmation
    #[arg(short, long)]
    force: bool,
}

#[derive(Deserialize, Debug)]
struct TopicConfig {
    name: String,
    description: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct ContentConfig {
    base_dir: String,
    topics: HashMap<String, TopicConfig>,
}

#[derive(Deserialize, Debug)]
struct Config {
    content: ContentConfig,
}

fn read_config() -> Result<Config> {
    let config_content = fs::read_to_string("config.yaml")
        .context("Failed to read config.yaml")?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse config.yaml")?;
    
    Ok(config)
}

fn find_content_dir(config: &Config, slug: &str, topic: Option<&str>) -> Result<(PathBuf, String)> {
    if let Some(topic_key) = topic {
        // Validate topic
        if !config.content.topics.contains_key(topic_key) {
            let valid_topics: Vec<String> = config.content.topics.keys()
                .map(|k| k.to_string())
                .collect();
            return Err(anyhow::anyhow!(
                "Invalid topic: {}. Valid topics are: {}", 
                topic_key, 
                valid_topics.join(", ")
            ));
        }
        
        let topic_path = &config.content.topics[topic_key].path;
        let content_dir = PathBuf::from(format!("{}/{}/{}", 
            config.content.base_dir, 
            topic_path, 
            slug
        ));
        
        if content_dir.exists() {
            return Ok((content_dir, topic_key.to_string()));
        }
        
        return Err(anyhow::anyhow!("Content not found: {}", content_dir.display()));
    }
    
    // Search for content in all topics
    for (topic_key, topic_config) in &config.content.topics {
        let content_dir = PathBuf::from(format!("{}/{}/{}", 
            config.content.base_dir, 
            topic_config.path, 
            slug
        ));
        
        if content_dir.exists() {
            return Ok((content_dir, topic_key.clone()));
        }
    }
    
    Err(anyhow::anyhow!("Content not found for slug: {}", slug))
}

fn list_all_content(config: &Config) -> Result<Vec<(String, String, PathBuf)>> {
    let mut content_list = Vec::new();
    
    for (topic_key, topic_config) in &config.content.topics {
        let topic_dir = PathBuf::from(format!("{}/{}", 
            config.content.base_dir, 
            topic_config.path
        ));
        
        if !topic_dir.exists() {
            continue;
        }
        
        for entry in fs::read_dir(topic_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let content_file = path.join("index.mdx");
                if content_file.exists() {
                    let slug = path.file_name().unwrap().to_string_lossy().to_string();
                    content_list.push((topic_key.clone(), slug, path));
                }
            }
        }
    }
    
    Ok(content_list)
}

fn extract_title_from_content(content_path: &Path) -> Result<String> {
    let content = fs::read_to_string(content_path)
        .context(format!("Failed to read content: {:?}", content_path))?;
    
    // Extract title from frontmatter
    for line in content.lines() {
        if line.starts_with("title:") {
            return Ok(line.trim_start_matches("title:").trim().trim_matches('"').to_string());
        }
    }
    
    Ok("Untitled".to_string())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read configuration
    let config = read_config()?;
    
    // Find content to delete
    let (content_dir, topic) = match &args.slug {
        Some(slug) => find_content_dir(&config, slug, args.topic.as_deref())?,
        None => {
            // List all content and let user select
            let content_list = list_all_content(&config)?;
            
            if content_list.is_empty() {
                return Err(anyhow::anyhow!("No content found"));
            }
            
            let content_display: Vec<String> = content_list.iter()
                .map(|(topic, slug, _)| format!("{}/{}", topic, slug))
                .collect();
            
            let selection = Select::new()
                .with_prompt("Select content to delete")
                .items(&content_display)
                .default(0)
                .interact()?;
            
            (
                content_list[selection].2.clone(),
                content_list[selection].0.clone()
            )
        }
    };
    
    let slug = content_dir.file_name().unwrap().to_string_lossy().to_string();
    
    // Get content title for confirmation
    let content_file = content_dir.join("index.mdx");
    let title = extract_title_from_content(&content_file)?;
    
    // Confirm deletion
    if !args.force {
        let confirm_message = format!("Delete content '{}/{}' ({})?", topic, slug, title);
        if !Confirm::new().with_prompt(confirm_message).interact()? {
            println!("Operation cancelled");
            return Ok(());
        }
    }
    
    // Delete content directory
    fs::remove_dir_all(&content_dir)
        .context(format!("Failed to delete directory: {:?}", content_dir))?;
    
    println!("{} Content deleted: {}/{} ({})", 
        "SUCCESS:".green().bold(), 
        topic, slug, title
    );
    
    Ok(())
} 