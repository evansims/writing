use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about = "Generate table of contents")]
struct Args {
    /// Output file path (default: index.md)
    #[arg(short, long, default_value = "index.md")]
    output: PathBuf,
}

#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: String,
    tagline: String,
    topics: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct TopicConfig {
    name: String,
    description: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct ContentConfig {
    base_dir: String,
    topics: HashMap<String, TopicConfig>,
}

#[derive(Deserialize, Debug)]
struct Config {
    content: ContentConfig,
}

fn extract_frontmatter(content: &str) -> Result<Frontmatter> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid frontmatter format"));
    }
    
    let yaml = parts[1];
    let frontmatter: Frontmatter = serde_yaml::from_str(yaml)
        .context("Failed to parse frontmatter")?;
    
    Ok(frontmatter)
}

fn read_config() -> Result<Config> {
    let config_content = fs::read_to_string("config.yaml")
        .context("Failed to read config.yaml")?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse config.yaml")?;
    
    Ok(config)
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read configuration
    let config = read_config()?;
    
    // Collect articles by topic
    let mut articles: HashMap<String, Vec<(String, String, PathBuf)>> = HashMap::new();
    
    // Initialize articles map with all topics from config
    for (topic_key, _) in &config.content.topics {
        articles.insert(topic_key.clone(), Vec::new());
    }
    
    // Walk through content directory
    for entry in WalkDir::new(&config.content.base_dir).min_depth(3).max_depth(3) {
        let entry = entry?;
        let path = entry.path();
        
        if path.file_name().unwrap_or_default() == "index.mdx" {
            let content = fs::read_to_string(path)
                .context(format!("Failed to read file: {:?}", path))?;
            
            let frontmatter = match extract_frontmatter(&content) {
                Ok(fm) => fm,
                Err(e) => {
                    eprintln!("Warning: Failed to parse frontmatter in {:?}: {}", path, e);
                    continue;
                }
            };
            
            let article_path = path.parent().unwrap();
            let relative_path = article_path.strip_prefix(Path::new("."))?;
            
            for topic in &frontmatter.topics {
                if let Some(articles_for_topic) = articles.get_mut(topic) {
                    articles_for_topic.push((
                        frontmatter.title.clone(),
                        frontmatter.tagline.clone(),
                        relative_path.to_path_buf(),
                    ));
                }
            }
        }
    }
    
    // Generate table of contents
    let mut toc = String::new();
    toc.push_str("# Writing Collection\n\n");
    toc.push_str("A curated collection of personal writings exploring creativity, engineering, focus, mindset, strategy, and tools.\n\n");
    
    // Add table of contents with topic descriptions
    for (topic_key, topic_config) in &config.content.topics {
        toc.push_str(&format!("## {}\n\n", topic_config.name));
        toc.push_str(&format!("{}\n\n", topic_config.description));
        
        if let Some(articles_for_topic) = articles.get(topic_key) {
            if articles_for_topic.is_empty() {
                toc.push_str("*No articles yet*\n\n");
            } else {
                for (title, tagline, path) in articles_for_topic {
                    toc.push_str(&format!("- [{}]({}) - {}\n", title, path.display(), tagline));
                }
                toc.push_str("\n");
            }
        }
    }
    
    toc.push_str("---\n\n");
    toc.push_str("This collection is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).\n");
    
    // Write to output file
    fs::write(&args.output, toc)
        .context(format!("Failed to write to file: {:?}", args.output))?;
    
    println!("{} Table of contents generated at: {:?}", "SUCCESS:".green().bold(), args.output);
    
    Ok(())
}

// Helper function to convert string to title case
fn to_title_case(s: &str) -> String {
    let mut c = 0;
    s.chars().map(|x| {
        if c == 0 || s.chars().nth(c - 1).unwrap_or(' ') == ' ' {
            c += 1;
            x.to_uppercase().next().unwrap_or(x)
        } else {
            c += 1;
            x
        }
    }).collect()
} 