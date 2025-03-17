use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use content_search::{SearchOptions, build_index, search_content, index_exists};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum ContentType {
    Article,
    Note,
    Tutorial,
    All,
}

impl ContentType {
    fn to_string(&self) -> Option<String> {
        match self {
            ContentType::All => None,
            _ => Some(format!("{:?}", self).to_lowercase()),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about = "Search content with full-text and metadata queries")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for content (default command)
    Search {
        /// Search query
        #[arg(index = 1)]
        query: String,
        
        /// Limit search to a specific topic
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Limit search to a specific content type
        #[arg(short, long, value_enum)]
        content_type: Option<ContentType>,
        
        /// Limit search to content with these tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        
        /// Maximum number of results to return
        #[arg(short, long, default_value = "10")]
        limit: usize,
        
        /// Include draft content
        #[arg(long, default_value = "false")]
        include_drafts: bool,
        
        /// Search only in titles
        #[arg(long, default_value = "false")]
        title_only: bool,
        
        /// Path to index directory
        #[arg(short, long)]
        index_path: Option<PathBuf>,
        
        /// Always rebuild the index before searching
        #[arg(short, long, default_value = "false")]
        rebuild: bool,
    },
    
    /// Build or rebuild the search index
    Build {
        /// Path to index directory
        #[arg(short, long)]
        index_path: Option<PathBuf>,
        
        /// Include draft content
        #[arg(long, default_value = "false")]
        include_drafts: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Search {
            query,
            topic,
            content_type,
            tags,
            limit,
            include_drafts,
            title_only,
            index_path,
            rebuild,
        } => {
            // Convert tags to Vec<String>
            let tags_vec = tags.as_ref().map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
            });
            
            // Convert content type enum to string
            let content_type_str = content_type.and_then(|ct| ct.to_string());
            
            // Create search options
            let options = SearchOptions {
                query: query.clone(),
                topic: topic.clone(),
                content_type: content_type_str,
                tags: tags_vec,
                limit,
                include_drafts,
                title_only,
                raw_query: false,
                case_sensitive: false,
                include_metadata: true,
            };
            
            // Get index path
            let index_path_ref = index_path.as_deref();
            
            // Check if we need to build/rebuild the index
            if rebuild || (index_path_ref.is_some() && !index_exists(index_path_ref.unwrap())) {
                println!("{} search index...", "Building".green().bold());
                build_index(index_path_ref, include_drafts)?;
            }
            
            // Perform the search
            println!("{} for: {}", "Searching".green().bold(), query.yellow());
            
            if let Some(topic) = &topic {
                println!("  {} {}", "Topic:".cyan().bold(), topic);
            }
            
            if let Some(ct) = &content_type {
                println!("  {} {:?}", "Content type:".cyan().bold(), ct);
            }
            
            if let Some(t) = &tags {
                println!("  {} {}", "Tags:".cyan().bold(), t);
            }
            
            // Search
            match search_content(&options) {
                Ok(results) => {
                    println!("\n{} {} results found", "Success:".green().bold(), results.len());
                    
                    for (i, result) in results.iter().enumerate() {
                        println!("\n{}. {} ({})", 
                            (i + 1).to_string().bold(), 
                            result.title.bold(), 
                            format!("score: {:.2}", result.score).cyan()
                        );
                        
                        println!("   Topic: {}, Type: {}", result.topic, result.content_type);
                        
                        if !result.tags.is_empty() {
                            println!("   Tags: {}", result.tags.join(", ").cyan());
                        }
                        
                        if let Some(date) = &result.date {
                            println!("   Date: {}", date);
                        }
                        
                        println!("   {}", result.content);
                        println!("   Path: {}", result.path.cyan());
                    }
                },
                Err(e) => {
                    println!("{} {}", "Error:".red().bold(), e);
                    
                    if e.to_string().contains("No search results found") {
                        println!("\nNo results found for the query '{}'.", query);
                        println!("Try using different search terms, or removing filters.");
                    } else if e.to_string().contains("Search index not found") {
                        println!("\nThe search index was not found. Try building it first:");
                        println!("  writing search build");
                    }
                }
            }
        },
        
        Commands::Build { index_path, include_drafts } => {
            println!("{} search index...", "Building".green().bold());
            
            let index_path_str = index_path.as_ref().map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "build/search_index".to_string());
                
            println!("  {} {}", "Index path:".cyan().bold(), index_path_str);
            println!("  {} {}", "Include drafts:".cyan().bold(), include_drafts);
            
            build_index(index_path.as_deref(), include_drafts)?;
            
            println!("{} Search index built successfully", "Success:".green().bold());
        }
    }
    
    Ok(())
} 