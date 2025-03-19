use common_errors::{Result, WritingError};
use clap::Parser;
use content_new::{NewOptions, create_content, get_available_topics, list_templates};
use dialoguer::{Input, Select, Confirm};
use std::convert::From;
use std::env;
use slug::slugify;

// Define our own simplified versions of the CLI argument structs
#[derive(Parser, Debug, Clone)]
pub struct ContentArgs {
    /// Topic of the content
    #[arg(short, long)]
    pub topic: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct CreateArgs {
    /// Title of the content
    #[arg(short = 'T', long, default_value = "")]
    pub title: String,

    /// Description
    #[arg(short = 'g', long, default_value = "")]
    pub description: String,

    /// Content type (article, note, etc.)
    #[arg(short, long, default_value = "")]
    pub content_type: String,

    /// Introduction text
    #[arg(short, long)]
    pub introduction: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct DraftArgs {
    /// Create as draft
    #[arg(short, long)]
    pub draft: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct TemplateArgs {
    /// Template to use
    #[arg(short = 'e', long)]
    pub template: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct TagArgs {
    /// Tags for the content (comma-separated)
    #[arg(short = 'a', long)]
    pub tags: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about = "Create new content")]
struct Args {
    #[command(flatten)]
    content: ContentArgs,

    #[command(flatten)]
    create: CreateArgs,

    #[command(flatten)]
    draft: DraftArgs,

    #[command(flatten)]
    template: TemplateArgs,

    #[command(flatten)]
    tag: TagArgs,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Check if we're in a test environment
    let is_test = env::var("CI").is_ok() || env::var("TEST_MODE").is_ok();

    // Get title if not provided
    let title = if !args.create.title.is_empty() {
        args.create.title.clone()
    } else if is_test {
        "Test Title".to_string()
    } else {
        let input = Input::new()
            .with_prompt("Enter content title")
            .interact_text()
            .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;
        input
    };

    // Generate slug from title
    let slug = slugify(&title);

    // Get topic if not provided
    let topic = match &args.content.topic {
        Some(t) => t.clone(),
        None => {
            if is_test {
                "blog".to_string()
            } else {
                // Get available topics
                let topics = get_available_topics()
                    .map_err(|e| WritingError::validation_error(format!("Error getting topics: {}", e)))?;

                // Create a list of topic options
                let topic_display: Vec<String> = topics.iter()
                    .map(|(k, v)| format!("{} - {}", k, v.description))
                    .collect();

                let selection = Select::new()
                    .with_prompt("Select a topic")
                    .items(&topic_display)
                    .default(0)
                    .interact()
                    .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;

                topics[selection].0.clone()
            }
        }
    };

    // Get description if not provided
    let description = if !args.create.description.is_empty() {
        args.create.description.clone()
    } else if is_test {
        "Test Description".to_string()
    } else {
        let input = Input::new()
            .with_prompt("Enter content description")
            .interact_text()
            .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;
        input
    };

    // Get content type if not already specified
    let _content_type = if !args.create.content_type.is_empty() {
        args.create.content_type.clone()
    } else if is_test {
        "article".to_string()
    } else {
        // Offer content type selection
        let content_types = vec!["article", "note", "tutorial"];
        let selection = Select::new()
            .with_prompt("Select content type")
            .items(&content_types)
            .default(0)
            .interact()
            .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;

        content_types[selection].to_string()
    };

    // Get template if not provided
    let template = match &args.template.template {
        Some(t) => Some(t.clone()),
        None => {
            if is_test {
                None
            } else {
                // Ask if user wants to select a template
                let select_template = Confirm::new()
                    .with_prompt("Would you like to select a template?")
                    .default(false)
                    .interact()
                    .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;

                if select_template {
                    // Get available templates
                    let templates = list_templates()
                        .map_err(|e| WritingError::validation_error(format!("Error listing templates: {}", e)))?;

                    // Create a list of template options
                    let template_display: Vec<String> = templates.iter()
                        .map(|t| format!("{} ({})", t.name, t.content_type))
                        .collect();

                    let selection = Select::new()
                        .with_prompt("Select a template")
                        .items(&template_display)
                        .default(0)
                        .interact()
                        .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;

                    Some(templates[selection].name.clone())
                } else {
                    None
                }
            }
        }
    };

    // Get tags if not provided
    let tags = match &args.tag.tags {
        Some(t) => t.clone(),
        None => {
            if is_test {
                "".to_string()
            } else {
                let input = Input::new()
                    .with_prompt("Enter tags (comma-separated)")
                    .allow_empty(true)
                    .interact_text()
                    .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;
                input
            }
        }
    };

    // Get draft status if not provided
    let draft = args.draft.draft;

    // Get introduction if not provided
    let _introduction = match &args.create.introduction {
        Some(i) => Some(i.clone()),
        None => {
            if is_test {
                None
            } else {
                // Ask if user wants to provide an introduction
                let provide_intro = Confirm::new()
                    .with_prompt("Would you like to provide an introduction?")
                    .default(false)
                    .interact()
                    .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;

                if provide_intro {
                    let input = Input::new()
                        .with_prompt("Enter introduction")
                        .interact_text()
                        .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;
                    Some(input)
                } else {
                    None
                }
            }
        }
    };

    // Create content options
    let title_clone = title.clone();
    let options = NewOptions {
        slug: Some(slug),
        topic: Some(topic),
        title: Some(title),
        description: Some(description),
        template,
        tags: if !tags.is_empty() {
            Some(tags.split(',').map(|s| s.trim().to_string()).collect())
        } else {
            None
        },
        draft: Some(draft),
    };

    // Create content
    let content_path = create_content(&options)
        .map_err(|e| WritingError::validation_error(format!("Error creating content: {}", e)))?;

    // Print success message
    println!("Created content: {} (\"{}\")", content_path.display(), title_clone);

    Ok(())
}