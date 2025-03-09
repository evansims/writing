use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use content_build::{BuildOptions, build_content};
use std::path::PathBuf;

/// Tool for building content into static files (JSON, HTML, RSS, sitemap)
#[derive(Parser, Debug)]
#[clap(name = "content-build")]
struct Args {
    /// Output directory for generated files (default: public)
    #[clap(long, short)]
    output_dir: Option<String>,

    /// Specific content slug to build
    #[clap(long, short)]
    slug: Option<String>,

    /// Specific topic to build content from
    #[clap(long, short)]
    topic: Option<String>,

    /// Include draft content
    #[clap(long)]
    include_drafts: bool,

    /// Skip HTML generation
    #[clap(long)]
    skip_html: bool,

    /// Skip JSON generation
    #[clap(long)]
    skip_json: bool,

    /// Skip RSS feed generation
    #[clap(long)]
    skip_rss: bool,

    /// Skip sitemap generation
    #[clap(long)]
    skip_sitemap: bool,

    /// Show verbose output
    #[clap(long, short)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Convert args to BuildOptions
    let options = BuildOptions {
        output_dir: args.output_dir,
        slug: args.slug,
        topic: args.topic,
        include_drafts: args.include_drafts,
        skip_html: args.skip_html,
        skip_json: args.skip_json,
        skip_rss: args.skip_rss,
        skip_sitemap: args.skip_sitemap,
        verbose: args.verbose,
    };

    // Build the content
    build_content(&options)?;

    // Determine output directory for success message
    let output_dir = match &options.output_dir {
        Some(dir) => dir.clone(),
        None => "public".to_string(),
    };

    println!("{} Content built successfully to {}", "✓".green(), output_dir);
    Ok(())
} 