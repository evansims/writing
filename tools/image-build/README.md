# Image Build Tool

A tool for building and generating responsive image assets for content.

## Features

- Responsive image generation
- Multiple format support
- Quality optimization
- Metadata handling
- Asset organization
- Build caching
- Progress tracking

## Usage

```bash
# Build all images
image-build

# Build specific directory
image-build --dir "content/strategy"

# Build with custom settings
image-build \
  --quality 85 \
  --max-width 1920 \
  --formats "webp,jpg"

# Force rebuild
image-build --force

# Dry run (preview changes)
image-build --dry-run
```

## Build Process

1. **Image Analysis**

   - Read source image
   - Extract metadata
   - Determine dimensions
   - Check cache

2. **Image Processing**

   - Resize images
   - Convert formats
   - Optimize quality
   - Strip metadata

3. **Asset Generation**
   - Create responsive variants
   - Generate thumbnails
   - Update references
   - Save to output

## Output Structure

```
public/
  └── images/
      └── [topic]/
          └── [image_name]/
              ├── original.jpg     # Original file
              ├── optimized.jpg    # Optimized original
              ├── optimized.webp   # WebP variant
              └── responsive/      # Responsive variants
                  ├── 480w.jpg
                  ├── 768w.jpg
                  └── 1024w.jpg
```

## Configuration

Default settings:

```yaml
image:
  quality:
    jpg: 85
    webp: 80
    png: 9
  max_width: 1920
  formats: ["webp", "jpg"]
  responsive:
    enabled: true
    breakpoints: [480, 768, 1024, 1920]
  cache:
    enabled: true
    timeout: 3600
```

## Performance

- Incremental builds
- Parallel processing
- Memory optimization
- Disk caching
- Progress tracking
