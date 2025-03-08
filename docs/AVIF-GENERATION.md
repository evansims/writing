# AVIF Image Generation

This document explains how AVIF images are generated in the writing repository.

## Overview

AVIF (AV1 Image File Format) is a modern image format that offers superior compression and quality compared to traditional formats like JPEG and even newer formats like WebP. This repository automatically generates AVIF versions of all images during the build process.

## How AVIF Generation Works

AVIF generation is fully integrated into the image build process. When you run `make images`, the system:

1. Finds all source images (`index.jpg`) in article directories
2. Generates optimized versions in multiple formats, including AVIF
3. Places them in the build directory with a consistent naming pattern

## Prerequisites

AVIF generation requires:

1. **ImageMagick** with AVIF support
   - On macOS: `brew install imagemagick`
   - On Ubuntu: `sudo apt-get install imagemagick`

2. **Rust** (for building the tools)
   - Install with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Then build the tools: `cd tools && cargo build --release`

## Usage

To generate AVIF images:

```bash
# Generate images for all articles
make images

# Generate images for a specific article
make images article=article-slug

# Generate images for a specific topic
make images topic=topic-name
```

## Technical Details

The `image-build` tool:

1. Reads the configuration from `config.yaml`
2. For each source image, it generates multiple sizes as defined in the config
3. For each size, it creates versions in all configured formats (AVIF, WebP, JPEG)
4. For AVIF specifically, it uses ImageMagick's convert command with enhanced settings:
   - Quality levels defined in `config.yaml` (default: 70% for standard, 65% for thumbnails)
   - Speed setting of 2 (good balance between encoding speed and compression)
   - AV1 compression for maximum quality and efficiency

These optimized settings provide the best balance between image quality, file size, and encoding performance.

## Naming Convention

All generated images follow the naming convention defined in `config.yaml`:

```
{slug}-{type}-{width}x{height}.{format}
```

For example:
```
article-slug-featured-1200x630.avif
```

## HTML Usage

To use AVIF images in HTML with appropriate fallbacks:

```html
<picture>
  <source srcset="article-slug-featured-1200x630.avif" type="image/avif">
  <source srcset="article-slug-featured-1200x630.webp" type="image/webp">
  <img src="article-slug-featured-1200x630.jpg" alt="Description">
</picture>
```

This ensures that browsers will use the best format they support, with AVIF being the preferred option when available.

## Benefits of AVIF

- **Superior Compression**: AVIF files are typically 50% smaller than JPEG and 20% smaller than WebP
- **Better Quality**: AVIF maintains higher quality at lower file sizes
- **HDR Support**: AVIF supports high dynamic range imaging
- **Alpha Transparency**: AVIF supports alpha channel transparency
- **Wide Color Gamut**: AVIF supports wide color gamuts

## Workflow

The complete workflow for managing images with AVIF support:

1. **Create the source image**:
   ```bash
   make optimize source=path/to/image.jpg article=article-slug
   ```
   This creates a high-quality source image (`index.jpg`) in the article directory.

2. **Generate optimized versions including AVIF**:
   ```bash
   make images article=article-slug
   ```
   This generates all optimized formats (AVIF, WebP, JPEG) and sizes for the build process.

For more details on image management, see [IMAGE-MANAGEMENT.md](IMAGE-MANAGEMENT.md).

## Legacy Script Removal

Previously, a separate script (`generate-avif.sh`) was used to generate AVIF versions of existing WebP and JPEG images. This script has been removed as AVIF generation is now fully integrated into the main image build process.

The current approach is more efficient because:
1. It generates all formats from a single high-quality source image
2. It eliminates the need for a separate AVIF generation step
3. It ensures consistent quality across all image formats 