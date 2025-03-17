use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use image::{DynamicImage, GenericImageView, ImageFormat, RgbaImage};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tempfile::tempdir;

#[cfg(feature = "webp")]
use webp::Encoder as WebPEncoder;

#[cfg(feature = "avif")]
use ravif::Encoder as AvifEncoder;

// Import the image-optimize crate
use image_optimize::{
    optimize_image, optimize_image_bytes,
    types::{ImageHandler, ImageSize, OptimizeContext, ResizeMode},
};

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

// Create an optimization context with various settings
fn create_optimize_context(quality: u8, resize_mode: ResizeMode, sizes: Vec<ImageSize>) -> OptimizeContext {
    let mut context = OptimizeContext::new();
    context.jpeg_quality = quality;
    
    #[cfg(feature = "webp")]
    {
        context.webp_quality = quality;
    }
    
    #[cfg(feature = "avif")]
    {
        context.avif_quality = quality;
    }
    
    context.resize_mode = resize_mode;
    context.sizes = sizes;
    
    context
}

// Benchmark different image patterns
fn bench_image_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_patterns");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let patterns = vec!["gradient", "checkerboard", "noise", "photo"];
    
    for pattern in patterns {
        let img = create_test_image(1920, 1080, pattern);
        let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
        
        let context = create_optimize_context(85, ResizeMode::Lanczos3, vec![
            ImageSize::new(1920, 1080),
        ]);
        
        group.bench_with_input(BenchmarkId::new("pattern", pattern), &path, |b, path| {
            b.iter(|| {
                let mut handler = ImageHandler::new();
                optimize_image(path, &context, &mut handler).expect("Failed to optimize image");
            });
        });
    }
    
    group.finish();
}

// Benchmark different resize algorithms
fn bench_resize_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("resize_algorithms");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1920, 1080, "photo");
    let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
    
    let resize_modes = vec![
        (ResizeMode::Nearest, "Nearest"),
        (ResizeMode::Triangle, "Triangle"),
        (ResizeMode::CatmullRom, "CatmullRom"),
        (ResizeMode::Gaussian, "Gaussian"),
        (ResizeMode::Lanczos3, "Lanczos3"),
    ];
    
    for (mode, name) in resize_modes {
        let context = create_optimize_context(85, mode, vec![
            ImageSize::new(960, 540),
        ]);
        
        group.bench_with_input(BenchmarkId::new("algorithm", name), &path, |b, path| {
            b.iter(|| {
                let mut handler = ImageHandler::new();
                optimize_image(path, &context, &mut handler).expect("Failed to optimize image");
            });
        });
    }
    
    group.finish();
}

// Benchmark different quality settings with different image types
fn bench_quality_vs_image_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("quality_vs_image_type");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let patterns = vec!["gradient", "checkerboard", "noise", "photo"];
    let qualities = vec![60, 75, 85, 95];
    
    for pattern in patterns {
        let img = create_test_image(1280, 720, pattern);
        let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
        
        for quality in &qualities {
            let context = create_optimize_context(*quality, ResizeMode::Lanczos3, vec![
                ImageSize::new(1280, 720),
            ]);
            
            group.bench_with_input(
                BenchmarkId::new(format!("{}_quality", pattern), quality), 
                &path, 
                |b, path| {
                    b.iter(|| {
                        let mut handler = ImageHandler::new();
                        optimize_image(path, &context, &mut handler).expect("Failed to optimize image");
                    });
                }
            );
        }
    }
    
    group.finish();
}

// Benchmark memory usage during optimization
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let sizes = vec![(1280, 720), (1920, 1080), (2560, 1440), (3840, 2160)];
    
    for (width, height) in sizes {
        let img = create_test_image(width, height, "photo");
        let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
        
        let context = create_optimize_context(85, ResizeMode::Lanczos3, vec![
            ImageSize::new(width, height),
        ]);
        
        group.bench_with_input(
            BenchmarkId::new("resolution", format!("{}x{}", width, height)), 
            &path, 
            |b, path| {
                b.iter(|| {
                    let mut handler = ImageHandler::new();
                    optimize_image(path, &context, &mut handler).expect("Failed to optimize image");
                });
            }
        );
    }
    
    group.finish();
}

// Benchmark parallel processing of multiple images
fn bench_parallel_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_processing");
    group.measurement_time(Duration::from_secs(15));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let batch_sizes = vec![1, 2, 4, 8];
    
    // Create a set of test images
    let mut paths = Vec::new();
    for i in 0..8 {
        let pattern = match i % 4 {
            0 => "gradient",
            1 => "checkerboard",
            2 => "noise",
            _ => "photo",
        };
        let img = create_test_image(1280, 720, pattern);
        let path = save_test_image(&img, ImageFormat::Jpeg, &temp_dir);
        paths.push(path);
    }
    
    let context = create_optimize_context(85, ResizeMode::Lanczos3, vec![
        ImageSize::new(1280, 720),
    ]);
    
    for batch_size in batch_sizes {
        group.bench_with_input(
            BenchmarkId::new("batch_size", batch_size), 
            &batch_size, 
            |b, &batch_size| {
                b.iter(|| {
                    let batch = &paths[0..batch_size];
                    let results: Vec<_> = batch.iter().map(|path| {
                        let mut handler = ImageHandler::new();
                        optimize_image(path, &context, &mut handler).expect("Failed to optimize image");
                        handler
                    }).collect();
                    results
                });
            }
        );
    }
    
    group.finish();
}

// Benchmark format-specific optimizations
#[cfg(feature = "webp")]
fn bench_webp_specific(c: &mut Criterion) {
    let mut group = c.benchmark_group("webp_specific");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1280, 720, "photo");
    let path = save_test_image(&img, ImageFormat::WebP, &temp_dir);
    
    let qualities = vec![60, 75, 85, 95];
    
    for quality in qualities {
        let mut context = create_optimize_context(85, ResizeMode::Lanczos3, vec![
            ImageSize::new(1280, 720),
        ]);
        context.webp_quality = quality;
        
        group.bench_with_input(
            BenchmarkId::new("webp_quality", quality), 
            &path, 
            |b, path| {
                b.iter(|| {
                    let mut handler = ImageHandler::new();
                    optimize_image(path, &context, &mut handler).expect("Failed to optimize image");
                });
            }
        );
    }
    
    group.finish();
}

// Benchmark AVIF-specific optimizations
#[cfg(feature = "avif")]
fn bench_avif_specific(c: &mut Criterion) {
    let mut group = c.benchmark_group("avif_specific");
    group.measurement_time(Duration::from_secs(10));
    
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let img = create_test_image(1280, 720, "photo");
    let path = save_test_image(&img, ImageFormat::Avif, &temp_dir);
    
    let qualities = vec![60, 75, 85, 95];
    
    for quality in qualities {
        let mut context = create_optimize_context(85, ResizeMode::Lanczos3, vec![
            ImageSize::new(1280, 720),
        ]);
        context.avif_quality = quality;
        
        group.bench_with_input(
            BenchmarkId::new("avif_quality", quality), 
            &path, 
            |b, path| {
                b.iter(|| {
                    let mut handler = ImageHandler::new();
                    optimize_image(path, &context, &mut handler).expect("Failed to optimize image");
                });
            }
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_image_patterns,
    bench_resize_algorithms,
    bench_quality_vs_image_type,
    bench_memory_usage,
    bench_parallel_processing
);

#[cfg(feature = "webp")]
criterion_group!(webp_benches, bench_webp_specific);

#[cfg(feature = "avif")]
criterion_group!(avif_benches, bench_avif_specific);

#[cfg(all(feature = "webp", feature = "avif"))]
criterion_main!(benches, webp_benches, avif_benches);

#[cfg(all(feature = "webp", not(feature = "avif")))]
criterion_main!(benches, webp_benches);

#[cfg(all(not(feature = "webp"), feature = "avif"))]
criterion_main!(benches, avif_benches);

#[cfg(not(any(feature = "webp", feature = "avif")))]
criterion_main!(benches); 