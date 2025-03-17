use clap::Args;
use std::path::PathBuf;

/// Common arguments for content-related commands
#[derive(Args, Debug, Clone)]
pub struct ContentArgs {
    /// Slug of the content
    #[arg(short, long)]
    pub slug: Option<String>,
    
    /// Topic of the content
    #[arg(short, long)]
    pub topic: Option<String>,
}

/// Common arguments for draft-related commands
#[derive(Args, Debug, Clone)]
pub struct DraftArgs {
    /// Create as draft
    #[arg(short, long)]
    pub draft: bool,
}

/// Common arguments for template-related commands
#[derive(Args, Debug, Clone)]
pub struct TemplateArgs {
    /// Template to use
    #[arg(short, long)]
    pub template: Option<String>,
}

/// Common arguments for tag-related commands
#[derive(Args, Debug, Clone)]
pub struct TagArgs {
    /// Tags for the content (comma-separated)
    #[arg(short, long)]
    pub tags: Option<String>,
}

/// Common arguments for content editing
#[derive(Args, Debug, Clone)]
pub struct EditArgs {
    /// Edit only the frontmatter
    #[arg(long)]
    pub frontmatter_only: bool,
    
    /// Edit only the content
    #[arg(long)]
    pub content_only: bool,
}

/// Common arguments for content creation
#[derive(Args, Debug, Clone)]
pub struct CreateArgs {
    /// Title of the content
    #[arg(short, long)]
    pub title: String,
    
    /// Tagline or description
    #[arg(short, long)]
    pub tagline: String,
    
    /// Content type (article, note, etc.)
    #[arg(short, long)]
    pub content_type: String,
    
    /// Introduction text
    #[arg(short, long)]
    pub introduction: Option<String>,
}

/// Common arguments for force operations
#[derive(Args, Debug, Clone)]
pub struct ForceArgs {
    /// Force operation without confirmation
    #[arg(short, long)]
    pub force: bool,
}

/// Common arguments for output format
#[derive(Args, Debug, Clone)]
pub struct OutputFormatArgs {
    /// Output format (text, json, yaml)
    #[arg(short, long, default_value = "text")]
    pub format: String,
}

/// Common arguments for verbose output
#[derive(Args, Debug, Clone)]
pub struct VerboseArgs {
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

/// Common arguments for file operations
#[derive(Args, Debug, Clone)]
pub struct FileArgs {
    /// File path
    #[arg(short, long)]
    pub file: PathBuf,
}

/// Common arguments for directory operations
#[derive(Args, Debug, Clone)]
pub struct DirectoryArgs {
    /// Directory path
    #[arg(short, long)]
    pub directory: PathBuf,
}

/// Common arguments for recursive operations
#[derive(Args, Debug, Clone)]
pub struct RecursiveArgs {
    /// Recursive operation
    #[arg(short, long)]
    pub recursive: bool,
}

/// Common arguments for limit operations
#[derive(Args, Debug, Clone)]
pub struct LimitArgs {
    /// Limit the number of results
    #[arg(short, long, default_value = "10")]
    pub limit: usize,
}

/// Common arguments for search operations
#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    /// Search query
    #[arg(short, long)]
    pub query: String,
}

/// Common arguments for sort operations
#[derive(Args, Debug, Clone)]
pub struct SortArgs {
    /// Sort field
    #[arg(short, long, default_value = "title")]
    pub sort: String,
    
    /// Sort direction (asc, desc)
    #[arg(long, default_value = "asc")]
    pub direction: String,
}

/// Common arguments for pagination
#[derive(Args, Debug, Clone)]
pub struct PaginationArgs {
    /// Page number
    #[arg(short, long, default_value = "1")]
    pub page: usize,
    
    /// Page size
    #[arg(long, default_value = "10")]
    pub page_size: usize,
} 