use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{Input, Confirm};
use topic_add::{AddOptions, add_topic, add_tags_to_topic, generate_key_from_name};

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

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Get topic name (either from args or prompt)
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
            let suggested_key = generate_key_from_name(&name);
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
    
    // Create options for adding a topic
    let options = AddOptions {
        key: key,
        name: name,
        description: description,
        directory: path,
    };
    
    // Add the topic
    let topic_key = add_topic(&options)?;
    
    println!("{} Topic '{}' added successfully", "SUCCESS:".green().bold(), topic_key);
    
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
            if add_tags_to_topic(&topic_key, tags)? {
                println!("{} Tags added for topic '{}'", "SUCCESS:".green().bold(), topic_key);
            }
        }
    }
    
    Ok(())
} 