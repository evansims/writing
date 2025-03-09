use anyhow::Result;
use clap::Parser;
use colored::*;
use content_move::{MoveOptions, move_content, list_all_content};
use dialoguer::{Confirm, Input, Select};

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

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Create move options
    let options = MoveOptions {
        slug: args.slug.clone(),
        new_slug: args.new_slug.clone(),
        topic: args.topic.clone(),
        new_topic: args.new_topic.clone(),
    };
    
    // If no slug is provided, show a selection menu
    if options.slug.is_none() {
        // List all content and let user select
        let content_list = list_all_content()?;
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

        let (topic, slug, _) = &content_list[selection];
        
        // Create a new options struct with the selected content
        let mut new_options = MoveOptions {
            slug: Some(slug.clone()),
            new_slug: None,
            topic: Some(topic.clone()),
            new_topic: None,
        };
        
        // Get new slug
        let new_slug: String = Input::new()
            .with_prompt("Enter new content slug")
            .default(slug.clone())
            .interact_text()?;
        
        new_options.new_slug = Some(new_slug);
        
        // Get new topic if desired
        if Confirm::new().with_prompt("Move to a different topic?").interact()? {
            let content_list = list_all_content()?;
            let unique_topics: Vec<String> = content_list.iter()
                .map(|(topic, _, _)| topic.clone())
                .collect::<std::collections::HashSet<String>>()
                .into_iter()
                .collect();
            
            let topic_selection = Select::new()
                .with_prompt("Select new topic")
                .items(&unique_topics)
                .default(unique_topics.iter().position(|t| t == topic).unwrap_or(0))
                .interact()?;
            
            new_options.new_topic = Some(unique_topics[topic_selection].clone());
        }
        
        // Confirm the move
        let confirm_message = match (&new_options.topic, &new_options.new_topic) {
            (Some(current_topic), Some(new_topic)) => {
                format!("Move content from '{}/{}' to '{}/{}'?", 
                    current_topic, slug, 
                    new_topic, new_options.new_slug.as_ref().unwrap()
                )
            },
            (Some(current_topic), None) => {
                format!("Rename content from '{}/{}' to '{}/{}'?", 
                    current_topic, slug, 
                    current_topic, new_options.new_slug.as_ref().unwrap()
                )
            },
            _ => "Move content?".to_string(),
        };
        
        if !Confirm::new().with_prompt(confirm_message).interact()? {
            println!("Operation cancelled");
            return Ok(());
        }
        
        // Move the content
        match move_content(&new_options) {
            Ok((current_topic, current_slug, new_topic, new_slug)) => {
                println!("{} Content moved from '{}/{}' to '{}/{}'", 
                    "SUCCESS:".green().bold(), 
                    current_topic, current_slug, 
                    new_topic, new_slug
                );
            },
            Err(err) => {
                eprintln!("Error moving content: {}", err);
                return Err(err);
            }
        }
    } else {
        // If slug is provided through command line
        
        // Get new slug if not provided
        let options = if options.new_slug.is_none() {
            let mut updated_options = options.clone();
            let input: String = Input::new()
                .with_prompt("Enter new content slug")
                .interact_text()?;
            updated_options.new_slug = Some(input);
            updated_options
        } else {
            options
        };
        
        // If source and target are the same, there's nothing to do
        if options.slug == options.new_slug && options.topic == options.new_topic {
            println!("No changes requested, nothing to do");
            return Ok(());
        }
        
        // Confirm the move
        let confirm_message = format!("Move content? (details will be shown after successful move)");
        if !Confirm::new().with_prompt(confirm_message).interact()? {
            println!("Operation cancelled");
            return Ok(());
        }
        
        // Move the content
        match move_content(&options) {
            Ok((current_topic, current_slug, new_topic, new_slug)) => {
                println!("{} Content moved from '{}/{}' to '{}/{}'", 
                    "SUCCESS:".green().bold(), 
                    current_topic, current_slug, 
                    new_topic, new_slug
                );
            },
            Err(err) => {
                eprintln!("Error moving content: {}", err);
                return Err(err);
            }
        }
    }
    
    Ok(())
} 