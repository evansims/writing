use anyhow::Result;
use std::process::Command;
use std::path::PathBuf;

// Import refactored tool libraries
use content_new::{ContentOptions, get_available_topics};
use content_stats::{StatsOptions, generate_stats, format_date};
use content_edit::{EditOptions, edit_content as lib_edit_content, save_edited_content};
use content_delete::{DeleteOptions, delete_content as lib_delete_content};
use content_move::{MoveOptions, move_content as lib_move_content};
use content_build::{BuildOptions, build_content as lib_build_content};
use image_optimize::{OptimizeOptions, optimize_image as lib_optimize_image};
use image_build::{BuildImagesOptions, build_images as lib_build_images};
use topic_add::{TopicAddOptions, add_topic as lib_add_topic};
use topic_edit::{TopicEditOptions, edit_topic as lib_edit_topic};
use topic_rename::{TopicRenameOptions, rename_topic as lib_rename_topic};
use topic_delete::{TopicDeleteOptions, delete_topic as lib_delete_topic};
use common_config::load_config;
use dialoguer::Editor;
use toc_generate::{TocOptions, generate_toc as lib_generate_toc};
use common_templates;
use llms_generate::{LlmsOptions, generate_llms as lib_generate_llms};

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
    template: Option<String>,
    introduction: Option<String>,
) -> Result<()> {
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

    if let Some(template) = template {
        args.push(String::from("--template"));
        args.push(template);
    }

    if let Some(intro) = introduction {
        args.push(String::from("--introduction"));
        args.push(intro);
    }

    run_tool_command("content-new", &args)
}

pub fn edit_content(
    slug: Option<String>,
    topic: Option<String>,
    frontmatter: bool,
    content: bool,
) -> Result<()> {
    // Create options for editing
    let options = EditOptions {
        slug,
        topic: topic.clone(),
        frontmatter_only: frontmatter,
        content_only: content,
    };
    
    // If slug is not provided, we need to fall back to the binary for interactive selection
    if options.slug.is_none() {
        let mut args = Vec::new();
        
        if let Some(topic_val) = topic {
            args.push(String::from("--topic"));
            args.push(topic_val);
        }
        
        if frontmatter {
            args.push(String::from("--frontmatter"));
        }
        
        if content {
            args.push(String::from("--content"));
        }
        
        return run_tool_command("content-edit", &args);
    }
    
    // Use direct library call for editing
    println!("Editing content using direct library call...");
    
    // Get the content to edit
    let (content_path, content_to_edit) = match lib_edit_content(&options) {
        Ok(result) => result,
        Err(e) => {
            // If we get an error, show the error and return
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };
    
    // Open the content in an editor
    if let Some(edited_content) = Editor::new().edit(&content_to_edit)? {
        // Save the edited content
        save_edited_content(&content_path, &edited_content)?;
        println!("Content updated successfully");
        Ok(())
    } else {
        println!("Edit cancelled");
        Ok(())
    }
}

pub fn move_content(
    slug: Option<String>,
    new_slug: Option<String>,
    topic: Option<String>,
    new_topic: Option<String>,
) -> Result<()> {
    // If any required parameter is missing, fall back to the binary
    if slug.is_none() {
        println!("Using interactive mode via binary...");
        let mut args = Vec::new();
        
        if let Some(s) = slug {
            args.push(String::from("--slug"));
            args.push(s);
        }
        
        if let Some(ns) = new_slug {
            args.push(String::from("--new-slug"));
            args.push(ns);
        }
        
        if let Some(t) = topic {
            args.push(String::from("--topic"));
            args.push(t);
        }
        
        if let Some(nt) = new_topic {
            args.push(String::from("--new-topic"));
            args.push(nt);
        }
        
        return run_tool_command("content-move", &args);
    }
    
    // Create options for content movement
    let options = MoveOptions {
        slug: slug.clone(),
        new_slug: new_slug.clone(),
        topic: topic.clone(),
        new_topic: new_topic.clone(),
    };
    
    // Call the library function and handle the result
    match lib_move_content(&options) {
        Ok((current_topic, current_slug, new_topic, new_slug)) => {
            println!("Content moved from '{}/{}' to '{}/{}'", 
                current_topic, current_slug, 
                new_topic, new_slug
            );
            Ok(())
        },
        Err(e) => Err(e.into())
    }
}

pub fn delete_content(slug: Option<String>, topic: Option<String>, force: bool) -> Result<()> {
    // Create options for content deletion
    let options = DeleteOptions {
        slug,
        topic,
        force,
    };
    
    // If slug is not provided, we need to fall back to the binary for interactive selection
    if options.slug.is_none() {
        let mut args = Vec::new();
        
        if let Some(topic) = options.topic.clone() {
            args.push(String::from("--topic"));
            args.push(topic);
        }
        
        if options.force {
            args.push(String::from("--force"));
        }
        
        return run_tool_command("content-delete", &args);
    }
    
    // Otherwise, use the library function
    match lib_delete_content(&options) {
        Ok((topic, slug, title)) => {
            println!("Content deleted: {}/{} ({})", topic, slug, title);
            Ok(())
        },
        Err(_) => {
            // If there's an error, fall back to the binary
            let mut args = Vec::new();
            
            if let Some(slug) = options.slug {
                args.push(String::from("--slug"));
                args.push(slug);
            }
            
            if let Some(topic) = options.topic {
                args.push(String::from("--topic"));
                args.push(topic);
            }
            
            if options.force {
                args.push(String::from("--force"));
            }
            
            run_tool_command("content-delete", &args)
        }
    }
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
    // Create options and add topic using the library function
    let options = TopicAddOptions {
        key,
        name,
        description,
        path,
    };
    
    // Call the library function and handle the result
    match lib_add_topic(&options) {
        Ok(topic_key) => {
            println!("Topic '{}' added successfully", topic_key);
            Ok(())
        },
        Err(e) => Err(e)
    }
}

pub fn edit_topic(key: Option<String>, name: Option<String>, description: Option<String>) -> Result<()> {
    // Create options and edit topic using the library function
    let options = TopicEditOptions {
        key,
        name,
        description,
    };
    
    // Call the library function and handle the result
    match lib_edit_topic(&options) {
        Ok(topic_key) => {
            println!("Topic '{}' updated successfully", topic_key);
            Ok(())
        },
        Err(e) => Err(e)
    }
}

pub fn rename_topic(
    key: Option<String>,
    new_key: Option<String>,
    new_name: Option<String>,
    new_path: Option<String>,
) -> Result<()> {
    // Create options and rename topic using the library function
    let options = TopicRenameOptions {
        key,
        new_key,
        new_name,
        new_path,
    };
    
    // Call the library function and handle the result
    match lib_rename_topic(&options) {
        Ok(topic_key) => {
            println!("Topic renamed to '{}' successfully", topic_key);
            Ok(())
        },
        Err(e) => Err(e)
    }
}

pub fn delete_topic(key: Option<String>, target: Option<String>, force: bool) -> Result<()> {
    // Create options and delete topic using the library function
    let options = TopicDeleteOptions {
        key,
        target,
        force,
    };
    
    // Call the library function and handle the result
    match lib_delete_topic(&options) {
        Ok(topic_key) => {
            println!("Topic '{}' deleted successfully", topic_key);
            Ok(())
        },
        Err(e) => Err(e)
    }
}

pub fn optimize_image(source: String, article: String, topic: Option<String>) -> Result<()> {
    println!("Optimizing image using direct library call...");
    
    // Convert parameters to an OptimizeOptions struct
    let options = OptimizeOptions {
        source: PathBuf::from(source),
        article,
        topic,
    };
    
    // Call the library function
    let target_path = lib_optimize_image(&options)?;
    
    println!("Image optimized successfully and saved to: {}", target_path.display());
    println!("To generate all image formats, run: build images --article={}", options.article);
    
    Ok(())
}

pub fn build_images(
    article: Option<String>,
    topic: Option<String>,
    source_filename: Option<String>,
) -> Result<()> {
    println!("Building images using direct library call...");
    
    // Create options
    let options = BuildImagesOptions {
        output_dir: PathBuf::from("build/images"),
        source_dir: PathBuf::from("content"),
        source_filename: source_filename.unwrap_or_else(|| "index.jpg".to_string()),
        article,
        topic,
    };
    
    // Call the library function
    match lib_build_images(&options) {
        Ok((total_articles, total_images, processed_images, skipped_articles)) => {
            println!("Image build complete!");
            println!("Articles scanned: {}, Images found: {}, Processed: {}, Skipped: {}", 
                total_articles, total_images, processed_images, skipped_articles);
            Ok(())
        },
        Err(e) => {
            eprintln!("Error building images: {}", e);
            Err(e)
        }
    }
}

pub fn build_content(
    output_dir: Option<String>,
    slug: Option<String>,
    topic: Option<String>,
    include_drafts: bool,
    skip_html: bool,
    skip_json: bool,
    skip_rss: bool,
    skip_sitemap: bool,
    verbose: bool,
) -> Result<()> {
    // Create options and build content using the library function
    let options = BuildOptions {
        output_dir,
        slug,
        topic,
        include_drafts,
        skip_html,
        skip_json,
        skip_rss,
        skip_sitemap,
        verbose,
    };
    
    lib_build_content(&options)
}

pub fn generate_toc(output: Option<String>) -> Result<()> {
    // Create options for TOC generation
    let output_path = output.unwrap_or_else(|| "build/index.md".to_string());
    let options = TocOptions {
        output: std::path::PathBuf::from(output_path),
        title: None, // Use default title
        description: None, // Use default description
    };
    
    // Call the library function and handle the result
    match lib_generate_toc(&options) {
        Ok(output_path) => {
            println!("Table of contents generated at: {:?}", output_path);
            Ok(())
        },
        Err(e) => Err(e)
    }
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

/// List available templates
pub fn list_templates() -> Result<()> {
    // Use the common_templates crate directly
    let templates = common_templates::list_templates()?;
    
    if templates.is_empty() {
        println!("No templates found.");
        return Ok(());
    }
    
    println!("Available templates:");
    for template in templates {
        println!("  - {} ({})", template.name, template.content_type);
    }
    
    Ok(())
}

/// Create a new template
pub fn create_template(
    name: Option<String>,
    content_type: Option<String>,
) -> Result<()> {
    let mut args = Vec::new();

    if let Some(name) = name {
        args.push(String::from("--name"));
        args.push(name);
    }

    if let Some(content_type) = content_type {
        args.push(String::from("--content-type"));
        args.push(content_type);
    }

    run_tool_command("content-template", &args)
}

pub fn generate_llms(
    site_url: Option<String>,
    output_dir: Option<String>,
    include_drafts: bool,
) -> Result<()> {
    // Create options for LLMS generation
    let output_path = output_dir.unwrap_or_else(|| "build".to_string());
    let options = LlmsOptions {
        output_dir: std::path::PathBuf::from(output_path),
        site_url,
        include_drafts,
    };
    
    // Call the library function and handle the result
    match lib_generate_llms(&options) {
        Ok((llms_txt_path, llms_full_txt_path)) => {
            println!("Generated LLMS files: {} and {}", 
                llms_txt_path.display(), 
                llms_full_txt_path.display());
            Ok(())
        },
        Err(e) => Err(e)
    }
} 