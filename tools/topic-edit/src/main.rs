use anyhow::{Result};
use clap::Parser;
use colored::*;
use dialoguer::{Input, Select};
use topic_edit::{TopicEditOptions, edit_topic, get_topic_keys};
use common_config::load_config;

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

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read the current configuration for topic listing
    let config = load_config()?;
    
    // Get the topic key to update (interactive if not provided)
    let key = match args.key {
        Some(k) => k,
        None => {
            // Get a list of available topics
            let topics = get_topic_keys(&config);
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
    
    // Get the current topic configuration
    let topic_config = match config.content.topics.get(&key) {
        Some(tc) => tc,
        None => return Err(anyhow::anyhow!("Topic with key '{}' not found", key)),
    };
    
    // Interactive name update if not provided via args
    let name = match args.name {
        Some(n) => Some(n),
        None => {
            let current_name = &topic_config.name;
            let prompt = format!("Enter new name (current: '{}')", current_name);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() { None } else { Some(input) }
        }
    };
    
    // Interactive description update if not provided via args
    let description = match args.description {
        Some(d) => Some(d),
        None => {
            let current_description = &topic_config.description;
            let prompt = format!("Enter new description (current: '{}')", current_description);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() { None } else { Some(input) }
        }
    };
    
    // Prepare options
    let options = TopicEditOptions {
        key: Some(key),
        name,
        description,
    };
    
    // Call the library function
    match edit_topic(&options) {
        Ok(key) => {
            println!("{} Topic '{}' updated successfully", "SUCCESS:".green().bold(), key);
            Ok(())
        },
        Err(e) => Err(e),
    }
} 