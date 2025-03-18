# Image Build Tool

Tool for generating responsive images.

## Usage

```bash
# Build images
./image-build "path/to/image.jpg"

# Build with sizes
./image-build "path/to/image.jpg" --sizes "800,1200,1600"

# Build with formats
./image-build "path/to/image.jpg" --formats "webp,avif"
```

## Features

- Responsive images
- Multiple formats
- Multiple sizes
- Quality control
- Batch processing
- Progress reporting

## Output

```
output/
  image-800.webp
  image-800.avif
  image-1200.webp
  image-1200.avif
  image-1600.webp
  image-1600.avif
```

## Options

- Image sizes
- Output formats
- Quality levels
- Batch mode
- Progress display
- Output directory
