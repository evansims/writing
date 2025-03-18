# AVIF Generation

AVIF (AV1 Image File Format) is automatically generated for all images during build, providing superior compression and quality.

## Usage

```bash
# Generate all images (including AVIF)
./write build --site-url "https://example.com"
```

## HTML Implementation

```html
<picture>
  <source srcset="image.avif" type="image/avif" />
  <source srcset="image.webp" type="image/webp" />
  <img src="image.jpg" alt="Description" />
</picture>
```

## Benefits

- 50% smaller than JPEG
- 20% smaller than WebP
- Better quality at lower sizes
- HDR support
- Alpha transparency
- Wide color gamut

## Requirements

- ImageMagick with AVIF support
- AVIF support in browser for optimal viewing
