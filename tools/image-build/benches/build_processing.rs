use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image_build::{BuildImagesOptions, process_image};
use common_models::{Config, ContentConfig, ImageConfig, TopicConfig, ImageSize, ImageNaming};
use std::path::PathBuf;
use std::collections::HashMap;
use tempfile::TempDir;

fn create_test_image(dir: &std::path::Path) -> anyhow::Result<PathBuf> {
    let img_path = dir.join("test.jpg");
    let img = image::RgbaImage::new(1920, 1080); // HD resolution for realistic testing
    img.save(&img_path)?;
    Ok(img_path)
}

fn create_test_config() -> Config {
    Config {
        content: ContentConfig {
            base_dir: "content".into(),
            topics: {
                let mut map = HashMap::new();
                map.insert("test-topic".into(), TopicConfig {
                    path: "test-topic".into(),
                    title: "Test Topic".into(),
                    description: None,
                });
                map
            },
        },
        images: ImageConfig {
            sizes: {
                let mut map = HashMap::new();
                map.insert("standard".into(), ImageSize {
                    width: 800,
                    height: 600,
                });
                map.insert("thumbnail".into(), ImageSize {
                    width: 200,
                    height: 150,
                });
                map
            },
            naming: Some(ImageNaming {
                pattern: "{slug}-{type}.{format}".into(),
                examples: vec![],
            }),
            quality: Some({
                let mut map = HashMap::new();
                let mut jpg_settings = HashMap::new();
                jpg_settings.insert("standard".into(), 85);
                jpg_settings.insert("thumbnail".into(), 80);
                map.insert("jpg".into(), jpg_settings);
                
                #[cfg(feature = "basic-formats")]
                {
                    let mut webp_settings = HashMap::new();
                    webp_settings.insert("standard".into(), 80);
                    webp_settings.insert("thumbnail".into(), 75);
                    map.insert("webp".into(), webp_settings);
                }
                
                #[cfg(feature = "avif")]
                {
                    let mut avif_settings = HashMap::new();
                    avif_settings.insert("standard".into(), 70);
                    avif_settings.insert("thumbnail".into(), 65);
                    map.insert("avif".into(), avif_settings);
                }
                
                map
            }),
        },
    }
}

fn bench_basic_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    let output_dir = temp_dir.path().join("output");
    let config = create_test_config();
    
    c.bench_function("process_basic_image", |b| {
        b.iter(|| {
            process_image(
                black_box(&source_path),
                black_box("test-article"),
                black_box("test-topic"),
                black_box(&output_dir),
                black_box(&config),
            ).unwrap();
        })
    });
}

fn bench_size_variants(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    let output_dir = temp_dir.path().join("output");
    let mut config = create_test_config();
    
    let mut group = c.benchmark_group("size_variants");
    
    // Test with different numbers of size variants
    let size_variants = [
        ("single_size", vec![(800, 600)]),
        ("two_sizes", vec![(800, 600), (400, 300)]),
        ("four_sizes", vec![(1200, 900), (800, 600), (400, 300), (200, 150)]),
    ];
    
    for (name, sizes) in size_variants {
        let mut size_map = HashMap::new();
        for (i, (width, height)) in sizes.iter().enumerate() {
            size_map.insert(format!("size_{}", i), ImageSize {
                width: *width,
                height: *height,
            });
        }
        config.images.sizes = size_map;
        
        group.bench_function(name, |b| {
            b.iter(|| {
                process_image(
                    black_box(&source_path),
                    black_box("test-article"),
                    black_box("test-topic"),
                    black_box(&output_dir),
                    black_box(&config),
                ).unwrap();
            })
        });
    }
    
    group.finish();
}

#[cfg(feature = "basic-formats")]
fn bench_format_combinations(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    let output_dir = temp_dir.path().join("output");
    let config = create_test_config();
    
    let mut group = c.benchmark_group("format_combinations");
    group.sample_size(20); // Fewer samples for longer running benchmarks
    
    // Test different format combinations
    group.bench_function("jpeg_only", |b| {
        b.iter(|| {
            process_image(
                black_box(&source_path),
                black_box("test-article"),
                black_box("test-topic"),
                black_box(&output_dir),
                black_box(&config),
            ).unwrap();
        })
    });
    
    #[cfg(feature = "basic-formats")]
    group.bench_function("jpeg_and_webp", |b| {
        b.iter(|| {
            process_image(
                black_box(&source_path),
                black_box("test-article"),
                black_box("test-topic"),
                black_box(&output_dir),
                black_box(&config),
            ).unwrap();
        })
    });
    
    #[cfg(all(feature = "basic-formats", feature = "avif"))]
    group.bench_function("all_formats", |b| {
        b.iter(|| {
            process_image(
                black_box(&source_path),
                black_box("test-article"),
                black_box("test-topic"),
                black_box(&output_dir),
                black_box(&config),
            ).unwrap();
        })
    });
    
    group.finish();
}

fn bench_quality_settings(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    let output_dir = temp_dir.path().join("output");
    let mut config = create_test_config();
    
    let mut group = c.benchmark_group("quality_settings");
    group.sample_size(20); // Fewer samples for longer running benchmarks
    
    let qualities = [60, 75, 85, 95];
    
    for quality in qualities {
        let mut quality_settings = HashMap::new();
        let mut format_settings = HashMap::new();
        format_settings.insert("standard".into(), quality);
        format_settings.insert("thumbnail".into(), quality);
        
        quality_settings.insert("jpg".into(), format_settings.clone());
        #[cfg(feature = "basic-formats")]
        quality_settings.insert("webp".into(), format_settings.clone());
        #[cfg(feature = "avif")]
        quality_settings.insert("avif".into(), format_settings.clone());
        
        config.images.quality = Some(quality_settings);
        
        group.bench_function(format!("quality_{}", quality), |b| {
            b.iter(|| {
                process_image(
                    black_box(&source_path),
                    black_box("test-article"),
                    black_box("test-topic"),
                    black_box(&output_dir),
                    black_box(&config),
                ).unwrap();
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_basic_processing,
    bench_size_variants,
    #[cfg(feature = "basic-formats")]
    bench_format_combinations,
    bench_quality_settings,
);
criterion_main!(benches); 