# Image Management

This document explains the new approach to managing images in the writing repository.

## Overview

The new approach uses a single high-quality source image per article, which is then transformed into various optimized formats and sizes during the build process. This offers several advantages:

1. **Reduced Storage**: Only one source image per article is stored in the repository
2. **Simplified Management**: No need to manage multiple image files
3. **Consistency**: All derived images come from the same source
4. **Easier Updates**: To update an image, just replace the source file

## Directory Structure

```
content/
  topic/
    article-slug/
      index.jpg       # High-quality source image
      index.mdx       # Article content
```

## Build Process

During the build process, the `image-build` tool:

1. Scans for source images in article directories
2. Generates all required formats (AVIF, WebP, JPEG) and sizes
3. Places them in the build directory

The generated images follow the naming convention defined in `config.yaml`, which is:
```
{slug}-{type}-{width}x{height}.{format}
```

For example:
```
article-slug-featured-1200x630.avif
article-slug-featured-1200x630.webp
article-slug-featured-1200x630.jpg
```

## Prerequisites

The image build process requires:

1. **Rust**: Required for building and running the tools
   - Install with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Then build the tools: `cd tools && cargo build --release`

2. **ImageMagick**: Used for image processing and AVIF conversion
   - On macOS: `brew install imagemagick`
   - On Ubuntu: `sudo apt-get install imagemagick`

## Workflow

### Adding a New Image

1. **Create the source image**:
   ```bash
   make optimize source=path/to/image.jpg article=article-slug
   ```
   This creates a high-quality source image (`index.jpg`) in the article directory.

2. **Generate optimized versions**:
   ```bash
   make images article=article-slug
   ```
   This generates all optimized formats (AVIF, WebP, JPEG) and sizes for the build process.

### Updating an Image

1. Replace the source image:
   ```bash
   make optimize source=path/to/new-image.jpg article=article-slug
   ```

2. Regenerate optimized versions:
   ```bash
   make images article=article-slug
   ```

### Building All Images

```bash
# Build images for all articles
make images

# Build images for a specific article
make images article=article-slug

# Build images for a specific topic
make images topic=topic-name

# Use a different source filename
make images source_filename=custom-image.jpg
```

## Technical Details

The image management system uses two main tools:

1. **image-optimize**: Creates high-quality source images (`index.jpg`) in article directories.
   - Takes a source image and optimizes it for use as the article's source image
   - Ensures the image is of sufficient quality for all derived formats

2. **image-build**: Generates optimized versions of source images for the build process.
   - Reads the configuration from `config.yaml`
   - Processes each source image according to the defined sizes and formats
   - Generates optimized versions using appropriate quality settings
   - Places all generated files in the build directory

For AVIF generation, ImageMagick is used with optimal settings:
- Quality levels defined in `config.yaml` (default: 70% for standard, 65% for thumbnails)
- Speed setting of 2 (good balance between encoding speed and compression)
- AV1 compression for maximum quality and efficiency

## Benefits

- **Better Performance**: Optimized images load faster
- **Reduced Bandwidth**: Smaller file sizes save bandwidth
- **Improved SEO**: Faster loading times improve search engine rankings
- **Better User Experience**: High-quality images with fast loading times
- **Future-Proof**: Easy to add new formats or sizes in the future 