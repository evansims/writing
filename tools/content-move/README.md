# Content Move Tool

A tool for reorganizing content within the content management system.

## Features

- Content relocation
- Topic reassignment
- Slug updates
- Asset migration
- Reference updates
- Link validation
- Dry run mode

## Usage

```bash
# Move content to new topic
content-move strategy/my-article business

# Move with new slug
content-move strategy/my-article business/new-slug

# Dry run (preview changes)
content-move strategy/my-article business --dry-run

# Move multiple content
content-move strategy/* business

# Move with asset preservation
content-move strategy/my-article business --preserve-assets
```

## Move Process

1. **Pre-move Checks**

   - Validate source content
   - Check target location
   - Verify permissions
   - Validate new slug

2. **Content Migration**

   - Move content files
   - Update metadata
   - Migrate assets
   - Update references

3. **Post-move Tasks**
   - Update build cache
   - Regenerate TOC
   - Validate links
   - Update search index

## Asset Handling

Options for asset management:

- `--preserve-assets`: Keep original asset locations
- `--move-assets`: Move assets with content
- `--copy-assets`: Create copies in new location

## Reference Updates

The tool automatically updates:

- Internal links
- Asset references
- Topic references
- Navigation paths

## Validation

The tool validates:

- Target location availability
- Slug uniqueness
- Asset references
- Link integrity
- Topic existence
