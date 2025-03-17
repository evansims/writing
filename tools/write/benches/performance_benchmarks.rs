//! Performance benchmarks for critical operations
//!
//! This file contains benchmarks for measuring the performance of critical operations
//! to establish baseline metrics and track improvements over time.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

// Import the modules to benchmark
use write::tools::*;
use write::cli;

/// Set up a test environment with a temporary directory for benchmarking
fn setup() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();
    
    // Create basic config directory structure
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir).unwrap();
    
    // Create a minimal configuration file
    let config_content = r#"{
        "topics": {
            "blog": {
                "name": "Blog",
                "description": "Blog posts",
                "directory": "content/blog"
            }
        },
        "content": {
            "base_dir": "content"
        }
    }"#;
    
    fs::write(config_dir.join("config.json"), config_content).unwrap();
    
    // Create content directory structure
    let content_dir = temp_dir.path().join("content");
    let blog_dir = content_dir.join("blog");
    fs::create_dir_all(&blog_dir).unwrap();
    
    temp_dir
}

/// Benchmark content creation
fn bench_content_creation(c: &mut Criterion) {
    let temp_dir = setup();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to the temp directory for the benchmark
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    c.bench_function("content_creation", |b| {
        b.iter(|| {
            // Create a unique slug for each iteration
            let slug = format!("test-post-{}", uuid::Uuid::new_v4());
            
            let cmd = cli::ContentCommands::New {
                title: Some(format!("Test Post {}", black_box(slug))),
                topic: Some("blog".to_string()),
                tagline: Some("This is a test post".to_string()),
                tags: Some("test,benchmark".to_string()),
                draft: false,
                template: None,
                edit: false,
            };
            
            let result = execute_content_command(cmd);
            assert!(result.is_ok());
        })
    });
    
    // Restore the original directory
    std::env::set_current_dir(original_dir).unwrap();
}

/// Benchmark frontmatter parsing and validation
fn bench_frontmatter_validation(c: &mut Criterion) {
    let temp_dir = setup();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to the temp directory for the benchmark
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Create test content
    let article_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&article_dir).unwrap();
    
    let article_content = r#"---
title: Test Article
date: 2023-01-01
tags: test,article,benchmark,performance,metrics
description: This is a longer description that includes more text to benchmark performance of frontmatter parsing
keywords: [testing, benchmark, performance, metrics, validation, frontmatter]
author: Test Author
image: assets/cover.jpg
---

# Test Article

This is a test article for benchmarking frontmatter validation.
"#;
    
    fs::write(article_dir.join("index.mdx"), article_content).unwrap();
    
    c.bench_function("frontmatter_validation", |b| {
        b.iter(|| {
            let cmd = cli::ContentCommands::Edit {
                slug: Some("test-article".to_string()),
                topic: Some("blog".to_string()),
                field: Some("keywords".to_string()),
                value: Some(black_box("[testing, benchmark, performance, metrics, validation, updated]".to_string())),
                editor: false,
            };
            
            let result = execute_content_command(cmd);
            assert!(result.is_ok());
        })
    });
    
    // Restore the original directory
    std::env::set_current_dir(original_dir).unwrap();
}

/// Benchmark content building
fn bench_content_building(c: &mut Criterion) {
    let temp_dir = setup();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to the temp directory for the benchmark
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Create multiple articles for building
    for i in 1..21 {
        let article_dir = temp_dir.path().join(format!("content/blog/test-article-{}", i));
        fs::create_dir_all(&article_dir).unwrap();
        
        let article_content = format!(r#"---
title: Test Article {}
date: 2023-01-{}
tags: test,article,benchmark
---

# Test Article {}

This is test article {} for benchmarking content building.
"#, i, i, i, i);
        
        fs::write(article_dir.join("index.mdx"), article_content).unwrap();
    }
    
    // Create the public directory
    fs::create_dir_all(temp_dir.path().join("public")).unwrap();
    
    c.bench_function("content_building", |b| {
        b.iter(|| {
            let cmd = cli::BuildCommands::Content {
                topic: Some("blog".to_string()),
                rebuild: true,
            };
            
            let result = execute_build_command(cmd);
            assert!(result.is_ok());
        })
    });
    
    // Restore the original directory
    std::env::set_current_dir(original_dir).unwrap();
}

/// Benchmark image optimization
fn bench_image_optimization(c: &mut Criterion) {
    let temp_dir = setup();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to the temp directory for the benchmark
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Create test content with images
    let article_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&article_dir).unwrap();
    
    let assets_dir = article_dir.join("assets");
    fs::create_dir_all(&assets_dir).unwrap();
    
    let article_content = r#"---
title: Test Article
date: 2023-01-01
tags: test,article,benchmark
---

# Test Article

This is a test article with images:

![Test Image 1](assets/test-image-1.jpg)
![Test Image 2](assets/test-image-2.jpg)
![Test Image 3](assets/test-image-3.jpg)
"#;
    
    fs::write(article_dir.join("index.mdx"), article_content).unwrap();
    
    // Create dummy image files
    let image_data = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        0x01, 0x01, 0x00, 0x60, 0x00, 0x60, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43,
        0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        // Add more bytes to make a larger file
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    
    for i in 1..4 {
        fs::write(assets_dir.join(format!("test-image-{}.jpg", i)), &image_data).unwrap();
    }
    
    c.bench_function("image_optimization", |b| {
        b.iter(|| {
            let cmd = cli::ImageCommands::Optimize {
                slug: Some("test-article".to_string()),
                topic: Some("blog".to_string()),
                all: true,
            };
            
            let result = execute_image_command(cmd);
            assert!(result.is_ok());
        })
    });
    
    // Restore the original directory
    std::env::set_current_dir(original_dir).unwrap();
}

/// Benchmark statistics generation
fn bench_statistics_generation(c: &mut Criterion) {
    let temp_dir = setup();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to the temp directory for the benchmark
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Create multiple articles with varied content
    for i in 1..21 {
        let article_dir = temp_dir.path().join(format!("content/blog/test-article-{}", i));
        fs::create_dir_all(&article_dir).unwrap();
        
        // Create a more substantial article with varying length for better benchmarking
        let article_content = format!(r#"---
title: Test Article {}
date: 2023-01-{}
tags: test,article,benchmark,stats{}
---

# Test Article {}

This is test article {} for benchmarking statistics generation.

## Section 1

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
{}

## Section 2

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
{}

## Section 3

Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
{}
"#, 
            i, i, i % 5, i, i, 
            // Add varying length paragraphs
            "Lorem ipsum dolor sit amet. ".repeat(i % 10 + 1),
            "Consectetur adipiscing elit. ".repeat(i % 8 + 1),
            "Excepteur sint occaecat cupidatat non proident. ".repeat(i % 6 + 1)
        );
        
        fs::write(article_dir.join("index.mdx"), article_content).unwrap();
    }
    
    c.bench_function("statistics_generation", |b| {
        b.iter(|| {
            let cmd = cli::StatsCommands::Generate {
                topic: Some("blog".to_string()),
            };
            
            let result = execute_stats_command(cmd);
            assert!(result.is_ok());
        })
    });
    
    // Restore the original directory
    std::env::set_current_dir(original_dir).unwrap();
}

// Configure the benchmark groups
criterion_group!(
    benches,
    bench_content_creation,
    bench_frontmatter_validation,
    bench_content_building,
    bench_image_optimization,
    bench_statistics_generation
);
criterion_main!(benches); 