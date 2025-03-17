# Image Processing Features

This document describes the image processing capabilities and how to configure them using feature flags.

## Overview

The image processing system consists of two main crates:
- `image-optimize`: Optimizes individual images with support for multiple formats and sizes
- `image-build`: Processes and builds images for the entire content structure

## Feature Flags

### image-optimize

The `image-optimize` crate supports the following feature flags:

```toml
[features]
default = ["avif", "webp"]  # Both AVIF and WebP enabled by default
avif = ["ravif"]           # AVIF support using ravif
webp = ["dep:webp"]        # WebP support
```

You can customize the build by enabling/disabling features:

```bash
# Build with only WebP support
cargo build --no-default-features --features webp

# Build with only AVIF support
cargo build --no-default-features --features avif

# Build with both (default)
cargo build

# Build with only JPEG support
cargo build --no-default-features
```

### image-build

The `image-build` crate supports the following feature flags:

```toml
[features]
default = ["basic-formats"]
basic-formats = []        # Support for basic image formats (PNG, JPEG, GIF, WebP)
```

Additional format support can be added in the future through new feature flags.

## Supported Formats

The following image formats are supported based on feature flags:

1. JPEG (Always available)
   - Core format, always enabled
   - High-quality compression
   - Excellent browser compatibility

2. WebP (Optional via `webp` feature)
   - Modern format with good compression
   - Supports both lossy and lossless compression
   - Wide browser support
   - Enable with `--features webp`

3. AVIF (Optional via `avif` feature)
   - Next-generation format with excellent compression
   - Best quality-to-size ratio
   - Growing browser support
   - Enable with `--features avif`
   - Requires ImageMagick for `image-build`

## Quality Settings

Quality settings can be configured per format and size in the configuration:

```yaml
images:
  quality:
    jpg:
      standard: 85
      thumbnail: 80
    webp:
      standard: 80
      thumbnail: 75
    avif:
      standard: 70
      thumbnail: 65
```

## Size Variants

The following size variants are supported:

- Original: Maintains original dimensions
- Large: 1200px width (default)
- Medium: 800px width (default)
- Small: 400px width (default)
- Thumbnail: 200px width (default)

## Usage Examples

### Basic Usage

```rust
use image_optimize::{OptimizeOptions, SizeVariant, OutputFormat};

let options = OptimizeOptions {
    source: "path/to/image.jpg".into(),
    article: "my-article".into(),
    formats: image_optimize::default_formats(), // Uses enabled formats
    sizes: image_optimize::default_size_variants(),
    quality: 85,
    preserve_metadata: false,
};
```

### Format-Specific Usage

```rust
// Using specific formats based on feature availability
let mut formats = vec![OutputFormat::Jpeg];

#[cfg(feature = "webp")]
formats.push(OutputFormat::WebP);

#[cfg(feature = "avif")]
formats.push(OutputFormat::Avif);

let options = OptimizeOptions {
    formats,
    ..Default::default()
};
```

## Performance Considerations

1. JPEG processing is always fast and reliable
2. WebP processing has minimal overhead
3. AVIF processing:
   - Provides best compression but is slower
   - CPU-intensive, especially for large images
   - Consider using lower quality settings for thumbnails

## Dependencies

- JPEG support: Built-in via `image` crate
- WebP support: Requires `webp` crate
- AVIF support:
  - `image-optimize`: Uses `ravif` crate
  - `image-build`: Requires ImageMagick with AVIF support

## Recommendations

1. For maximum compatibility:
   - Use default features (AVIF + WebP)
   - Always include JPEG as fallback

2. For faster builds:
   - Disable AVIF if build time is a concern
   - Use `--no-default-features --features webp`

3. For minimal size:
   - Disable optional formats if not needed
   - Use `--no-default-features` 