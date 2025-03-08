use anyhow::Result;
use std::process::Command;
use std::path::PathBuf;

// Import refactored tool libraries
use content_new::{ContentOptions, create_content as lib_create_content, get_available_topics};
use content_stats::{StatsOptions, generate_stats, format_date};

// Define the release directory path
const TOOLS_DIR: &str = "tools";
const BIN_DIR: &str = "target/release";

// Function to run a tool directly via binary
pub fn run_tool_command(tool_name: &str, args: &[String]) -> Result<()> {
    // Construct the path to the tool binary
    let mut tool_path = PathBuf::from(TOOLS_DIR);
    tool_path.push(BIN_DIR);
    tool_path.push(tool_name);
    
    // Print a message indicating what tool is being run
    println!("Running tool: {}", tool_name);
    
    // Print the full command with arguments for visibility
    let args_display = args.join(" ");
    println!("Full command: {} {}", tool_path.display(), args_display);
    
    // Execute the tool directly
    let output = Command::new(&tool_path)
        .args(args)
        .output()?;
    
    // Print the command output
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    if !output.status.success() {
        anyhow::bail!("Tool execution failed: {}", tool_name);
    }
    
    println!("Command completed successfully.");
    
    Ok(())
}

// Content management functions using direct library calls
pub fn create_content(
    title: Option<String>,
    topic: Option<String>,
    tagline: Option<String>,
    tags: Option<String>,
    content_type: Option<String>,
    draft: bool,
) -> Result<()> {
    // If any required parameter is missing, fall back to the binary
    if title.is_none() || topic.is_none() || tagline.is_none() || tags.is_none() {
        println!("Using interactive mode via binary...");
        let mut args = Vec::new();
        
        if let Some(title) = title {
            args.push(String::from("--title"));
            args.push(title);
        }
        
        if let Some(topic) = topic {
            args.push(String::from("--topic"));
            args.push(topic);
        }
        
        if let Some(tagline) = tagline {
            args.push(String::from("--tagline"));
            args.push(tagline);
        }
        
        if let Some(tags) = tags {
            args.push(String::from("--tags"));
            args.push(tags);
        }
        
        if let Some(content_type) = content_type {
            args.push(String::from("--content-type"));
            args.push(content_type);
        }
        
        if draft {
            args.push(String::from("--draft"));
        }
        
        run_tool_command("content-new", &args)
    } else {
        // Use direct library call
        println!("Creating content using direct library call...");
        let options = ContentOptions {
            title: title.unwrap(),
            topic: topic.unwrap(),
            tagline: tagline.unwrap(),
            tags: tags.unwrap(),
            content_type: content_type.unwrap_or_else(|| "article".to_string()),
            draft,
        };
        
        let result = lib_create_content(options)?;
        println!("Content created at: {}", result);
        Ok(())
    }
}

pub fn edit_content(slug: Option<String>, topic: Option<String>, frontmatter: bool, content: bool) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(slug) = slug {
        args.push(String::from("--slug"));
        args.push(slug);
    }
    
    if let Some(topic) = topic {
        args.push(String::from("--topic"));
        args.push(topic);
    }
    
    if frontmatter {
        args.push(String::from("--frontmatter"));
    }
    
    if content {
        args.push(String::from("--content"));
    }
    
    run_tool_command("content-edit", &args)
}

pub fn move_content(
    slug: Option<String>,
    new_slug: Option<String>,
    topic: Option<String>,
    new_topic: Option<String>,
) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(slug) = slug {
        args.push(String::from("--slug"));
        args.push(slug);
    }
    
    if let Some(new_slug) = new_slug {
        args.push(String::from("--new-slug"));
        args.push(new_slug);
    }
    
    if let Some(topic) = topic {
        args.push(String::from("--topic"));
        args.push(topic);
    }
    
    if let Some(new_topic) = new_topic {
        args.push(String::from("--new-topic"));
        args.push(new_topic);
    }
    
    run_tool_command("content-move", &args)
}

pub fn delete_content(slug: Option<String>, topic: Option<String>, force: bool) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(slug) = slug {
        args.push(String::from("--slug"));
        args.push(slug);
    }
    
    if let Some(topic) = topic {
        args.push(String::from("--topic"));
        args.push(topic);
    }
    
    if force {
        args.push(String::from("--force"));
    }
    
    run_tool_command("content-delete", &args)
}

pub fn list_content() -> Result<()> {
    // Direct implementation since this uses find in the Makefile
    println!("Listing all content:");
    let output = Command::new("find")
        .arg("content")
        .arg("-type")
        .arg("d")
        .arg("-mindepth")
        .arg("2")
        .arg("-maxdepth")
        .arg("2")
        .output()?;
    
    let dirs = String::from_utf8_lossy(&output.stdout);
    for dir in dirs.lines() {
        let index_path = format!("{}/index.mdx", dir);
        if std::path::Path::new(&index_path).exists() {
            let topic = std::path::Path::new(dir)
                .parent()
                .and_then(|p| p.file_name())
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            
            let article = std::path::Path::new(dir)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            
            // Get the title from the file
            let grep_output = Command::new("grep")
                .arg("-m")
                .arg("1")
                .arg("title:")
                .arg(&index_path)
                .output()?;
            
            let title_line = String::from_utf8_lossy(&grep_output.stdout);
            let title = title_line
                .trim()
                .trim_start_matches("title:")
                .trim()
                .trim_matches('"')
                .to_string();
            
            println!("{}/{} - {}", topic, article, title);
        }
    }
    
    Ok(())
}

pub fn list_topics() -> Result<()> {
    println!("Available Topics:");
    
    // Parse config.yaml directly to extract topic info
    let config_path = "config.yaml";
    let config_content = std::fs::read_to_string(config_path)?;
    
    let mut in_topics = false;
    let mut current_name = String::new();
    
    for line in config_content.lines() {
        if line.contains("topics:") {
            in_topics = true;
            continue;
        }
        
        if in_topics {
            if line.trim().starts_with('-') {
                // New topic entry
                current_name.clear();
            } else if line.trim().starts_with("name:") {
                current_name = line.trim()
                    .trim_start_matches("name:")
                    .trim()
                    .trim_matches('"')
                    .to_string();
                println!("    {}", current_name);
            } else if line.trim().starts_with("description:") && !current_name.is_empty() {
                let description = line.trim()
                    .trim_start_matches("description:")
                    .trim()
                    .trim_matches('"')
                    .to_string();
                println!("      {}", description);
                println!("--");
            }
        }
    }
    
    Ok(())
}

pub fn add_topic(key: Option<String>, name: Option<String>, description: Option<String>, path: Option<String>) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(key) = key {
        args.push(String::from("--key"));
        args.push(key);
    }
    
    if let Some(name) = name {
        args.push(String::from("--name"));
        args.push(name);
    }
    
    if let Some(description) = description {
        args.push(String::from("--description"));
        args.push(description);
    }
    
    if let Some(path) = path {
        args.push(String::from("--path"));
        args.push(path);
    }
    
    run_tool_command("topic-add", &args)
}

pub fn edit_topic(key: Option<String>, name: Option<String>, description: Option<String>) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(key) = key {
        args.push(String::from("--key"));
        args.push(key);
    }
    
    if let Some(name) = name {
        args.push(String::from("--name"));
        args.push(name);
    }
    
    if let Some(description) = description {
        args.push(String::from("--description"));
        args.push(description);
    }
    
    run_tool_command("topic-edit", &args)
}

pub fn rename_topic(
    key: Option<String>,
    new_key: Option<String>,
    new_name: Option<String>,
    new_path: Option<String>,
) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(key) = key {
        args.push(String::from("--key"));
        args.push(key);
    }
    
    if let Some(new_key) = new_key {
        args.push(String::from("--new-key"));
        args.push(new_key);
    }
    
    if let Some(new_name) = new_name {
        args.push(String::from("--new-name"));
        args.push(new_name);
    }
    
    if let Some(new_path) = new_path {
        args.push(String::from("--new-path"));
        args.push(new_path);
    }
    
    run_tool_command("topic-rename", &args)
}

pub fn delete_topic(key: Option<String>, target: Option<String>, force: bool) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(key) = key {
        args.push(String::from("--key"));
        args.push(key);
    }
    
    if let Some(target) = target {
        args.push(String::from("--target"));
        args.push(target);
    }
    
    if force {
        args.push(String::from("--force"));
    }
    
    run_tool_command("topic-delete", &args)
}

pub fn optimize_image(source: String, article: String, topic: Option<String>) -> Result<()> {
    let mut args = Vec::new();
    
    args.push(String::from("--source"));
    args.push(source);
    
    args.push(String::from("--article"));
    args.push(article);
    
    if let Some(topic) = topic {
        args.push(String::from("--topic"));
        args.push(topic);
    }
    
    run_tool_command("image-optimize", &args)
}

pub fn build_images(
    article: Option<String>,
    topic: Option<String>,
    source_filename: Option<String>,
) -> Result<()> {
    let mut args = Vec::new();
    
    args.push(String::from("--output-dir"));
    args.push(String::from("build/images"));
    
    if let Some(article) = article {
        args.push(String::from("--article"));
        args.push(article);
    }
    
    if let Some(topic) = topic {
        args.push(String::from("--topic"));
        args.push(topic);
    }
    
    if let Some(source_filename) = source_filename {
        args.push(String::from("--source-filename"));
        args.push(source_filename);
    }
    
    run_tool_command("image-build", &args)
}

pub fn build_content(
    output_dir: Option<String>,
    slug: Option<String>,
    topic: Option<String>,
    include_drafts: bool,
    template_dir: Option<String>,
    site_url: Option<String>,
) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(output_dir) = output_dir {
        args.push(String::from("--output-dir"));
        args.push(output_dir);
    }
    
    if let Some(slug) = slug {
        args.push(String::from("--slug"));
        args.push(slug);
    }
    
    if let Some(topic) = topic {
        args.push(String::from("--topic"));
        args.push(topic);
    }
    
    if include_drafts {
        args.push(String::from("--include-drafts"));
    }
    
    if let Some(template_dir) = template_dir {
        args.push(String::from("--template-dir"));
        args.push(template_dir);
    }
    
    if let Some(site_url) = site_url {
        args.push(String::from("--site-url"));
        args.push(site_url);
    }
    
    run_tool_command("content-build", &args)
}

pub fn generate_toc(output: Option<String>) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(output) = output {
        args.push(String::from("--output"));
        args.push(output);
    } else {
        args.push(String::from("--output"));
        args.push(String::from("build/index.md"));
    }
    
    run_tool_command("toc-generate", &args)
}

pub fn generate_llms(
    site_url: Option<String>,
    output_dir: Option<String>,
    include_drafts: bool,
) -> Result<()> {
    let mut args = Vec::new();
    
    if let Some(site_url) = site_url {
        args.push(String::from("--site-url"));
        args.push(site_url);
    }
    
    if let Some(output_dir) = output_dir {
        args.push(String::from("--output-dir"));
        args.push(output_dir);
    } else {
        args.push(String::from("--output-dir"));
        args.push(String::from("build"));
    }
    
    if include_drafts {
        args.push(String::from("--include-drafts"));
    }
    
    run_tool_command("llms-generate", &args)
}

pub fn generate_content_stats(
    slug: Option<String>,
    topic: Option<String>,
    include_drafts: bool,
    sort_by: String,
    detailed: bool,
) -> Result<()> {
    println!("Generating content statistics:");
    
    let options = StatsOptions {
        slug,
        topic,
        include_drafts,
        sort_by,
        detailed,
    };
    
    let (stats, tag_counts, total_words, total_articles, total_drafts) = generate_stats(&options)?;
    
    // Print statistics
    if detailed {
        println!("Content Statistics (Detailed)");
        println!("=========================================");
        
        if stats.is_empty() {
            println!("No content found.");
            return Ok(());
        }
        
        for stat in &stats {
            println!("\n{}", stat.title);
            println!("  Topic: {}", stat.topic);
            println!("  Slug: {}", stat.slug);
            println!("  Published: {}", format_date(&stat.published));
            println!("  Word Count: {} words", stat.word_count);
            println!("  Reading Time: {} minutes", stat.reading_time);
            println!("  Character Count: {}", stat.character_count);
            println!("  Paragraph Count: {}", stat.paragraph_count);
            println!("  Sentence Count: {}", stat.sentence_count);
            
            if !stat.tags.is_empty() {
                println!("  Tags: {}", stat.tags.join(", "));
            }
            
            if stat.is_draft {
                println!("  Draft: Yes");
            }
        }
        
        // Print tag counts
        if !tag_counts.is_empty() {
            println!("\nTag Usage");
            println!("------------------");
            
            let mut tag_count_vec: Vec<(String, usize)> = tag_counts.into_iter().collect();
            tag_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (tag, count) in tag_count_vec {
                println!("  {}: {}", tag, count);
            }
        }
    } else {
        // Print summary statistics
        println!("Content Statistics");
        println!("=========================================");
        
        println!("Total Content: {}", total_articles);
        println!("Published Articles: {}", total_articles - total_drafts);
        println!("Drafts: {}", total_drafts);
        println!("Total Words: {}", total_words);
        
        if total_articles > 0 {
            println!("Average Words per Article: {}", total_words / total_articles);
        }
        
        println!("\nContent List");
        println!("------------------");
        
        if stats.is_empty() {
            println!("No content found.");
            return Ok(());
        }
        
        for stat in &stats {
            let published_str = format_date(&stat.published);
            let draft_indicator = if stat.is_draft { " [DRAFT]" } else { "" };
            
            println!("{} - {}{} - {} words ({} min)", 
                     published_str,
                     stat.title,
                     draft_indicator,
                     stat.word_count,
                     stat.reading_time);
        }
    }
    
    Ok(())
} 