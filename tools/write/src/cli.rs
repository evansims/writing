//! # CLI Module
//!
//! This module defines the CLI interface for the application.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Content Management CLI",
    long_about = "A tool for managing writing content, topics, images, and build processes.",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
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

    /// Generate statistics about content
    Stats {
        /// Generate statistics for a specific content slug
        #[arg(short, long)]
        slug: Option<String>,

        /// Generate statistics for a specific topic
        #[arg(short, long)]
        topic: Option<String>,

        /// Include draft content in statistics
        #[arg(short, long)]
        include_drafts: bool,

        /// Sort by field (date, title, words, reading_time)
        #[arg(short, long, default_value = "date")]
        sort_by: String,

        /// Show detailed statistics (per content)
        #[arg(short, long)]
        detailed: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ContentCommands {
    /// Create new content
    New {
        /// Title of the content
        #[arg(short, long)]
        title: String,

        /// Topic to create the content in
        #[arg(short, long)]
        topic: String,

        /// Tagline or subtitle for the content
        #[arg(short, long)]
        tagline: Option<String>,

        /// Tags for the content (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,

        /// Mark as draft
        #[arg(short, long)]
        draft: bool,

        /// Template to use
        #[arg(short, long)]
        template: Option<String>,

        /// Open editor after creation
        #[arg(short, long)]
        edit: bool,
    },

    /// Edit content
    Edit {
        /// Slug of the content to edit
        #[arg(short, long)]
        slug: Option<String>,

        /// Topic of the content
        #[arg(short, long)]
        topic: Option<String>,

        /// Field to update (for single field updates)
        #[arg(short, long)]
        field: Option<String>,

        /// Value to set (for single field updates)
        #[arg(short, long)]
        value: Option<String>,

        /// Open in editor
        #[arg(short, long)]
        editor: bool,
    },

    /// Move content to a different location
    Move {
        /// Slug of the content to move
        #[arg(short, long)]
        slug: String,

        /// Topic to move from
        #[arg(short, long)]
        from: Option<String>,

        /// Topic to move to
        #[arg(short, long)]
        to: Option<String>,
    },

    /// Delete content
    Delete {
        /// Slug of the content to delete
        #[arg(short, long)]
        slug: String,

        /// Topic of the content
        #[arg(short, long)]
        topic: Option<String>,

        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Validate content
    Validate {
        /// Slug of the content to validate
        #[arg(short, long)]
        slug: Option<String>,

        /// Topic of the content
        #[arg(short, long)]
        topic: Option<String>,

        /// Validate all content
        #[arg(short, long)]
        all: bool,

        /// Fix validation issues
        #[arg(short, long)]
        fix: bool,
    },

    /// List content
    List {
        /// Topic to list content for
        #[arg(short, long)]
        topic: Option<String>,

        /// Include draft content
        #[arg(short, long)]
        drafts: bool,

        /// Output format (table, json, yaml)
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Search content
    Search {
        /// Search query
        #[arg(short, long)]
        query: Option<String>,

        /// Topic to search in
        #[arg(short, long)]
        topic: Option<String>,

        /// Include draft content
        #[arg(short, long)]
        drafts: bool,

        /// Output format (table, json, yaml)
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Content template management
    Template {
        /// Template to create or use
        #[arg(short, long)]
        template: Option<String>,

        /// List available templates
        #[arg(short, long)]
        list: bool,

        /// Output path for new template
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum TopicCommands {
    /// Add a new topic
    Add {
        /// Key/slug for the topic
        #[arg(short, long)]
        key: String,

        /// Name of the topic
        #[arg(short, long)]
        name: Option<String>,

        /// Description of the topic
        #[arg(short, long)]
        description: Option<String>,

        /// Directory to store the topic content
        #[arg(short, long)]
        directory: Option<String>,
    },

    /// Edit a topic
    Edit {
        /// Key/slug of the topic to edit
        #[arg(short, long)]
        key: String,

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
    Rename {
        /// Current key/slug of the topic
        #[arg(short, long)]
        from: String,

        /// New key/slug for the topic
        #[arg(short, long)]
        to: String,
    },

    /// Delete a topic
    Delete {
        /// Key/slug of the topic to delete
        #[arg(short, long)]
        key: String,

        /// Force deletion (even if content exists)
        #[arg(short, long)]
        force: bool,
    },

    /// List topics
    List {
        /// Output format (table, json, yaml)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ImageCommands {
    /// Build images
    Build {
        /// Topic to build images for
        #[arg(short, long)]
        topic: Option<String>,

        /// Force rebuild all images
        #[arg(short, long)]
        rebuild: bool,
    },

    /// Optimize images
    Optimize {
        /// Topic to optimize images for
        #[arg(short, long)]
        topic: Option<String>,

        /// Re-optimize already optimized images
        #[arg(short, long)]
        reoptimize: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum BuildCommands {
    /// Build content (generate HTML, JSON)
    Content {
        /// Topic to build content for (optional)
        #[arg(short, long)]
        topic: Option<String>,

        /// Force rebuild all content
        #[arg(short, long)]
        rebuild: bool,
    },

    /// Generate table of contents
    Toc {
        /// Topic to generate TOC for (optional)
        #[arg(short, long)]
        topic: Option<String>,
    },

    /// Analyze benchmark results
    Benchmark {
        /// Directory containing baseline benchmark results
        #[arg(short, long)]
        baseline: Option<String>,

        /// Directory containing current benchmark results
        #[arg(short, long)]
        current: String,

        /// Regression threshold percentage
        #[arg(short, long, default_value = "10")]
        threshold: f64,

        /// Output report file
        #[arg(short, long, default_value = "benchmark_report.md")]
        report: String,

        /// Output JSON format
        #[arg(short, long)]
        json: bool,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}