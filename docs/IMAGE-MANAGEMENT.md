# Image Management

Images are managed using a single high-quality source image per article, which is transformed into optimized formats during build.

## Directory Structure

```
content/
  topic/
    article-slug/
      index.jpg       # Source image
      index.mdx       # Article content
```

## Usage

```bash
# Optimize a new image
./write image-optimize --source path/to/image.jpg --article article-slug

# Build all images
./write build --site-url "https://example.com"
```

## Generated Images

The build process generates:

- AVIF format (modern, high compression)
- WebP format (broad browser support)
- JPEG format (fallback)

Each format is generated in multiple sizes for responsive design.

## Requirements

- ImageMagick installed for image processing
- AVIF support in ImageMagick for AVIF generation
