use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use image::{DynamicImage, GenericImageView, ImageFormat, RgbaImage};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::tempdir;

// Import the image-build crate
use image_build::{
    build_images, 
    types::{BuildConfig, ContentConfig, ImageConfig, ImageSize, NamingConfig, QualityConfig}
};

// Import common models
use common_models::content::ContentItem;

// Create a test image with various patterns for more realistic benchmarking
fn create_test_image(width: u32, height: u32, pattern: &str) -> DynamicImage {
    let mut img = RgbaImage::new(width, height);
    
    match pattern {
        "gradient" => {
            // Create a gradient pattern
            for y in 0..height {
                for x in 0..width {
                    let r = (x as f32 / width as f32 * 255.0) as u8;
                    let g = (y as f32 / height as f32 * 255.0) as u8;
                    let b = ((x + y) as f32 / (width + height) as f32 * 255.0) as u8;
                    img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
                }
            }
        },
        "checkerboard" => {
            // Create a checkerboard pattern
            let square_size = 32;
            for y in 0..height {
                for x in 0..width {
                    let is_white = ((x / square_size) + (y / square_size)) % 2 == 0;
                    let color = if is_white { 255 } else { 0 };
                    img.put_pixel(x, y, image::Rgba([color, color, color, 255]));
                }
            }
        },
        "noise" => {
            // Create a noisy pattern
            use rand::Rng;
            let mut rng = rand::thread_rng();
            for y in 0..height {
                for x in 0..width {
                    let r = rng.gen_range(0..255);
                    let g = rng.gen_range(0..255);
                    let b = rng.gen_range(0..255);
                    img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
                }
            }
        },
        "photo" => {
            // Simulate a photo-like image with areas of detail and smooth gradients
            for y in 0..height {
                for x in 0..width {
                    let center_x = width as f32 / 2.0;
                    let center_y = height as f32 / 2.0;
                    let dx = (x as f32 - center_x) / center_x;
                    let dy = (y as f32 - center_y) / center_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    let r = ((1.0 - distance) * 255.0) as u8;
                    let g = ((0.5 - distance * 0.5) * 255.0) as u8;
                    let b = ((dx.abs() * 0.5 + dy.abs() * 0.5) * 255.0) as u8;
                    img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
                }
            }
        },
        _ => {
            // Default to a solid color
            for y in 0..height {
                for x in 0..width {
                    img.put_pixel(x, y, image::Rgba([100, 150, 200, 255]));
                }
            }
        }
    }
    
    DynamicImage::ImageRgba8(img)
}

// Save a test image to a temporary file
fn save_test_image(img: &DynamicImage, format: ImageFormat, dir: &tempfile::TempDir) -> PathBuf {
    let extension = match format {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::WebP => "webp",
        ImageFormat::Avif => "avif",
        _ => "png",
    };
    
    let path = dir.path().join(format!("test.{}", extension));
    img.save_with_format(&path, format).expect("Failed to save test image");
    path
}

// Create a test content item
fn create_test_content_item(image_path: &PathBuf, title: &str) -> ContentItem {
    ContentItem {
        id: format!("test-{}", title.to_lowercase().replace(' ', "-")),
        title: title.to_string(),
        description: format!("Test description for {}", title),
        content: "Test content".to_string(),
        image: image_path.to_string_lossy().to_string(),
        date: chrono::Utc::now(),
        tags: vec!["test".to_string(), "benchmark".to_string()],
        ..Default::default()
    }
}

// Create a build configuration with various settings
fn create_build_config(
    sizes: Vec<ImageSize>,
    formats: Vec<&str>,
    quality_jpeg: u8,
    quality_webp: u8,
    quality_avif: u8
) -> BuildConfig {
    let mut config = BuildConfig {
        content: ContentConfig {
            source_dir: "".to_string(),
            output_dir: "".to_string(),
            content_type: "test".to_string(),
        },
        images: ImageConfig {
            sizes,
            formats: formats.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        },
        naming: NamingConfig {
            pattern: "{id}-{width}x{height}.{ext}".to_string(),
            thumbnail_suffix: "-thumb".to_string(),
        },
        quality: QualityConfig {
            jpeg: quality_jpeg,
            jpeg_thumbnail: quality_jpeg.saturating_sub(5),
            webp: quality_webp,
            webp_thumbnail: quality_webp.saturating_sub(5),
            avif: quality_avif,
            avif_thumbnail: quality_avif.saturating_sub(5),
        },
        ..Default::default()
    };
    
    config
}

// Benchmark different image patterns
fn bench_image_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_image_patterns");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let patterns = vec!["gradient", "checkerboard", "noise", "photo"];
    
    for pattern in patterns {
        let img = create_test_image(1920, 1080, pattern);
        let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
        let content_item = create_test_content_item(&path, &format!("Test {}", pattern));
        
        let config = create_build_config(
            vec![ImageSize::new(1920, 1080)],
            vec!["jpg"],
            85,
            85,
            85
        );
        
        group.bench_with_input(BenchmarkId::new("pattern", pattern), &content_item, |b, item| {
            b.iter(|| {
                let output_dir = tempdir().expect("Failed to create output dir");
                let mut config_clone = config.clone();
                config_clone.content.output_dir = output_dir.path().to_string_lossy().to_string();
                
                build_images(&vec![item.clone()], &config_clone).expect("Failed to build images");
            });
        });
    }
    
    group.finish();
}

// Benchmark different size configurations
fn bench_size_configurations(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_size_configurations");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1920, 1080, "photo");
    let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
    let content_item = create_test_content_item(&path, "Test Photo");
    
    let size_configs = vec![
        (vec![ImageSize::new(1920, 1080)], "single_size"),
        (vec![ImageSize::new(1920, 1080), ImageSize::new(960, 540)], "two_sizes"),
        (vec![
            ImageSize::new(1920, 1080),
            ImageSize::new(1280, 720),
            ImageSize::new(960, 540),
            ImageSize::new(640, 360)
        ], "four_sizes"),
    ];
    
    for (sizes, name) in size_configs {
        let config = create_build_config(
            sizes,
            vec!["jpg"],
            85,
            85,
            85
        );
        
        group.bench_with_input(BenchmarkId::new("sizes", name), &content_item, |b, item| {
            b.iter(|| {
                let output_dir = tempdir().expect("Failed to create output dir");
                let mut config_clone = config.clone();
                config_clone.content.output_dir = output_dir.path().to_string_lossy().to_string();
                
                build_images(&vec![item.clone()], &config_clone).expect("Failed to build images");
            });
        });
    }
    
    group.finish();
}

// Benchmark different format combinations
fn bench_format_combinations(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_format_combinations");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1280, 720, "photo");
    let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
    let content_item = create_test_content_item(&path, "Test Photo");
    
    let format_configs = vec![
        (vec!["jpg"], "jpeg_only"),
        (vec!["jpg", "webp"], "jpeg_webp"),
        #[cfg(feature = "avif")]
        (vec!["jpg", "webp", "avif"], "all_formats"),
    ];
    
    for (formats, name) in format_configs {
        let config = create_build_config(
            vec![ImageSize::new(1280, 720)],
            formats,
            85,
            85,
            85
        );
        
        group.bench_with_input(BenchmarkId::new("formats", name), &content_item, |b, item| {
            b.iter(|| {
                let output_dir = tempdir().expect("Failed to create output dir");
                let mut config_clone = config.clone();
                config_clone.content.output_dir = output_dir.path().to_string_lossy().to_string();
                
                build_images(&vec![item.clone()], &config_clone).expect("Failed to build images");
            });
        });
    }
    
    group.finish();
}

// Benchmark different quality settings
fn bench_quality_settings(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_quality_settings");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1280, 720, "photo");
    let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
    let content_item = create_test_content_item(&path, "Test Photo");
    
    let qualities = vec![60, 75, 85, 95];
    
    for quality in qualities {
        let config = create_build_config(
            vec![ImageSize::new(1280, 720)],
            vec!["jpg"],
            quality,
            quality,
            quality
        );
        
        group.bench_with_input(BenchmarkId::new("quality", quality), &content_item, |b, item| {
            b.iter(|| {
                let output_dir = tempdir().expect("Failed to create output dir");
                let mut config_clone = config.clone();
                config_clone.content.output_dir = output_dir.path().to_string_lossy().to_string();
                
                build_images(&vec![item.clone()], &config_clone).expect("Failed to build images");
            });
        });
    }
    
    group.finish();
}

// Benchmark batch processing
fn bench_batch_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_batch_processing");
    group.measurement_time(Duration::from_secs(15));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let batch_sizes = vec![1, 5, 10, 20];
    
    // Create a set of test content items
    let mut content_items = Vec::new();
    for i in 0..20 {
        let pattern = match i % 4 {
            0 => "gradient",
            1 => "checkerboard",
            2 => "noise",
            _ => "photo",
        };
        let img = create_test_image(1280, 720, pattern);
        let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
        let content_item = create_test_content_item(&path, &format!("Test Item {}", i));
        content_items.push(content_item);
    }
    
    let config = create_build_config(
        vec![ImageSize::new(1280, 720)],
        vec!["jpg"],
        85,
        85,
        85
    );
    
    for batch_size in batch_sizes {
        group.bench_with_input(BenchmarkId::new("batch_size", batch_size), &batch_size, |b, &size| {
            b.iter(|| {
                let output_dir = tempdir().expect("Failed to create output dir");
                let mut config_clone = config.clone();
                config_clone.content.output_dir = output_dir.path().to_string_lossy().to_string();
                
                let batch = &content_items[0..size];
                build_images(batch, &config_clone).expect("Failed to build images");
            });
        });
    }
    
    group.finish();
}

// Benchmark WebP-specific configurations
#[cfg(feature = "basic-formats")]
fn bench_webp_configurations(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_webp_configurations");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1280, 720, "photo");
    let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
    let content_item = create_test_content_item(&path, "Test Photo");
    
    let qualities = vec![60, 75, 85, 95];
    
    for quality in qualities {
        let config = create_build_config(
            vec![ImageSize::new(1280, 720)],
            vec!["jpg", "webp"],
            85,
            quality,
            85
        );
        
        group.bench_with_input(BenchmarkId::new("webp_quality", quality), &content_item, |b, item| {
            b.iter(|| {
                let output_dir = tempdir().expect("Failed to create output dir");
                let mut config_clone = config.clone();
                config_clone.content.output_dir = output_dir.path().to_string_lossy().to_string();
                
                build_images(&vec![item.clone()], &config_clone).expect("Failed to build images");
            });
        });
    }
    
    group.finish();
}

// Benchmark AVIF-specific configurations
#[cfg(feature = "avif")]
fn bench_avif_configurations(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_avif_configurations");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1280, 720, "photo");
    let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
    let content_item = create_test_content_item(&path, "Test Photo");
    
    let qualities = vec![60, 75, 85, 95];
    
    for quality in qualities {
        let config = create_build_config(
            vec![ImageSize::new(1280, 720)],
            vec!["jpg", "webp", "avif"],
            85,
            85,
            quality
        );
        
        group.bench_with_input(BenchmarkId::new("avif_quality", quality), &content_item, |b, item| {
            b.iter(|| {
                let output_dir = tempdir().expect("Failed to create output dir");
                let mut config_clone = config.clone();
                config_clone.content.output_dir = output_dir.path().to_string_lossy().to_string();
                
                build_images(&vec![item.clone()], &config_clone).expect("Failed to build images");
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_image_patterns,
    bench_size_configurations,
    bench_format_combinations,
    bench_quality_settings,
    bench_batch_processing
);

#[cfg(feature = "basic-formats")]
criterion_group!(webp_benches, bench_webp_configurations);

#[cfg(feature = "avif")]
criterion_group!(avif_benches, bench_avif_configurations);

#[cfg(all(feature = "basic-formats", feature = "avif"))]
criterion_main!(benches, webp_benches, avif_benches);

#[cfg(all(feature = "basic-formats", not(feature = "avif")))]
criterion_main!(benches, webp_benches);

#[cfg(all(not(feature = "basic-formats"), feature = "avif"))]
criterion_main!(benches, avif_benches);

#[cfg(not(any(feature = "basic-formats", feature = "avif")))]
criterion_main!(benches); 