use anyhow::Result;
use common_config::load_config;
use common_fs::{find_files_with_extension, read_file};
use common_markdown::extract_frontmatter_and_content;
use common_models::Config;
use common_models::Frontmatter;
use pulldown_cmark::{Event, Options, Parser, Tag};
use reqwest::blocking::Client;
use reqwest::Url;
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;

/// Link kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LocalLinkKind {
    Internal,
    External,
}

/// Link in content
#[derive(Debug)]
pub struct Link {
    url: String,
    kind: LocalLinkKind,
    line: Option<usize>,
    column: Option<usize>,
}

impl Link {
    /// Create a new link
    pub fn new(
        url: String,
        kind: LocalLinkKind,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        Self {
            url,
            kind,
            line,
            column,
        }
    }

    /// Get the URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the kind
    pub fn kind(&self) -> &LocalLinkKind {
        &self.kind
    }

    /// Get the line number
    pub fn line(&self) -> Option<usize> {
        self.line
    }

    /// Get the column number
    pub fn column(&self) -> Option<usize> {
        self.column
    }
}

/// Extract links from content
pub fn extract_links(content: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(content, options);

    for event in parser {
        if let Event::Start(Tag::Link(_, ref url, _)) = event {
            let kind = if url.starts_with("http://") || url.starts_with("https://") {
                LocalLinkKind::External
            } else {
                LocalLinkKind::Internal
            };

            links.push(Link::new(url.to_string(), kind, None, None));
        }
        if let Event::Start(Tag::Image(_, ref url, _)) = event {
            let kind = if url.starts_with("http://") || url.starts_with("https://") {
                LocalLinkKind::External
            } else {
                LocalLinkKind::Internal
            };

            links.push(Link::new(url.to_string(), kind, None, None));
        }
    }

    links
}

/// Validation errors
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Broken link: {0}")]
    BrokenLink(String),

    #[error("Missing resource: {0}")]
    MissingResource(String),

    #[error("Markdown error: {0}")]
    MarkdownError(String),

    #[error("Article not found: {0}")]
    ArticleNotFound(String),

    #[error("Topic not found: {0}")]
    TopicNotFound(String),
}

/// Validation types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationType {
    Links,
    Markdown,
    All,
}

/// Validation options
pub struct ValidationOptions {
    pub article_slug: Option<String>,
    pub topic: Option<String>,
    pub validation_types: Vec<ValidationType>,
    pub check_external_links: bool,
    pub timeout: Option<u64>,
    pub dictionary_path: Option<PathBuf>,
    pub include_drafts: bool,
}

/// Validation result
#[derive(Debug)]
pub struct ValidationResult {
    pub file_path: PathBuf,
    pub issues: Vec<ValidationIssue>,
}

/// Validation issue
#[derive(Debug)]
pub struct ValidationIssue {
    pub issue_type: ValidationIssueType,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub description: String,
    pub suggested_fix: Option<String>,
}

/// Validation issue type
#[derive(Debug, PartialEq)]
pub enum ValidationIssueType {
    BrokenLink,
    MissingInternalLink,
    InvalidUrl,
    MarkdownFormatting,
}

/// Validate content
///
/// This function validates content based on the provided options.
///
/// # Parameters
///
/// * `options` - Validation options
///
/// # Returns
///
/// Returns a list of validation results
///
/// # Errors
///
/// Returns an error if the validation fails
pub fn validate_content(options: &ValidationOptions) -> Result<Vec<ValidationResult>> {
    let config = load_config()?;
    let mut results = Vec::new();

    // If a specific article is requested, only validate that article
    if let Some(article_slug) = &options.article_slug {
        if let Some(topic_key) = &options.topic {
            // Validate a specific article in a specific topic
            if let Some(topic_config) = config.content.topics.get(topic_key) {
                let topic_dir =
                    PathBuf::from(&config.content.base_dir).join(&topic_config.directory);
                let article_dir = topic_dir.join(article_slug);

                if !article_dir.exists() {
                    return Err(ValidationError::ArticleNotFound(article_slug.clone()).into());
                }

                // Check for content file with matching name
                let content_file_md = article_dir.join(format!("{}.md", article_slug));
                let content_file_mdx = article_dir.join(format!("{}.mdx", article_slug));

                // Determine which file to use
                let content_file = if content_file_md.exists() {
                    content_file_md
                } else if content_file_mdx.exists() {
                    content_file_mdx
                } else {
                    return Err(ValidationError::ArticleNotFound(article_slug.clone()).into());
                };

                let content = read_file(&content_file)?;

                // Validate the article
                let mut issues = Vec::new();

                for validation_type in &options.validation_types {
                    match validation_type {
                        ValidationType::Links => {
                            validate_links(&content_file, &content, &config, options, &mut issues)?;
                        }
                        ValidationType::Markdown => {
                            validate_markdown(&content_file, &content, &mut issues)?;
                        }
                        ValidationType::All => {
                            validate_links(&content_file, &content, &config, options, &mut issues)?;
                            validate_markdown(&content_file, &content, &mut issues)?;
                        }
                    }
                }

                results.push(ValidationResult {
                    file_path: content_file,
                    issues,
                });
            } else {
                return Err(ValidationError::TopicNotFound(topic_key.clone()).into());
            }
        } else {
            // Validate a specific article in any topic
            let mut found = false;

            for (_topic_key, topic_config) in &config.content.topics {
                let topic_dir =
                    PathBuf::from(&config.content.base_dir).join(&topic_config.directory);
                let article_dir = topic_dir.join(article_slug);

                if article_dir.exists() {
                    // Check for content file with matching name
                    let content_file_md = article_dir.join(format!("{}.md", article_slug));
                    let content_file_mdx = article_dir.join(format!("{}.mdx", article_slug));

                    // Determine which file to use
                    let content_file = if content_file_md.exists() {
                        Some(content_file_md)
                    } else if content_file_mdx.exists() {
                        Some(content_file_mdx)
                    } else {
                        None
                    };

                    if let Some(content_file) = content_file {
                        found = true;
                        let content = read_file(&content_file)?;

                        // Validate the article
                        let mut issues = Vec::new();

                        for validation_type in &options.validation_types {
                            match validation_type {
                                ValidationType::Links => {
                                    validate_links(
                                        &content_file,
                                        &content,
                                        &config,
                                        options,
                                        &mut issues,
                                    )?;
                                }
                                ValidationType::Markdown => {
                                    validate_markdown(&content_file, &content, &mut issues)?;
                                }
                                ValidationType::All => {
                                    validate_links(
                                        &content_file,
                                        &content,
                                        &config,
                                        options,
                                        &mut issues,
                                    )?;
                                    validate_markdown(&content_file, &content, &mut issues)?;
                                }
                            }
                        }

                        results.push(ValidationResult {
                            file_path: content_file,
                            issues,
                        });
                    }
                }
            }

            if !found {
                return Err(ValidationError::ArticleNotFound(article_slug.clone()).into());
            }
        }
    } else if let Some(topic_key) = &options.topic {
        // Validate all articles in a specific topic
        if let Some(topic_config) = config.content.topics.get(topic_key) {
            let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.directory);

            if !topic_dir.exists() {
                return Err(ValidationError::TopicNotFound(topic_key.clone()).into());
            }

            // Find all content directories in the topic
            let entries = std::fs::read_dir(&topic_dir)?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let slug = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("")
                        .to_string();

                    if slug.is_empty() {
                        continue;
                    }

                    // Check for content files with matching name
                    let content_file_md = path.join(format!("{}.md", slug));
                    let content_file_mdx = path.join(format!("{}.mdx", slug));

                    // Determine which file to use
                    let content_file = if content_file_md.exists() {
                        Some(content_file_md)
                    } else if content_file_mdx.exists() {
                        Some(content_file_mdx)
                    } else {
                        None
                    };

                    if let Some(content_file) = content_file {
                        let content = read_file(&content_file)?;

                        // Check if the content is a draft
                        if !options.include_drafts && is_draft(&content) {
                            continue;
                        }

                        // Validate the article
                        let mut issues = Vec::new();

                        for validation_type in &options.validation_types {
                            match validation_type {
                                ValidationType::Links => {
                                    validate_links(
                                        &content_file,
                                        &content,
                                        &config,
                                        options,
                                        &mut issues,
                                    )?;
                                }
                                ValidationType::Markdown => {
                                    validate_markdown(&content_file, &content, &mut issues)?;
                                }
                                ValidationType::All => {
                                    validate_links(
                                        &content_file,
                                        &content,
                                        &config,
                                        options,
                                        &mut issues,
                                    )?;
                                    validate_markdown(&content_file, &content, &mut issues)?;
                                }
                            }
                        }

                        results.push(ValidationResult {
                            file_path: content_file,
                            issues,
                        });
                    }
                }
            }
        } else {
            return Err(ValidationError::TopicNotFound(topic_key.clone()).into());
        }
    } else {
        // Validate all articles in all topics
        for (_topic_key, topic_config) in &config.content.topics {
            let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.directory);

            if !topic_dir.exists() {
                continue;
            }

            // Find all content directories in the topic
            let entries = std::fs::read_dir(&topic_dir)?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let slug = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("")
                        .to_string();

                    if slug.is_empty() {
                        continue;
                    }

                    // Check for content files with matching name
                    let content_file_md = path.join(format!("{}.md", slug));
                    let content_file_mdx = path.join(format!("{}.mdx", slug));

                    // Determine which file to use
                    let content_file = if content_file_md.exists() {
                        Some(content_file_md)
                    } else if content_file_mdx.exists() {
                        Some(content_file_mdx)
                    } else {
                        None
                    };

                    if let Some(content_file) = content_file {
                        let content = read_file(&content_file)?;

                        // Check if the content is a draft
                        if !options.include_drafts && is_draft(&content) {
                            continue;
                        }

                        // Validate the article
                        let mut issues = Vec::new();

                        for validation_type in &options.validation_types {
                            match validation_type {
                                ValidationType::Links => {
                                    validate_links(
                                        &content_file,
                                        &content,
                                        &config,
                                        options,
                                        &mut issues,
                                    )?;
                                }
                                ValidationType::Markdown => {
                                    validate_markdown(&content_file, &content, &mut issues)?;
                                }
                                ValidationType::All => {
                                    validate_links(
                                        &content_file,
                                        &content,
                                        &config,
                                        options,
                                        &mut issues,
                                    )?;
                                    validate_markdown(&content_file, &content, &mut issues)?;
                                }
                            }
                        }

                        results.push(ValidationResult {
                            file_path: content_file,
                            issues,
                        });
                    }
                }
            }
        }
    }

    Ok(results)
}

/// Validate links in content
fn validate_links(
    _file_path: &Path,
    content: &str,
    _config: &Config,
    options: &ValidationOptions,
    issues: &mut Vec<ValidationIssue>,
) -> Result<()> {
    // Extract links from content
    let links = extract_links(content);

    // Check each link
    for link in &links {
        // Check if the link is a URL
        if *link.kind() == LocalLinkKind::External {
            // Check if the URL is valid
            if let Ok(url) = Url::parse(link.url()) {
                // Check if the URL is accessible
                if options.check_external_links {
                    let client = Client::builder()
                        .timeout(Duration::from_secs(options.timeout.unwrap_or(10)))
                        .build()?;

                    let response = client.head(url.clone()).send();

                    if let Err(e) = response {
                        issues.push(ValidationIssue {
                            issue_type: ValidationIssueType::BrokenLink,
                            line: link.line(),
                            column: link.column(),
                            description: format!("Broken link: {} ({})", link.url(), e),
                            suggested_fix: None,
                        });
                    }
                }
            } else {
                issues.push(ValidationIssue {
                    issue_type: ValidationIssueType::InvalidUrl,
                    line: link.line(),
                    column: link.column(),
                    description: format!("Invalid URL: {}", link.url()),
                    suggested_fix: None,
                });
            }
        } else {
            // Check if the internal link exists
            let target_path = PathBuf::from(link.url());

            if !target_path.exists() {
                issues.push(ValidationIssue {
                    issue_type: ValidationIssueType::MissingInternalLink,
                    line: link.line(),
                    column: link.column(),
                    description: format!("Missing internal link: {}", link.url()),
                    suggested_fix: None,
                });
            }
        }
    }

    Ok(())
}

/// Validate markdown in content
fn validate_markdown(
    _file_path: &Path,
    _content: &str,
    _issues: &mut Vec<ValidationIssue>,
) -> Result<()> {
    // TODO: Implement markdown validation

    Ok(())
}

/// Validate resources in content
#[allow(dead_code)]
fn validate_resources(
    file_path: &Path,
    _content: &str,
    _config: &Config,
    _issues: &mut Vec<ValidationIssue>,
) -> Result<()> {
    // Get the article directory
    let article_dir = file_path.parent().unwrap_or(Path::new(""));

    // Get all files in the article directory
    let _all_files = find_files_with_extension(&article_dir, "")?;

    // Check if there's a build directory
    let build_dir = article_dir.join("build");

    if build_dir.exists() {
        // Get all files in the build directory
        let _build_files = find_files_with_extension(&build_dir, "")?;

        // TODO: Validate resources
    }

    Ok(())
}

#[allow(dead_code)]
fn validate_frontmatter_fields(
    frontmatter: &Frontmatter,
    _file_path: &Path,
    content_type: &str,
    _config: &Config,
) -> Result<()> {
    // Validate required fields based on content type
    match content_type {
        "article" | "note" | "tutorial" => {
            // Check required fields for all content types
            if frontmatter.title.is_empty() {
                return Err(ValidationError::MarkdownError("Title is required".to_string()).into());
            }
        }
        _ => {
            return Err(ValidationError::MarkdownError(format!(
                "Unknown content type: {}",
                content_type
            ))
            .into());
        }
    }

    Ok(())
}

/// Check if content is a draft
fn is_draft(content: &str) -> bool {
    let (frontmatter, _) = extract_frontmatter_and_content(content).unwrap_or_default();
    frontmatter.is_draft.unwrap_or(false)
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    /// Returns the extract_links function for testing
    pub use super::extract_links;

    /// Returns the LocalLinkKind for testing
    pub use super::LocalLinkKind;

    /// Returns the ValidationOptions for testing
    pub use super::ValidationOptions;

    /// Returns the ValidationType for testing
    pub use super::ValidationType;
}
