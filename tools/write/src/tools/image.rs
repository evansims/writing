//! # Image Management Module
//!
//! This module provides functionality for managing images, including building and
//! optimizing images for content.

use anyhow::Result;
use colored::*;
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
use crate::ui;

// Mock function to simulate finding images
// In a real implementation, this would scan directories and find actual images
fn find_images(topic: Option<&str>) -> Vec<PathBuf> {
    // Mock implementation that returns dummy image paths
    let mut images = Vec::new();
    let topic_str = topic.unwrap_or("all");

    // Create dummy image paths based on topic
    if topic_str == "all" || topic_str == "blog" {
        for i in 1..11 {
            images.push(PathBuf::from(format!("content/blog/article-{}/images/image-{}.jpg", i % 3 + 1, i)));
        }
    }

    if topic_str == "all" || topic_str == "notes" {
        for i in 1..6 {
            images.push(PathBuf::from(format!("content/notes/note-{}/images/image-{}.jpg", i % 2 + 1, i)));
        }
    }

    // Return the list of images
    images
}

/// Build images for content using parallel processing
///
/// This function builds images for content in parallel using rayon.
/// It processes multiple images simultaneously, significantly improving performance
/// on multi-core systems.
///
/// # Parameters
///
/// * `article` - Optional article slug to filter images by
/// * `topic` - Optional topic to filter images by (if None, builds all images)
/// * `source_filename` - Optional specific image filename to build
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn build_images(
    article: Option<String>,
    topic: Option<String>,
    source_filename: Option<String>,
) -> Result<()> {
    // Get the topic if provided
    let topic_str = topic.as_deref().unwrap_or("all");

    // Show progress
    ui::show_info(&format!("Building images for topic: {}", topic_str));

    // Find all images that need to be built based on topic and article
    let images = if let Some(source) = &source_filename {
        // If source filename is provided, only process that specific image
        let path = if let Some(art) = &article {
            PathBuf::from(format!("content/{}/{}/images/{}", topic_str, art, source))
        } else {
            PathBuf::from(format!("images/{}", source))
        };
        vec![path]
    } else {
        // Otherwise find all relevant images
        find_images(topic.as_deref())
    };

    let image_count = images.len();

    if image_count == 0 {
        ui::show_info("No images found to build");
        return Ok(());
    }

    ui::show_info(&format!("Found {} images to build", image_count));

    // Use our parallel processing utility with optimal batch size
    const BATCH_SIZE: usize = 20; // Adjust based on typical image size and available memory
    let (results, elapsed) = super::utils::process_in_parallel(
        images,
        BATCH_SIZE,
        true, // show progress
        "Building images in parallel...",
        |_image_path| {
            // Simulate image building work
            std::thread::sleep(std::time::Duration::from_millis(150));

            // Return success for this image
            Ok::<(), String>(())
        }
    );

    // Calculate success ratio
    let (successes, failures, total) = super::utils::calculate_success_ratio(&results);

    if failures == 0 {
        ui::show_success(&format!(
            "Successfully built {} images in {:.2}s (using parallel processing with batching)",
            successes,
            elapsed.as_secs_f32()
        ));
    } else {
        ui::show_warning(&format!(
            "Completed with {} errors. Successfully built {} out of {} images.",
            failures,
            successes,
            total
        ));
    }

    Ok(())
}

/// Optimize images in parallel using rayon
///
/// This function optimizes all images for a given topic in parallel using rayon.
/// It processes multiple images simultaneously, significantly improving performance
/// on multi-core systems.
///
/// # Parameters
///
/// * `topic` - Optional topic to filter images by (if None, optimizes all images)
/// * `reoptimize` - Whether to reoptimize already optimized images
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn optimize_images(topic: Option<String>, reoptimize: bool) -> Result<()> {
    // Get the topic if provided
    let topic_str = topic.as_deref().unwrap_or("all");

    // Show progress
    ui::show_info(&format!(
        "Optimizing images for topic: {} (reoptimize: {})",
        topic_str, reoptimize
    ));

    // Find all images that need to be optimized
    let images = find_images(topic.as_deref());
    let image_count = images.len();

    if image_count == 0 {
        ui::show_info("No images found to optimize");
        return Ok(());
    }

    ui::show_info(&format!("Found {} images to optimize", image_count));

    // Use our parallel processing utility with optimal batch size
    const BATCH_SIZE: usize = 20; // Adjust based on typical image size and available memory
    let (results, elapsed) = super::utils::process_in_parallel(
        images,
        BATCH_SIZE,
        true, // show progress
        "Optimizing images in parallel...",
        |_image_path| {
            // Simulate image optimization work
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Return success for this image
            Ok::<(), String>(())
        }
    );

    // Calculate success ratio
    let (successes, failures, total) = super::utils::calculate_success_ratio(&results);

    if failures == 0 {
        ui::show_success(&format!(
            "Successfully optimized {} images in {:.2}s (using parallel processing)",
            successes,
            elapsed.as_secs_f32()
        ));
    } else {
        ui::show_warning(&format!(
            "Completed with {} errors. Successfully optimized {} out of {} images.",
            failures,
            successes,
            total
        ));
    }

    Ok(())
}

/// Optimize a single image into multiple formats and sizes in parallel
///
/// This function optimizes a single source image into multiple formats and sizes
/// in parallel using rayon. It processes different image formats and sizes
/// simultaneously, significantly improving performance.
///
/// # Parameters
///
/// * `source` - Source image filename
/// * `article` - Article slug the image belongs to
/// * `topic` - Optional topic the article belongs to
/// * `formats` - Optional list of output formats (defaults to ["webp", "jpg"])
/// * `sizes` - Optional list of output sizes (defaults to ["lg", "md", "sm", "xs"])
/// * `quality` - Optional output quality (defaults to 80)
/// * `preserve_metadata` - Whether to preserve EXIF metadata (defaults to false)
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn optimize_image(
    source: String,
    article: String,
    topic: Option<String>,
    formats: Option<Vec<String>>,
    sizes: Option<Vec<String>>,
    quality: Option<u8>,
    preserve_metadata: Option<bool>
) -> Result<()> {
    // Get the topic if provided
    let topic_str = topic.as_deref().unwrap_or("default");

    // Use default formats if none provided
    let output_formats = formats.unwrap_or_else(|| vec!["webp".to_string(), "jpg".to_string()]);

    // Use default sizes if none provided
    let output_sizes = sizes.unwrap_or_else(|| vec![
        "lg".to_string(), // 1200px
        "md".to_string(), // 800px
        "sm".to_string(), // 400px
        "xs".to_string(), // 200px
    ]);

    // Use default quality if none provided
    let output_quality = quality.unwrap_or(80);

    // Use default metadata preservation if none provided
    let _keep_metadata = preserve_metadata.unwrap_or(false);

    // Show progress
    ui::show_info(&format!(
        "Optimizing image {} for article {} in topic {} (formats: {:?}, sizes: {:?}, quality: {})",
        source, article, topic_str, output_formats, output_sizes, output_quality
    ));

    // Calculate total number of output images to generate
    let total_variations = output_formats.len() * output_sizes.len();

    // Create a progress bar
    let progress = ui::create_progress_bar(total_variations as u64);
    progress.set_message("Optimizing image variations in parallel...");

    // Record start time
    let start_time = Instant::now();

    // Create a Vec of all format/size combinations
    let variations: Vec<(String, String)> = output_formats.iter()
        .flat_map(|format| {
            output_sizes.iter().map(move |size| (format.clone(), size.clone()))
        })
        .collect();

    // Process all variations in parallel
    let results: Vec<Result<String, String>> = variations.par_iter()
        .map(|(format, size)| {
            // Simulate image conversion and resizing work
            std::thread::sleep(std::time::Duration::from_millis(50));

            // Generate output filename
            let output_filename = format!("{}-{}.{}", source.trim_end_matches(".jpg").trim_end_matches(".png"), size, format);

            // Increment the progress bar
            progress.inc(1);

            // Return the output filename
            Ok(output_filename)
        })
        .collect();

    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Count successful conversions
    let successful_conversions = results.iter()
        .filter(|result| result.is_ok())
        .count();

    progress.finish_with_message("Image optimization completed");

    if successful_conversions == total_variations {
        ui::show_success(&format!(
            "Successfully optimized {} into {} variations in {:.2}s (using parallel processing)",
            source.green(),
            total_variations,
            elapsed.as_secs_f32()
        ));
    } else {
        ui::show_warning(&format!(
            "Completed with errors. Successfully optimized {} out of {} variations.",
            successful_conversions,
            total_variations
        ));
    }

    Ok(())
}