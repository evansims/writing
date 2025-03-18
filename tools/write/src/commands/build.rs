use clap::Args;
use anyhow::Result;
use crate::tools::build;

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[command(subcommand)]
    pub command: BuildCommands,
}

#[derive(clap::Subcommand, Debug)]
pub enum BuildCommands {
    /// Build content (generate HTML, JSON)
    Content {
        /// Topic to build content for (optional)
        #[arg(short, long)]
        topic: Option<String>,

        /// Force rebuild all content
        #[arg(short, long)]
        rebuild: bool,

        /// Skip HTML generation
        #[arg(long)]
        skip_html: bool,

        /// Skip JSON generation
        #[arg(long)]
        skip_json: bool,

        /// Skip RSS generation
        #[arg(long)]
        skip_rss: bool,

        /// Skip sitemap generation
        #[arg(long)]
        skip_sitemap: bool,

        /// Output directory (defaults to "public")
        #[arg(short, long)]
        output: Option<String>,

        /// Include draft content
        #[arg(short, long)]
        drafts: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Generate table of contents
    Toc {
        /// Topic to generate TOC for (optional)
        #[arg(short, long)]
        topic: Option<String>,

        /// Output file (defaults to "public/toc.json")
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Generate LLM (large language model) training data
    Llm {
        /// Site URL (defaults to "https://example.com")
        #[arg(short, long)]
        site_url: Option<String>,

        /// Output directory (defaults to "public/llm")
        #[arg(short, long)]
        output: Option<String>,

        /// Include draft content
        #[arg(short, long)]
        drafts: bool,
    },

    /// Build search index
    Search {
        /// Output file (defaults to "public/search-index.json")
        #[arg(short, long)]
        output: Option<String>,

        /// Include draft content
        #[arg(short, long)]
        drafts: bool,
    }
}

pub fn execute(args: BuildArgs) -> Result<()> {
    match args.command {
        BuildCommands::Content {
            topic,
            rebuild,
            skip_html,
            skip_json,
            skip_rss,
            skip_sitemap,
            output,
            drafts,
            verbose,
        } => {
            // Initialize the lazy build cache for better performance
            let _cache = build::lazy_build_cache();

            // Build content
            build::build_content(
                output,
                None, // slug
                topic,
                drafts,
                skip_html,
                skip_json,
                skip_rss,
                skip_sitemap,
                rebuild,
                verbose,
            )
        },
        BuildCommands::Toc { topic, output } => {
            // This function already handles both topic-specific and general ToC generation
            build::generate_toc(output)
        },
        BuildCommands::Llm { site_url, output, drafts } => {
            build::generate_llms(site_url, output, drafts)
        },
        BuildCommands::Search { output, drafts } => {
            build::build_search_index(output, drafts)
        }
    }
}