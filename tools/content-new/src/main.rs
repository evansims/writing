use common_errors::Result;
use clap::Parser;
use colored::*;
use content_new::{ContentOptions, create_content, get_available_topics, list_templates};
use dialoguer::{Input, Select, Confirm};

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

    /// Template to use
    #[arg(short, long)]
    template: Option<String>,

    /// Draft mode (don't set published date)
    #[arg(short, long)]
    draft: bool,
    
    /// Introduction text
    #[arg(long)]
    introduction: Option<String>,
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
    
    // Get content type if not already specified
    let content_type = if args.content_type != "article" {
        args.content_type.clone()
    } else {
        // Offer content type selection
        let content_types = vec!["article", "note", "tutorial"];
        let selection = Select::new()
            .with_prompt("Select content type")
            .items(&content_types)
            .default(0)
            .interact()?;
        
        content_types[selection].to_string()
    };
    
    // Get template if not provided
    let template = match args.template {
        Some(t) => Some(t),
        None => {
            // Ask if user wants to select a template
            let select_template = Confirm::new()
                .with_prompt("Would you like to select a template?")
                .default(false)
                .interact()?;
                
            if select_template {
                // List available templates
                let templates = list_templates()?;
                
                // Filter templates by content type if possible
                let filtered_templates: Vec<_> = templates.iter()
                    .filter(|t| t.content_type == content_type)
                    .collect();
                
                let template_list = if !filtered_templates.is_empty() {
                    filtered_templates
                } else {
                    templates.iter().collect()
                };
                
                // Create a list of template options
                let template_display: Vec<String> = template_list.iter()
                    .map(|t| format!("{} - {}", t.name, t.description))
                    .collect();
                
                let selection = Select::new()
                    .with_prompt("Select a template")
                    .items(&template_display)
                    .default(0)
                    .interact()?;
                
                Some(template_list[selection].name.clone())
            } else {
                None
            }
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
                .allow_empty(true)
                .interact_text()?;
            input
        }
    };
    
    // Get introduction if not provided and content type is article or tutorial
    let introduction = if content_type == "article" || content_type == "tutorial" {
        match args.introduction {
            Some(i) => Some(i),
            None => {
                let input: String = Input::new()
                    .with_prompt("Enter introduction paragraph")
                    .allow_empty(true)
                    .interact_text()?;
                
                if input.is_empty() {
                    None
                } else {
                    Some(input)
                }
            }
        }
    } else {
        None
    };
    
    // Create content
    let options = ContentOptions {
        title,
        topic,
        tagline,
        tags,
        content_type,
        draft: args.draft,
        template,
        introduction,
    };
    
    let content_path = create_content(options)?;
    
    println!("{} Content created at: {}", "SUCCESS:".green().bold(), content_path);
    
    // Extract slug from the path
    let slug = content_path.split('/').nth_back(1).unwrap_or("");
    let content_dir = content_path.trim_end_matches("index.mdx").trim_end_matches('/');
    
    println!("Don't forget to add a source image:");
    println!("  {}/index.jpg", content_dir);
    println!("\nTo optimize your source image:");
    println!("  ./write image optimize --source path/to/image.jpg --article {}", slug);
    
    Ok(())
} 