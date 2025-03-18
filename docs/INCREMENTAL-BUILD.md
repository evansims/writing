# Incremental Building

The build system uses caching to only rebuild content that has changed since the last build.

## Usage

```bash
# Normal build (uses cache)
./write build

# Force rebuild all content
./write build --force
```

## How It Works

1. Build cache tracks:

   - Processed files
   - Last modification times
   - Generated output files

2. During build:
   - Compares file modification times with cache
   - Only rebuilds changed files
   - Updates cache after build

## Benefits

- Faster builds for unchanged content
- Near-instant builds when nothing changed
- Automatic cache management
