use anyhow::{Result};
use clap::Parser;
use colored::*;
use dialoguer::{Input, Select, Confirm};
use topic_rename::{TopicRenameOptions, rename_topic, generate_key_from_name, topic_exists};
use common_config::load_config;

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

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read the current configuration for interactive selection
    let config = load_config()?;
    
    // Get the current topic key (interactive if not provided)
    let key = match args.key {
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
    if !topic_exists(&config, &key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", key));
    }
    
    // Get the current topic configuration
    let current_topic = config.content.topics.get(&key).unwrap().clone();
    
    // Get the new topic key (interactive if not provided)
    let new_key = match args.new_key {
        Some(k) => k,
        None => {
            let suggested_key = if let Some(new_name) = &args.new_name {
                generate_key_from_name(new_name)
            } else {
                key.clone()
            };
            
            let prompt = format!("Enter new topic key (default: '{}')", suggested_key);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() { suggested_key } else { input }
        }
    };
    
    // Get the new topic name (interactive if not provided)
    let new_name = match args.new_name {
        Some(n) => n,
        None => {
            let prompt = format!("Enter new topic name (current: '{}')", current_topic.name);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() { current_topic.name.clone() } else { input }
        }
    };
    
    // Get the new topic path (interactive if not provided)
    let new_path = match args.new_path {
        Some(p) => p,
        None => {
            let prompt = format!("Enter new topic path (current: '{}')", current_topic.path);
            let input = Input::<String>::new()
                .with_prompt(&prompt)
                .allow_empty(true)
                .interact_text()?;
            
            if input.is_empty() { current_topic.path.clone() } else { input }
        }
    };
    
    // Confirm the changes
    let summary = format!(
        "Topic:\n- Key: {} → {}\n- Name: {} → {}\n- Path: {} → {}",
        key.yellow(), new_key.yellow(),
        current_topic.name.yellow(), new_name.yellow(),
        current_topic.path.yellow(), new_path.yellow()
    );
    
    println!("{}", summary);
    
    if !Confirm::new()
        .with_prompt("Do you want to proceed with these changes?")
        .default(true)
        .interact()?
    {
        println!("Operation cancelled");
        return Ok(());
    }
    
    // Create options
    let options = TopicRenameOptions {
        key: Some(key),
        new_key: Some(new_key),
        new_name: Some(new_name),
        new_path: Some(new_path),
    };
    
    // Rename the topic
    match rename_topic(&options) {
        Ok(topic_key) => {
            println!("{} Topic renamed to '{}' successfully", "SUCCESS:".green().bold(), topic_key);
            Ok(())
        },
        Err(e) => Err(e),
    }
} 