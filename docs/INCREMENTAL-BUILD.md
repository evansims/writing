# Incremental Building

This document describes the incremental building feature implemented in the `write` CLI tool.

## Overview

Incremental building significantly improves build performance by only rebuilding content files that have changed since the last build. This is particularly valuable for large content repositories, where rebuilding all files can be time-consuming.

## How It Works

The incremental building system works through the following mechanism:

1. A build cache maintains a record of:

   - Which files have been processed
   - When they were last modified
   - What output files were generated for each input file
   - When the last build occurred

2. When a build is requested, the tool:

   - Compares the last modified time of each content file with its record in the cache
   - Only rebuilds files that are new or have been modified since they were last built
   - Updates the cache with the new build information

3. The cache is persisted between builds in a JSON file, which is read at the start of a build and updated at the end.

## Usage

Incremental building is enabled by default. Simply use the build command as usual:

```bash
./write build
```

If you want to force a full rebuild of all content (ignoring the cache), use the `--force` flag:

```bash
./write build --force
```

## Technical Implementation

The implementation consists of:

### Build Cache Module

- `BuildCache` struct - The main data structure that tracks file modifications and outputs
- Thread-safe global instance using `once_cell::sync::Lazy` and `std::sync::Mutex`
- Functions for loading, saving, and manipulating the cache

### File Detection and Processing

- `find_content_files` - Finds all content files matching specified criteria
- `get_files_to_rebuild` - Determines which files need to be rebuilt based on the cache
- Parallel processing using the `rayon` crate for efficient rebuilding

### Output Management

- Tracking of generated output files for each input file
- Cleanup of stale output files when source files change

## Performance Impact

The incremental building feature provides the following performance benefits:

- First build: Same performance as before (all files are built)
- Subsequent builds with no changes: Near-instant completion
- Builds with some changes: Only changed files are rebuilt

For large repositories, this can reduce build times from minutes to seconds when only a few files have changed.

## Implementation Notes

The incremental build system was implemented by:

1. Creating a build cache module that tracks file modifications and outputs
2. Integrating file system operations to detect changes
3. Implementing parallel processing with batching for efficient rebuilding
4. Adding detailed progress reporting

## Requirements

The incremental build feature requires the following crate dependencies:

- `serde` and `serde_json` for serializing and deserializing the build cache
- `rayon` for parallel processing
- `once_cell` for lazy initialization of the global cache instance
