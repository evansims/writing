# Image Management

Image management capabilities within the content system.

## Features

- Image organization by article/content
- Responsive image generation
- Format conversion (JPEG, WebP, AVIF)
- Optimization for web delivery
- Metadata handling

## Configuration

Image management parameters are defined in config.yaml:

- Quality settings for each format
- Size breakpoints for responsive images
- Optimization level
- Metadata preservation options

## Generated Formats

The build process generates:

- AVIF format (modern, high compression)
- WebP format (broad browser support)
- JPEG format (fallback)

Each format is generated in multiple sizes for responsive design.

## Requirements

- ImageMagick installed for image processing
- AVIF support in ImageMagick for AVIF generation
