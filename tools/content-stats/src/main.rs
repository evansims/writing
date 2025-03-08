use anyhow::Result;
use clap::Parser;
use colored::*;
use content_stats::{generate_stats, format_date, StatsOptions};

#[derive(Parser)]
#[command(author, version, about = "Generate content statistics")]
struct Args {
    /// Content slug (optional, will analyze all if not provided)
    #[arg(short, long)]
    slug: Option<String>,
    
    /// Topic (optional)
    #[arg(short, long)]
    topic: Option<String>,
    
    /// Include drafts in statistics
    #[arg(short, long)]
    include_drafts: bool,
    
    /// Sort by (date, words, reading_time)
    #[arg(short, long, default_value = "date")]
    sort_by: String,
    
    /// Show detailed statistics
    #[arg(short, long)]
    detailed: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let options = StatsOptions {
        slug: args.slug,
        topic: args.topic,
        include_drafts: args.include_drafts,
        sort_by: args.sort_by,
        detailed: args.detailed,
    };
    
    let (stats, tag_counts, total_words, total_articles, total_drafts) = generate_stats(&options)?;
    
    // Print statistics
    if options.detailed {
        println!("{}", "Content Statistics (Detailed)".yellow().bold());
        println!("=========================================");
        
        if stats.is_empty() {
            println!("No content found.");
            return Ok(());
        }
        
        for stat in &stats {
            println!("\n{}", stat.title.green().bold());
            println!("  {}: {}", "Topic".cyan(), stat.topic);
            println!("  {}: {}", "Slug".cyan(), stat.slug);
            println!("  {}: {}", "Published".cyan(), format_date(&stat.published));
            println!("  {}: {} words", "Word Count".cyan(), stat.word_count);
            println!("  {}: {} minutes", "Reading Time".cyan(), stat.reading_time);
            println!("  {}: {}", "Character Count".cyan(), stat.character_count);
            println!("  {}: {}", "Paragraph Count".cyan(), stat.paragraph_count);
            println!("  {}: {}", "Sentence Count".cyan(), stat.sentence_count);
            
            if !stat.tags.is_empty() {
                println!("  {}: {}", "Tags".cyan(), stat.tags.join(", "));
            }
            
            if stat.is_draft {
                println!("  {}: {}", "Draft".cyan(), "Yes".red());
            }
        }
        
        // Print tag counts
        if !tag_counts.is_empty() {
            println!("\n{}", "Tag Usage".yellow().bold());
            println!("------------------");
            
            let mut tag_count_vec: Vec<(String, usize)> = tag_counts.into_iter().collect();
            tag_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (tag, count) in tag_count_vec {
                println!("  {}: {}", tag.cyan(), count);
            }
        }
    } else {
        // Print summary statistics
        println!("{}", "Content Statistics".yellow().bold());
        println!("=========================================");
        
        println!("Total Content: {}", total_articles);
        println!("Published Articles: {}", total_articles - total_drafts);
        println!("Drafts: {}", total_drafts);
        println!("Total Words: {}", total_words);
        
        if total_articles > 0 {
            println!("Average Words per Article: {}", total_words / total_articles);
        }
        
        println!("\n{}", "Content List".yellow().bold());
        println!("------------------");
        
        if stats.is_empty() {
            println!("No content found.");
            return Ok(());
        }
        
        for stat in &stats {
            let published_str = format_date(&stat.published);
            let draft_indicator = if stat.is_draft { " [DRAFT]".red() } else { "".normal() };
            
            println!("{} - {}{} - {} words ({} min)", 
                     published_str.cyan(),
                     stat.title,
                     draft_indicator,
                     stat.word_count,
                     stat.reading_time);
        }
    }
    
    Ok(())
} 