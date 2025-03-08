use anyhow::Result;
use clap::Parser;
use colored::*;
use content_edit::{EditOptions, edit_content, list_all_content, save_edited_content};
use dialoguer::{Editor, Select};

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

fn main() -> Result<()> {
    let args = Args::parse();
    
    let options = EditOptions {
        slug: args.slug.clone(),
        topic: args.topic.clone(),
        frontmatter_only: args.frontmatter,
        content_only: args.content,
    };
    
    // If no slug is provided, show a selection menu
    if options.slug.is_none() {
        let content_list = list_all_content()?;
        
        if content_list.is_empty() {
            return Err(anyhow::anyhow!("No content found"));
        }
        
        let options_display: Vec<String> = content_list.iter()
            .map(|(topic, slug, _)| format!("{}/{}", topic, slug))
            .collect();
        
        let selection = Select::new()
            .with_prompt("Select content to edit")
            .items(&options_display)
            .default(0)
            .interact()?;
        
        let (topic, slug, _) = &content_list[selection];
        
        let updated_options = EditOptions {
            slug: Some(slug.clone()),
            topic: Some(topic.clone()),
            frontmatter_only: args.frontmatter,
            content_only: args.content,
        };
        
        let (content_path, content) = edit_content(&updated_options)?;
        
        // Open the content in an editor
        if let Some(edited_content) = Editor::new().edit(&content)? {
            // Save the edited content
            save_edited_content(&content_path, &edited_content)?;
            println!("{} Content updated successfully", "SUCCESS:".green().bold());
        } else {
            println!("{} Edit cancelled", "INFO:".blue().bold());
        }
    } else {
        // Edit the specified content
        let (content_path, content) = edit_content(&options)?;
        
        // Open the content in an editor
        if let Some(edited_content) = Editor::new().edit(&content)? {
            // Save the edited content
            save_edited_content(&content_path, &edited_content)?;
            println!("{} Content updated successfully", "SUCCESS:".green().bold());
        } else {
            println!("{} Edit cancelled", "INFO:".blue().bold());
        }
    }
    
    Ok(())
} 