use anyhow::Result;
use clap::Parser;
use colored::*;
use content_delete::{DeleteOptions, delete_content, list_all_content, extract_title_from_content};
use dialoguer::{Confirm, Select};

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

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Create delete options
    let options = DeleteOptions {
        slug: args.slug.clone(),
        topic: args.topic.clone(),
        force: args.force,
    };
    
    // If no slug is provided, show a selection menu
    if options.slug.is_none() {
        let content_list = list_all_content()?;
        
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
        
        let (topic, slug, content_dir) = &content_list[selection];
        
        // Get content title for confirmation
        let content_file = content_dir.join("index.mdx");
        let title = extract_title_from_content(&content_file)?;
        
        // Confirm deletion
        if !options.force {
            let confirm_message = format!("Delete content '{}/{}' ({})?", topic, slug, title);
            if !Confirm::new().with_prompt(confirm_message).interact()? {
                println!("Operation cancelled");
                return Ok(());
            }
        }
        
        // Create a DeleteOptions with the selected slug
        let selected_options = DeleteOptions {
            slug: Some(slug.clone()),
            topic: Some(topic.clone()),
            force: options.force,
        };
        
        // Delete the content
        let (topic, slug, title) = delete_content(&selected_options)?;
        
        println!("{} Content deleted: {}/{} ({})", 
            "SUCCESS:".green().bold(), 
            topic, slug, title
        );
    } else {
        // Delete using provided slug
        // Confirm deletion if not forced
        if !options.force {
            if let Some(slug) = &options.slug {
                // Since we don't have the title yet, we need to find the content first
                let content_list = list_all_content()?;
                let matching_content = content_list.iter().find(|(t, s, _)| {
                    s == slug && options.topic.as_ref().map_or(true, |topic| t == topic)
                });
                
                if let Some((topic, slug, content_dir)) = matching_content {
                    // Get content title for confirmation
                    let content_file = content_dir.join("index.mdx");
                    let title = extract_title_from_content(&content_file)?;
                    
                    let confirm_message = format!("Delete content '{}/{}' ({})?", topic, slug, title);
                    if !Confirm::new().with_prompt(confirm_message).interact()? {
                        println!("Operation cancelled");
                        return Ok(());
                    }
                }
            }
        }
        
        // Delete the content
        let (topic, slug, title) = delete_content(&options)?;
        
        println!("{} Content deleted: {}/{} ({})", 
            "SUCCESS:".green().bold(), 
            topic, slug, title
        );
    }
    
    Ok(())
} 