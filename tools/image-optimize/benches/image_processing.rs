use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image_optimize::{OptimizeOptions, SizeVariant, OutputFormat};
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_image(dir: &std::path::Path) -> anyhow::Result<PathBuf> {
    let img_path = dir.join("test.jpg");
    let img = image::RgbaImage::new(1920, 1080); // HD resolution for realistic testing
    img.save(&img_path)?;
    Ok(img_path)
}

fn bench_jpeg_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    
    let options = OptimizeOptions {
        source: source_path.clone(),
        article: "test-article".into(),
        formats: vec![OutputFormat::Jpeg],
        sizes: vec![
            SizeVariant::Original,
            SizeVariant::Large(1200),
            SizeVariant::Medium(800),
            SizeVariant::Small(400),
            SizeVariant::Thumbnail(200),
        ],
        quality: 85,
        preserve_metadata: false,
        topic: Some("test-topic".into()),
    };
    
    c.bench_function("process_jpeg_all_sizes", |b| {
        b.iter(|| {
            image_optimize::optimize_image(black_box(&options)).unwrap();
        })
    });
}

#[cfg(feature = "webp")]
fn bench_webp_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    
    let options = OptimizeOptions {
        source: source_path.clone(),
        article: "test-article".into(),
        formats: vec![OutputFormat::WebP],
        sizes: vec![
            SizeVariant::Original,
            SizeVariant::Large(1200),
            SizeVariant::Medium(800),
            SizeVariant::Small(400),
            SizeVariant::Thumbnail(200),
        ],
        quality: 85,
        preserve_metadata: false,
        topic: Some("test-topic".into()),
    };
    
    c.bench_function("process_webp_all_sizes", |b| {
        b.iter(|| {
            image_optimize::optimize_image(black_box(&options)).unwrap();
        })
    });
}

#[cfg(feature = "avif")]
fn bench_avif_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    
    let options = OptimizeOptions {
        source: source_path.clone(),
        article: "test-article".into(),
        formats: vec![OutputFormat::Avif],
        sizes: vec![
            SizeVariant::Original,
            SizeVariant::Large(1200),
            SizeVariant::Medium(800),
            SizeVariant::Small(400),
            SizeVariant::Thumbnail(200),
        ],
        quality: 85,
        preserve_metadata: false,
        topic: Some("test-topic".into()),
    };
    
    c.bench_function("process_avif_all_sizes", |b| {
        b.iter(|| {
            image_optimize::optimize_image(black_box(&options)).unwrap();
        })
    });
}

fn bench_single_size_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    
    let mut group = c.benchmark_group("single_size_processing");
    
    // JPEG single size
    let jpeg_options = OptimizeOptions {
        source: source_path.clone(),
        article: "test-article".into(),
        formats: vec![OutputFormat::Jpeg],
        sizes: vec![SizeVariant::Medium(800)],
        quality: 85,
        preserve_metadata: false,
        topic: Some("test-topic".into()),
    };
    
    group.bench_function("jpeg_medium_size", |b| {
        b.iter(|| {
            image_optimize::optimize_image(black_box(&jpeg_options)).unwrap();
        })
    });
    
    // WebP single size
    #[cfg(feature = "webp")]
    {
        let webp_options = OptimizeOptions {
            source: source_path.clone(),
            article: "test-article".into(),
            formats: vec![OutputFormat::WebP],
            sizes: vec![SizeVariant::Medium(800)],
            quality: 85,
            preserve_metadata: false,
            topic: Some("test-topic".into()),
        };
        
        group.bench_function("webp_medium_size", |b| {
            b.iter(|| {
                image_optimize::optimize_image(black_box(&webp_options)).unwrap();
            })
        });
    }
    
    // AVIF single size
    #[cfg(feature = "avif")]
    {
        let avif_options = OptimizeOptions {
            source: source_path.clone(),
            article: "test-article".into(),
            formats: vec![OutputFormat::Avif],
            sizes: vec![SizeVariant::Medium(800)],
            quality: 85,
            preserve_metadata: false,
            topic: Some("test-topic".into()),
        };
        
        group.bench_function("avif_medium_size", |b| {
            b.iter(|| {
                image_optimize::optimize_image(black_box(&avif_options)).unwrap();
            })
        });
    }
    
    group.finish();
}

fn bench_quality_impact(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let source_path = create_test_image(temp_dir.path()).unwrap();
    
    let mut group = c.benchmark_group("quality_impact");
    group.sample_size(20); // Fewer samples for longer running benchmarks
    
    let qualities = [60, 75, 85, 95];
    
    for quality in qualities {
        // JPEG quality test
        let jpeg_options = OptimizeOptions {
            source: source_path.clone(),
            article: "test-article".into(),
            formats: vec![OutputFormat::Jpeg],
            sizes: vec![SizeVariant::Large(1200)],
            quality,
            preserve_metadata: false,
            topic: Some("test-topic".into()),
        };
        
        group.bench_function(format!("jpeg_quality_{}", quality), |b| {
            b.iter(|| {
                image_optimize::optimize_image(black_box(&jpeg_options)).unwrap();
            })
        });
        
        // WebP quality test
        #[cfg(feature = "webp")]
        {
            let webp_options = OptimizeOptions {
                source: source_path.clone(),
                article: "test-article".into(),
                formats: vec![OutputFormat::WebP],
                sizes: vec![SizeVariant::Large(1200)],
                quality,
                preserve_metadata: false,
                topic: Some("test-topic".into()),
            };
            
            group.bench_function(format!("webp_quality_{}", quality), |b| {
                b.iter(|| {
                    image_optimize::optimize_image(black_box(&webp_options)).unwrap();
                })
            });
        }
        
        // AVIF quality test
        #[cfg(feature = "avif")]
        {
            let avif_options = OptimizeOptions {
                source: source_path.clone(),
                article: "test-article".into(),
                formats: vec![OutputFormat::Avif],
                sizes: vec![SizeVariant::Large(1200)],
                quality,
                preserve_metadata: false,
                topic: Some("test-topic".into()),
            };
            
            group.bench_function(format!("avif_quality_{}", quality), |b| {
                b.iter(|| {
                    image_optimize::optimize_image(black_box(&avif_options)).unwrap();
                })
            });
        }
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_jpeg_processing,
    #[cfg(feature = "webp")]
    bench_webp_processing,
    #[cfg(feature = "avif")]
    bench_avif_processing,
    bench_single_size_processing,
    bench_quality_impact,
);
criterion_main!(benches); 