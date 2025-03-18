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
fn setup() -> Result<tempfile::TempDir, std::io::Error> {
    let temp_dir = tempdir()?;

    // Create basic config directory structure
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir)?;

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

    fs::write(config_dir.join("config.json"), config_content)?;

    // Create content directory structure
    let content_dir = temp_dir.path().join("content");
    let blog_dir = content_dir.join("blog");
    fs::create_dir_all(&blog_dir)?;

    Ok(temp_dir)
}

/// Benchmark content creation
fn bench_content_creation(c: &mut Criterion) {
    let temp_dir = setup().expect("Failed to set up benchmark environment");
    let original_dir = std::env::current_dir().expect("Failed to get current directory");

    // We'll benchmark the time it takes to create a new content file
    let mut group = c.benchmark_group("Content Creation");

    group.bench_function("create_article", |b| {
        b.iter(|| {
            // For each iteration, we need to change to the temp directory
            std::env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");

            // Create a new article
            let result = content::create_content(
                black_box("Test Article"),
                black_box("blog"),
                black_box(Some("This is a test article for benchmarking")),
                black_box(Some("test,article,benchmark")),
                black_box(false),
                black_box(None),
                black_box(false),
            );

            // Assert that the operation succeeded
            assert!(result.is_ok());

            // Clean up - remove the created file to ensure each iteration starts fresh
            let _ = fs::remove_dir_all(temp_dir.path().join("content/blog/test-article"));
        });
    });

    group.finish();

    // Restore original directory after benchmarking
    std::env::set_current_dir(original_dir).expect("Failed to restore original directory");
}

/// Benchmark frontmatter parsing and validation
fn bench_frontmatter_validation(c: &mut Criterion) {
    let temp_dir = setup().unwrap();
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
    let temp_dir = setup().unwrap();
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
    let temp_dir = setup().unwrap();
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
    let temp_dir = setup().unwrap();
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

/// Benchmark parallel vs sequential image processing
fn bench_parallel_image_processing(c: &mut Criterion) {
    let temp_dir = setup().expect("Failed to set up benchmark environment");
    let original_dir = std::env::current_dir().expect("Failed to get current directory");

    // Change to the temp directory for the benchmark
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");

    // Create test content with images
    let article_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&article_dir).expect("Failed to create article directory");

    let assets_dir = article_dir.join("assets");
    fs::create_dir_all(&assets_dir).expect("Failed to create assets directory");

    // Create dummy image file
    let image_data = vec![0xFF; 10000]; // 10KB dummy image data
    fs::write(assets_dir.join("test-image.jpg"), &image_data)
        .expect("Failed to write test image file");

    let mut group = c.benchmark_group("Image Processing");

    // Benchmark sequential implementation
    group.bench_function("sequential_processing", |b| {
        b.iter(|| {
            // Sequential implementation - process each format and size one at a time
            let formats = vec!["webp", "jpg", "avif"];
            let sizes = vec!["lg", "md", "sm", "xs"];

            for format in &formats {
                for size in &sizes {
                    // Simulate image conversion and resizing work
                    std::thread::sleep(std::time::Duration::from_millis(5));

                    // Generate output filename (just to simulate the full process)
                    let _output_filename = format!("test-image-{}.{}", size, format);
                }
            }
        });
    });

    // Benchmark parallel implementation
    group.bench_function("parallel_processing", |b| {
        b.iter(|| {
            // Parallel implementation using rayon
            let formats = vec!["webp", "jpg", "avif"];
            let sizes = vec!["lg", "md", "sm", "xs"];

            // Create a Vec of all format/size combinations
            let variations: Vec<(&str, &str)> = formats.iter()
                .flat_map(|format| {
                    sizes.iter().map(move |size| (*format, *size))
                })
                .collect();

            // Process all variations in parallel
            let _results: Vec<String> = variations.par_iter()
                .map(|(format, size)| {
                    // Simulate image conversion and resizing work
                    std::thread::sleep(std::time::Duration::from_millis(5));

                    // Generate output filename
                    format!("test-image-{}.{}", size, format)
                })
                .collect();
        });
    });

    group.finish();

    // Restore the original directory
    std::env::set_current_dir(original_dir).expect("Failed to restore original directory");
}

/// Benchmark configuration loading
fn bench_config_loading(c: &mut Criterion) {
    let temp_dir = setup().expect("Failed to set up benchmark environment");
    let original_dir = std::env::current_dir().expect("Failed to get current directory");

    // Change to the temp directory for the benchmark
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change to temp directory");

    // Create a more complex configuration file to benchmark loading
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    // Generate a large configuration with many topics
    let mut topics = String::new();
    for i in 1..101 {
        topics.push_str(&format!(r#"
        "topic-{}": {{
            "name": "Topic {}",
            "description": "Description for topic {}",
            "directory": "content/topic-{}"
        }},
"#, i, i, i, i));
    }

    let config_content = format!(r#"
publication:
  title: "Test Publication"
  author: "Test Author"
  site: "https://example.com"
  description: "A test publication for benchmarking"
  copyright: "2023"

content:
  base_dir: "content"
  topics:
    {}

images:
  formats:
    - webp
    - jpg
    - avif
  sizes:
    - 1200
    - 800
    - 600
    - 400
    - 200
  quality: 80
  format_descriptions:
    webp: "Next-gen format with excellent compression"
    jpg: "Standard format with good compatibility"
    avif: "Advanced format with superior compression"
"#, topics);

    fs::write(temp_dir.path().join("config.yaml"), config_content)
        .expect("Failed to write config file");

    let mut group = c.benchmark_group("Configuration Loading");

    // Benchmark eagerly loading configuration
    group.bench_function("eager_loading", |b| {
        b.iter(|| {
            // Force clear the cache before each iteration
            // This simulates eager loading behavior
            crate::config::clear_config_cache();

            // Load the configuration
            let config = crate::config::get_config();
            assert!(config.is_ok());
        });
    });

    // Benchmark lazy loading configuration
    group.bench_function("lazy_loading", |b| {
        // Set up phase - load the config once to cache it
        let _ = crate::config::get_config();

        b.iter(|| {
            // With lazy loading, this should use the cached config
            let config = crate::config::get_config();
            assert!(config.is_ok());

            // Access some data to ensure we're getting the full config
            let config = config.unwrap();
            let _ = &config.publication.author;
            let _ = config.content.topics.len();
        });
    });

    group.finish();

    // Restore the original directory
    std::env::set_current_dir(original_dir).expect("Failed to restore original directory");
}

// Configure the benchmark groups
criterion_group!(
    benches,
    bench_content_creation,
    bench_frontmatter_validation,
    bench_content_building,
    bench_image_optimization,
    bench_statistics_generation,
    bench_parallel_image_processing,
    bench_config_loading
);
criterion_main!(benches);