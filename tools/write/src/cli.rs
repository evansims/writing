//! # CLI Argument Handling
//! 
//! This module provides CLI argument parsing and command structure for the write tool.

use clap::{Parser, Subcommand};

/// Main CLI parser for the write tool
#[derive(Parser)]
#[command(
    name = "write",
    author = "Evan Sims <evan@evansims.com>",
    version,
    about = "A comprehensive tool for managing writing content, topics, images, and output files",
    long_about = "The Content Management CLI tool provides a set of commands for managing writing content, topics, images, and build processes for your writing project. You can create, edit, move, and delete content; manage topics; optimize images; and build content into various formats.

When run without commands, it launches an interactive CLI experience for easier navigation through the tool's features."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Main command groups
#[derive(Subcommand)]
pub enum Commands {
    /// Content management commands
    #[command(subcommand)]
    Content(ContentCommands),
    
    /// Topic management commands
    #[command(subcommand)]
    Topic(TopicCommands),
    
    /// Image management commands
    #[command(subcommand)]
    Image(ImageCommands),
    
    /// Build management commands
    #[command(subcommand)]
    Build(BuildCommands),
    
    /// Generate statistics about your content
    Stats {
        /// Content slug to generate statistics for
        #[arg(long, short)]
        slug: Option<String>,
        
        /// Topic to filter content by
        #[arg(long, short)]
        topic: Option<String>,
        
        /// Include draft content
        #[arg(long)]
        include_drafts: bool,
        
        /// How to sort the statistics (date, words, time)
        #[arg(long, value_name = "SORT_BY")]
        sort_by: String,
        
        /// Show detailed statistics
        #[arg(long)]
        detailed: bool,
    },
}

/// Commands for content management
#[derive(Subcommand)]
pub enum ContentCommands {
    /// Create new content
    #[command(about = "Create a new content item")]
    New {
        /// Title of the content
        #[arg(short, long)]
        title: Option<String>,
        
        /// Topic for the content
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Tagline for the content
        #[arg(short, long)]
        tagline: Option<String>,
        
        /// Tags for the content (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        
        /// Whether the content is a draft
        #[arg(short, long)]
        draft: bool,
        
        /// Use a template for the content
        #[arg(short, long)]
        template: Option<String>,
        
        /// Edit the content after creation
        #[arg(short, long)]
        edit: bool,
    },
    
    /// Edit existing content
    #[command(about = "Edit existing content")]
    Edit {
        /// Slug of the content to edit
        #[arg(short, long)]
        slug: Option<String>,
        
        /// Topic of the content to edit
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Field to edit (title, tagline, tags, content, all)
        #[arg(short, long)]
        field: Option<String>,
        
        /// New value for the field
        #[arg(short, long)]
        value: Option<String>,
        
        /// Use external editor
        #[arg(short, long)]
        editor: bool,
    },
    
    /// Move content to a different topic
    #[command(about = "Move content to a different topic")]
    Move {
        /// Slug of the content to move
        #[arg(short, long)]
        slug: Option<String>,
        
        /// Source topic
        #[arg(short, long)]
        from: Option<String>,
        
        /// Destination topic
        #[arg(short, long)]
        to: Option<String>,
    },
    
    /// Delete content
    #[command(about = "Delete content")]
    Delete {
        /// Slug of the content to delete
        #[arg(short, long)]
        slug: Option<String>,
        
        /// Topic of the content to delete
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Validate content
    #[command(about = "Validate content")]
    Validate {
        /// Slug of the content to validate
        #[arg(short, long)]
        slug: Option<String>,
        
        /// Topic of the content to validate
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Validate all content
        #[arg(short, long)]
        all: bool,
        
        /// Fix validation issues automatically
        #[arg(short, long)]
        fix: bool,
    },
    
    /// List all content
    #[command(about = "List all content")]
    List {
        /// Topic to filter by
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Show draft content
        #[arg(short, long)]
        drafts: bool,
        
        /// Format output (table, json, yaml)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    
    /// Search content
    #[command(about = "Search content")]
    Search {
        /// Query to search for
        #[arg(short, long)]
        query: Option<String>,
        
        /// Topic to filter by
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Show draft content
        #[arg(short, long)]
        drafts: bool,
        
        /// Format output (table, json, yaml)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    
    /// Generate content from a template
    #[command(about = "Generate content from a template")]
    Template {
        /// Template to use
        #[arg(short, long)]
        template: Option<String>,
        
        /// List available templates
        #[arg(short, long)]
        list: bool,
        
        /// Output path for the generated content
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Commands for topic management
#[derive(Subcommand)]
pub enum TopicCommands {
    /// Add a new topic
    #[command(about = "Add a new topic")]
    Add {
        /// Key for the topic
        #[arg(short, long)]
        key: Option<String>,
        
        /// Name of the topic
        #[arg(short, long)]
        name: Option<String>,
        
        /// Description of the topic
        #[arg(short, long)]
        description: Option<String>,
        
        /// Directory for the topic
        #[arg(short, long)]
        directory: Option<String>,
    },
    
    /// Edit an existing topic
    #[command(about = "Edit an existing topic")]
    Edit {
        /// Key of the topic to edit
        #[arg(short, long)]
        key: Option<String>,
        
        /// New name for the topic
        #[arg(short, long)]
        name: Option<String>,
        
        /// New description for the topic
        #[arg(short, long)]
        description: Option<String>,
        
        /// New directory for the topic
        #[arg(short, long)]
        directory: Option<String>,
    },
    
    /// Rename a topic
    #[command(about = "Rename a topic")]
    Rename {
        /// Key of the topic to rename
        #[arg(short, long)]
        from: Option<String>,
        
        /// New key for the topic
        #[arg(short, long)]
        to: Option<String>,
    },
    
    /// Delete a topic
    #[command(about = "Delete a topic")]
    Delete {
        /// Key of the topic to delete
        #[arg(short, long)]
        key: Option<String>,
        
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// List all topics
    #[command(about = "List all topics")]
    List {
        /// Format output (table, json, yaml)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
}

/// Commands for image management
#[derive(Subcommand)]
pub enum ImageCommands {
    /// Build images
    #[command(about = "Build images for content")]
    Build {
        /// Topic to build images for
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Rebuild all images
        #[arg(short, long)]
        rebuild: bool,
    },
    
    /// Optimize images
    #[command(about = "Optimize images for content")]
    Optimize {
        /// Topic to optimize images for
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Re-optimize all images
        #[arg(short, long)]
        reoptimize: bool,
    },
}

/// Commands for building content
#[derive(Subcommand)]
pub enum BuildCommands {
    /// Build content
    #[command(about = "Build content")]
    Content {
        /// Topic to build content for
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Rebuild all content
        #[arg(short, long)]
        rebuild: bool,
    },
    
    /// Generate table of contents
    #[command(about = "Generate table of contents")]
    Toc {
        /// Topic to generate table of contents for
        #[arg(short, long)]
        topic: Option<String>,
    },
} 