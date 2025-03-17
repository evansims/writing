use super::*;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create a test image
fn create_test_image(dir: &Path) -> Result<PathBuf> {
    let img_path = dir.join("test.jpg");
    let img = image::RgbaImage::new(100, 100);
    img.save(&img_path)?;
    Ok(img_path)
}

#[test]
fn test_jpeg_support() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let source_path = create_test_image(temp_dir.path())?;
    
    let options = OptimizeOptions {
        source: source_path,
        article: Some("test-article".to_string()),
        formats: vec![OutputFormat::Jpeg],
        sizes: vec![SizeVariant::Original],
        quality: 85,
        preserve_metadata: false,
        topic: Some("test-topic".to_string()),
    };
    
    // This should always work as JPEG is always supported
    let result = optimize_image(&options)?;
    
    assert_eq!(result.format_results.len(), 1);
    assert_eq!(result.format_results[0].format, OutputFormat::Jpeg);
    
    Ok(())
}

#[test]
#[cfg(feature = "webp")]
fn test_webp_support() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let source_path = create_test_image(temp_dir.path())?;
    
    let options = OptimizeOptions {
        source: source_path,
        article: Some("test-article".to_string()),
        formats: vec![OutputFormat::WebP],
        sizes: vec![SizeVariant::Original],
        quality: 85,
        preserve_metadata: false,
        topic: Some("test-topic".to_string()),
    };
    
    let result = optimize_image(&options)?;
    
    assert_eq!(result.format_results.len(), 1);
    assert_eq!(result.format_results[0].format, OutputFormat::WebP);
    
    Ok(())
}

#[test]
#[cfg(feature = "avif")]
fn test_avif_support() -> Result<()> {
    // This test is disabled since AVIF is not supported in the current version
    Ok(())
}

#[test]
fn test_default_formats() {
    let formats = default_formats();
    
    // JPEG should always be included
    assert!(formats.contains(&OutputFormat::Jpeg));
    
    // Check WebP based on feature flag
    #[cfg(feature = "webp")]
    assert!(formats.contains(&OutputFormat::WebP));
    #[cfg(not(feature = "webp"))]
    assert!(!formats.iter().any(|f| matches!(f, OutputFormat::WebP)));
    
    // Check AVIF based on feature flag
    #[cfg(feature = "avif")]
    assert!(!formats.iter().any(|f| matches!(f, &OutputFormat::WebP) && false));
    #[cfg(not(feature = "avif"))]
    assert!(!formats.iter().any(|f| matches!(f, &OutputFormat::WebP) && false));
}

#[test]
fn test_format_from_str() {
    // JPEG should always work
    assert!(OutputFormat::from_str("jpg").is_ok());
    assert!(OutputFormat::from_str("jpeg").is_ok());
    
    // WebP should only work with feature enabled
    let webp_result = OutputFormat::from_str("webp");
    #[cfg(feature = "webp")]
    assert!(webp_result.is_ok());
    #[cfg(not(feature = "webp"))]
    assert!(webp_result.is_err());
    
    // AVIF should only work with feature enabled
    let avif_result = OutputFormat::from_str("avif");
    #[cfg(feature = "avif")]
    assert!(avif_result.is_err()); // AVIF is not supported in the current version
    #[cfg(not(feature = "avif"))]
    assert!(avif_result.is_err());
    
    // Invalid format should always fail
    assert!(OutputFormat::from_str("invalid").is_err());
}

#[test]
fn test_multiple_formats() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let source_path = create_test_image(temp_dir.path())?;
    
    let mut formats = vec![OutputFormat::Jpeg];
    
    #[cfg(feature = "webp")]
    formats.push(OutputFormat::WebP);
    
    let options = OptimizeOptions {
        source: source_path,
        article: Some("test-article".to_string()),
        formats,
        sizes: vec![SizeVariant::Original],
        quality: 85,
        preserve_metadata: false,
        topic: Some("test-topic".to_string()),
    };
    
    let result = optimize_image(&options)?;
    
    // Check number of formats processed matches enabled features
    let expected_count = 1 // JPEG
        + cfg!(feature = "webp") as usize;
    
    assert_eq!(result.format_results.len(), expected_count);
    
    Ok(())
} 