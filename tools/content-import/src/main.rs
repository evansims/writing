use chrono::Local;
use clap::Parser;
use common_errors::{Result, WritingError};
use dialoguer::{Confirm, Input};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde_yaml;
use slug::slugify;
use std::fs;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about = "Import web content as a new article")]
struct Args {
    /// URL to import content from
    #[arg(short, long)]
    url: Option<String>,

    /// Title override (optional)
    #[arg(short, long)]
    title: Option<String>,

    /// Create as draft
    #[arg(short, long)]
    draft: bool,

    /// Tags (comma-separated)
    #[arg(short, long)]
    tags: Option<String>,
}

fn extract_content(url: &str) -> Result<(String, String)> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .map_err(|e| WritingError::validation_error(format!("Failed to fetch URL: {}", e)))?;

    let html = response.text().map_err(|e| {
        WritingError::validation_error(format!("Failed to get response text: {}", e))
    })?;

    let document = Html::parse_document(&html);

    // Try to get title from og:title or regular title
    let title = {
        let og_title_selector = Selector::parse("meta[property='og:title']").unwrap();
        let title_selector = Selector::parse("title").unwrap();

        document
            .select(&og_title_selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .or_else(|| {
                document
                    .select(&title_selector)
                    .next()
                    .map(|el| el.inner_html())
            })
            .unwrap_or_default()
            .trim()
            .to_string()
    };

    // Try to get main content
    let content = {
        let article_selector = Selector::parse("article").unwrap();
        let main_selector = Selector::parse("main").unwrap();
        let body_selector = Selector::parse("body").unwrap();

        let content_html = document
            .select(&article_selector)
            .next()
            .or_else(|| document.select(&main_selector).next())
            .or_else(|| document.select(&body_selector).next())
            .map(|el| el.inner_html())
            .unwrap_or_default();

        html2md::parse_html(&content_html)
    };

    Ok((title, content))
}

fn create_content_file(
    title: &str,
    content: &str,
    url: &str,
    draft: bool,
    tags: Option<String>,
) -> Result<()> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let slug = slugify(title);
    let content_dir = if draft {
        "content/drafts"
    } else {
        "content/published"
    };
    let file_path = PathBuf::from(format!("{}/{}-{}.mdx", content_dir, date, slug));

    // Create directories if they don't exist
    fs::create_dir_all(content_dir).map_err(|e| {
        WritingError::validation_error(format!("Failed to create directory: {}", e))
    })?;

    // Create frontmatter
    let frontmatter = serde_yaml::to_string(&serde_yaml::Value::Mapping(
        vec![
            ("title".into(), title.into()),
            ("date".into(), date.into()),
            ("draft".into(), draft.into()),
            ("source_url".into(), url.into()),
            ("tags".into(), tags.unwrap_or_default().into()),
        ]
        .into_iter()
        .collect(),
    ))
    .map_err(|e| WritingError::validation_error(format!("Failed to create frontmatter: {}", e)))?;

    // Combine frontmatter and content
    let full_content = format!("---\n{}---\n\n{}", frontmatter, content);

    // Write to file
    fs::write(&file_path, full_content)
        .map_err(|e| WritingError::validation_error(format!("Failed to write file: {}", e)))?;

    println!("Content imported successfully to: {}", file_path.display());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get URL if not provided
    let url_str = match args.url {
        Some(u) => u,
        None => {
            let input: String = Input::new()
                .with_prompt("Enter URL to import")
                .interact_text()
                .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;
            input
        }
    };

    // Validate URL
    let url = Url::parse(&url_str)
        .map_err(|e| WritingError::validation_error(format!("Invalid URL: {}", e)))?;

    // Extract content
    let (extracted_title, content) = extract_content(url.as_str())?;

    // Get or confirm title
    let title = match args.title {
        Some(t) => t,
        None => {
            let input: String = Input::new()
                .with_prompt("Enter/confirm title")
                .default(extracted_title)
                .interact_text()
                .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;
            input
        }
    };

    // Get tags if not provided
    let tags = match args.tags {
        Some(t) => Some(t),
        None => {
            let input: String = Input::new()
                .with_prompt("Enter tags (comma-separated)")
                .allow_empty(true)
                .interact_text()
                .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?;
            if input.is_empty() {
                None
            } else {
                Some(input)
            }
        }
    };

    // Confirm draft status
    let draft = if args.draft {
        true
    } else {
        Confirm::new()
            .with_prompt("Create as draft?")
            .default(true)
            .interact()
            .map_err(|e| WritingError::validation_error(format!("Dialog error: {}", e)))?
    };

    // Create the content file
    create_content_file(&title, &content, url.as_str(), draft, tags)?;

    Ok(())
}
