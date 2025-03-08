use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{Editor, Select};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Edit existing content")]
struct Args {
    /// Content slug to edit
    #[arg(short, long)]
    slug: Option<String>,

    /// Topic (optional, will search if not provided)
    #[arg(short, long)]
    topic: Option<String>,

    /// Edit frontmatter only
    #[arg(short, long)]
    frontmatter: bool,

    /// Edit content only
    #[arg(short, long)]
    content: bool,
}

#[derive(Deserialize, Debug)]
struct TopicConfig {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
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

fn find_content_path(config: &Config, slug: &str, topic: Option<&str>) -> Result<PathBuf> {
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
        let content_path = PathBuf::from(format!("{}/{}/{}/index.mdx", 
            config.content.base_dir, 
            topic_path, 
            slug
        ));
        
        if content_path.exists() {
            return Ok(content_path);
        }
        
        return Err(anyhow::anyhow!("Content not found: {}", content_path.display()));
    }
    
    // Search for content in all topics
    for (_, topic_config) in &config.content.topics {
        let content_path = PathBuf::from(format!("{}/{}/{}/index.mdx", 
            config.content.base_dir, 
            topic_config.path, 
            slug
        ));
        
        if content_path.exists() {
            return Ok(content_path);
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
                    content_list.push((topic_key.clone(), slug, content_file));
                }
            }
        }
    }
    
    Ok(content_list)
}

fn extract_frontmatter_and_content(file_content: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = file_content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid frontmatter format"));
    }
    
    let frontmatter = format!("---\n{}\n---", parts[1].trim());
    let content = parts[2].trim().to_string();
    
    Ok((frontmatter, content))
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read configuration
    let config = read_config()?;
    
    // Find content to edit
    let content_path = match &args.slug {
        Some(slug) => find_content_path(&config, slug, args.topic.as_deref())?,
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
                .with_prompt("Select content to edit")
                .items(&content_display)
                .default(0)
                .interact()?;
            
            content_list[selection].2.clone()
        }
    };
    
    // Read content file
    let file_content = fs::read_to_string(&content_path)
        .context(format!("Failed to read content: {:?}", content_path))?;
    
    // Extract frontmatter and content
    let (frontmatter, content) = extract_frontmatter_and_content(&file_content)?;
    
    // Edit based on flags
    let mut updated_frontmatter = frontmatter.clone();
    let mut updated_content = content.clone();
    
    if args.frontmatter || (!args.frontmatter && !args.content) {
        // Edit frontmatter
        if let Some(edited) = Editor::new().edit(&frontmatter)? {
            updated_frontmatter = edited;
        } else {
            println!("Frontmatter editing cancelled");
        }
    }
    
    if args.content || (!args.frontmatter && !args.content) {
        // Edit content
        if let Some(edited) = Editor::new().edit(&content)? {
            updated_content = edited;
        } else {
            println!("Content editing cancelled");
        }
    }
    
    // Check if anything changed
    if updated_frontmatter == frontmatter && updated_content == content {
        println!("No changes made");
        return Ok(());
    }
    
    // Write updated content
    let updated_file_content = format!("{}\n\n{}", updated_frontmatter, updated_content);
    fs::write(&content_path, updated_file_content)
        .context(format!("Failed to write updated content: {:?}", content_path))?;
    
    println!("{} Content updated: {}", "SUCCESS:".green().bold(), content_path.display());
    
    Ok(())
} 