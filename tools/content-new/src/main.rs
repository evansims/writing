use anyhow::Result;
use clap::Parser;
use colored::*;
use content_new::{ContentOptions, create_content, get_available_topics};
use dialoguer::{Input, Select};

#[derive(Parser)]
#[command(author, version, about = "Create new content")]
struct Args {
    /// Content title
    #[arg(short, long)]
    title: Option<String>,

    /// Content topic
    #[arg(short, long)]
    topic: Option<String>,

    /// Content tagline
    #[arg(short, long)]
    tagline: Option<String>,

    /// Comma-separated tags
    #[arg(short, long)]
    tags: Option<String>,

    /// Content type (article, note, etc.)
    #[arg(short, long, default_value = "article")]
    content_type: String,

    /// Draft mode (don't set published date)
    #[arg(short, long)]
    draft: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Get title if not provided
    let title = match args.title {
        Some(t) => t,
        None => {
            let input: String = Input::new()
                .with_prompt("Enter content title")
                .interact_text()?;
            input
        }
    };
    
    // Get topic if not provided
    let topic = match args.topic {
        Some(t) => t,
        None => {
            // Get available topics
            let topics = get_available_topics()?;
            
            // Create a list of topic options
            let topic_display: Vec<String> = topics.iter()
                .map(|(k, v)| format!("{} - {}", k, v.description))
                .collect();
            
            let selection = Select::new()
                .with_prompt("Select a topic")
                .items(&topic_display)
                .default(0)
                .interact()?;
            
            topics[selection].0.clone()
        }
    };
    
    // Get tagline if not provided
    let tagline = match args.tagline {
        Some(t) => t,
        None => {
            let input: String = Input::new()
                .with_prompt("Enter content tagline")
                .interact_text()?;
            input
        }
    };
    
    // Get tags if not provided
    let tags = match args.tags {
        Some(t) => t,
        None => {
            let input: String = Input::new()
                .with_prompt("Enter comma-separated tags")
                .interact_text()?;
            input
        }
    };
    
    // Create content
    let options = ContentOptions {
        title,
        topic,
        tagline,
        tags,
        content_type: args.content_type,
        draft: args.draft,
    };
    
    let content_path = create_content(options)?;
    
    println!("{} Content created at: {}", "SUCCESS:".green().bold(), content_path);
    
    // Extract slug from the path
    let slug = content_path.split('/').nth_back(1).unwrap_or("");
    let content_dir = content_path.trim_end_matches("index.mdx").trim_end_matches('/');
    
    println!("Don't forget to add a source image:");
    println!("  {}/index.jpg", content_dir);
    println!("\nTo optimize your source image:");
    println!("  ./write image-optimize --source path/to/image.jpg --article {}", slug);
    
    Ok(())
} 