use anyhow::Result;
use colored::*;
use content_delete::{DeleteCommand, DeleteArgs, list_all_content, extract_title_from_content};
use common_cli::Command;
use clap::Parser;
use dialoguer::{Confirm, Select};
use common_errors::WritingError;

fn main() -> Result<()> {
    // Parse command line arguments directly
    let args = DeleteArgs::parse();

    // If no slug is provided, handle interactive selection
    if args.slug.is_none() {
        handle_interactive_selection(args)
    } else {
        // Create and execute command with provided arguments
        if !args.force {
            // Confirm deletion
            let slug = args.slug.as_ref().unwrap();
            let _topic_str = args.topic.as_ref().map_or("any topic", |t| t);
            
            let content_list = list_all_content()?;
            let matching_content = content_list.iter().find(|(t, s, _)| {
                s == slug && args.topic.as_ref().map_or(true, |topic| t == topic)
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
            } else {
                println!("{} Warning: Unable to find matching content for confirmation", "⚠".yellow());
                let confirm_message = format!("Are you sure you want to delete '{}'?", slug);
                if !Confirm::new().with_prompt(confirm_message).interact()? {
                    println!("Operation cancelled");
                    return Ok(());
                }
            }
        }
        
        // Execute the command
        let cmd = DeleteCommand::new(args);
        let result = cmd.execute()?;
        DeleteCommand::handle_result(result);
        Ok(())
    }
}

/// Handle interactive content selection
fn handle_interactive_selection(args: DeleteArgs) -> Result<()> {
    let content_list = list_all_content()?;
    
    if content_list.is_empty() {
        return Err(WritingError::content_not_found("No content found").into());
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
    if !args.force {
        let confirm_message = format!("Delete content '{}/{}' ({})?", topic, slug, title);
        if !Confirm::new().with_prompt(confirm_message).interact()? {
            println!("Operation cancelled");
            return Ok(());
        }
    }
    
    // Create updated args with the selected item
    let selected_args = DeleteArgs {
        slug: Some(slug.clone()),
        topic: Some(topic.clone()),
        force: args.force,
    };
    
    // Execute the command with selected item
    let cmd = DeleteCommand::new(selected_args);
    let result = cmd.execute()?;
    DeleteCommand::handle_result(result);
    Ok(())
} 