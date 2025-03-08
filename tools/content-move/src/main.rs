use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Input, Select};
use fs_extra::dir::{copy, CopyOptions};
use serde::Deserialize;
use slug::slugify;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about = "Move content by changing its slug and/or topic")]
struct Args {
    /// Current content slug
    #[arg(short, long)]
    slug: Option<String>,

    /// New slug for the content
    #[arg(short, long)]
    new_slug: Option<String>,

    /// Current topic (optional, will search if not provided)
    #[arg(short, long)]
    topic: Option<String>,

    /// New topic (optional, will move content to new topic)
    #[arg(short, long)]
    new_topic: Option<String>,
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

fn update_content_references(content_path: &Path, old_slug: &str, new_slug: &str) -> Result<()> {
    let content = fs::read_to_string(content_path)
        .context(format!("Failed to read content: {:?}", content_path))?;
    
    let updated_content = content.replace(old_slug, new_slug);
    
    if content != updated_content {
        fs::write(content_path, updated_content)
            .context(format!("Failed to write updated content: {:?}", content_path))?;
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = read_config()?;

    // Get content slug
    let (content_dir, current_topic) = match &args.slug {
        Some(slug) => find_content_dir(&config, slug, args.topic.as_deref())?,
        None => {
            // List all content and let user select
            let content_list = list_all_content(&config)?;
            if content_list.is_empty() {
                anyhow::bail!("No content found");
            }

            let items: Vec<String> = content_list
                .iter()
                .map(|(topic, slug, _)| format!("{}/{}", topic, slug))
                .collect();

            let selection = Select::new()
                .with_prompt("Select content to move")
                .items(&items)
                .interact()?;

            let (topic, slug, path) = &content_list[selection];
            (path.clone(), topic.clone())
        }
    };

    let current_slug = content_dir.file_name().unwrap().to_string_lossy().to_string();
    
    // Get new slug if not provided
    let new_slug = match args.new_slug {
        Some(s) => s,
        None => {
            let input: String = Input::new()
                .with_prompt("Enter new content slug")
                .interact_text()?;
            input
        }
    };
    
    // Store a copy of current_topic for later comparison
    let current_topic_copy = current_topic.clone();
    
    // Get new topic if provided
    let new_topic = match args.new_topic {
        Some(ref t) => {
            if !config.content.topics.contains_key(t) {
                anyhow::bail!("Topic '{}' not found", t);
            }
            t.clone()
        },
        None => current_topic,
    };
    
    // Confirm the rename
    let confirm_message = if current_topic_copy == new_topic {
        format!("Move content from '{}/{}' to '{}/{}'?", 
            current_topic_copy, current_slug, 
            new_topic, new_slug
        )
    } else {
        format!("Move content from '{}/{}' to '{}/{}'?", 
            current_topic_copy, current_slug, 
            new_topic, new_slug
        )
    };

    if !Confirm::new().with_prompt(confirm_message).interact()? {
        println!("Operation cancelled");
        return Ok(());
    }

    // Check if the content already exists at the destination
    let new_topic_config = &config.content.topics[&new_topic];
    let new_content_dir = PathBuf::from(&new_topic_config.path).join(&new_slug);
    
    if new_content_dir.exists() {
        anyhow::bail!("Content already exists at destination: {}/{}", new_topic, new_slug);
    }

    // If we're just changing the slug (same topic), rename the directory
    if current_topic_copy == new_topic {
        if current_slug == new_slug && current_topic_copy == new_topic {
            println!("No changes requested, nothing to do");
            return Ok(());
        }

        // Rename the directory
        fs::rename(&content_dir, &new_content_dir)
            .with_context(|| format!("Failed to rename content directory from '{}' to '{}'", 
                content_dir.display(), new_content_dir.display()))?;
    } else {
        // We're moving to a different topic, so copy the directory and then remove the original
        let mut options = CopyOptions::new();
        options.overwrite = false;
        
        // Create parent directory if it doesn't exist
        if !new_content_dir.parent().unwrap().exists() {
            fs::create_dir_all(new_content_dir.parent().unwrap())
                .with_context(|| format!("Failed to create directory: {}", new_content_dir.parent().unwrap().display()))?;
        }
        
        // Copy the directory
        copy(&content_dir, new_content_dir.parent().unwrap(), &options)
            .with_context(|| format!("Failed to copy content from '{}' to '{}'", 
                content_dir.display(), new_content_dir.parent().unwrap().display()))?;
        
        // Rename the copied directory if needed
        let copied_dir = new_content_dir.parent().unwrap().join(&current_slug);
        if current_slug != new_slug {
            fs::rename(&copied_dir, &new_content_dir)
                .with_context(|| format!("Failed to rename copied content directory from '{}' to '{}'", 
                    copied_dir.display(), new_content_dir.display()))?;
        }
        
        // Remove the original directory
        fs::remove_dir_all(&content_dir)
            .with_context(|| format!("Failed to remove original content directory: {}", content_dir.display()))?;
    }
    
    // Update references to the content in other content files
    update_content_references(&new_content_dir, &current_slug, &new_slug)?;
    
    println!("{} Content moved from '{}/{}' to '{}/{}'", 
        "SUCCESS:".green().bold(), 
        current_topic_copy, current_slug, 
        new_topic, new_slug
    );
    
    Ok(())
} 