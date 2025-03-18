# Image Optimize Tool

A tool for optimizing images used in content, ensuring fast loading and efficient storage.

## Features

- Automatic image optimization
- Multiple format support (JPEG, PNG, WebP)
- Responsive image generation
- Quality preservation
- Metadata stripping
- Batch processing
- Progress tracking

## Usage

```bash
# Optimize all images
image-optimize

# Optimize specific directory
image-optimize --dir "content/strategy"

# Optimize with custom settings
image-optimize \
  --quality 85 \
  --max-width 1920 \
  --formats "webp,jpg"

# Dry run (preview changes)
image-optimize --dry-run
```

## Optimization Process

1. Analyze original image
2. Strip unnecessary metadata
3. Resize if needed
4. Convert to optimal format
5. Apply compression
6. Generate responsive variants
7. Save optimized versions

## Output Formats

For each source image, the tool generates:

```
[image_name]/
  ├── original.jpg     # Original file (preserved)
  ├── optimized.jpg    # Optimized original format
  ├── optimized.webp   # WebP variant
  └── responsive/      # Responsive variants
      ├── 480w.jpg
      ├── 768w.jpg
      └── 1024w.jpg
```

## Configuration

Default settings:

- Max width: 1920px
- JPEG quality: 85
- PNG compression: Maximum
- WebP quality: 80
- Strip metadata: Yes
- Generate responsive: Yes
- Responsive breakpoints: [480, 768, 1024, 1920]
