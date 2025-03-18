# Image Optimize Tool

Tool for optimizing images for web use.

## Usage

```bash
# Optimize image
./image-optimize "path/to/image.jpg"

# Optimize with quality
./image-optimize "path/to/image.jpg" --quality 80

# Optimize with format
./image-optimize "path/to/image.jpg" --format "webp"
```

## Features

- Image optimization
- Format conversion
- Quality control
- Size reduction
- Batch processing
- Progress reporting

## Output

```
output/
  image.webp    # Optimized WebP
  image.jpg     # Optimized JPEG
  image.avif    # Optimized AVIF
```

## Options

- Quality level
- Output format
- Size limits
- Batch mode
- Progress display
- Output directory

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
