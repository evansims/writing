use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Select};
use topic_delete::{TopicDeleteOptions, delete_topic, topic_exists, has_content, get_topic_keys_except};
use common_config::load_config;

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

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read the current configuration for interactive selection
    let config = load_config()?;
    
    // Get the topic key to delete (interactive if not provided)
    let key = match args.key {
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
    if !topic_exists(&config, &key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", key));
    }
    
    // Get the topic configuration
    let topic_config = config.content.topics.get(&key).unwrap().clone();
    
    // Check if the topic has content
    let topic_has_content = has_content(&config.content.base_dir, &topic_config.directory);
    
    // Initialize target variable for possible content migration
    let mut target = args.target.clone();
    
    // If the topic has content, we need to migrate it
    if topic_has_content {
        println!("{} Topic '{}' has content that needs to be migrated", "WARNING:".yellow().bold(), key);
        
        // Get the target topic for migration (interactive if not provided)
        if target.is_none() {
            // Get a list of available topics (excluding the one being deleted)
            let topics = get_topic_keys_except(&config, &key);
            
            if topics.is_empty() {
                return Err(anyhow::anyhow!("No other topics available for migration"));
            }
            
            let selection = Select::new()
                .with_prompt("Select a target topic for content migration")
                .items(&topics)
                .interact()?;
            
            target = Some(topics[selection].clone());
        }
        
        // Get the target topic configuration
        let target_key = target.as_ref().unwrap();
        if !topic_exists(&config, target_key) {
            return Err(anyhow::anyhow!("Target topic with key '{}' not found", target_key));
        }
        
        let target_config = config.content.topics.get(target_key).unwrap();
        
        // Show migration summary
        println!("Will migrate content from topic '{}' to '{}'", key.yellow(), target_key.yellow());
        println!("From directory: {}", topic_config.directory.yellow());
        println!("To directory: {}", target_config.directory.yellow());
    }
    
    // Confirm deletion unless force flag is set
    if !args.force {
        let prompt = format!("Delete topic '{}'?", key);
        if !Confirm::new().with_prompt(prompt).default(false).interact()? {
            println!("Operation cancelled");
            return Ok(());
        }
    }
    
    // Create options
    let options = TopicDeleteOptions {
        key: Some(key),
        target,
        force: args.force,
    };
    
    // Delete the topic
    match delete_topic(&options) {
        Ok(topic_key) => {
            println!("{} Topic '{}' deleted successfully", "SUCCESS:".green().bold(), topic_key);
            Ok(())
        },
        Err(e) => Err(e),
    }
} 