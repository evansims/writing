# Topic Delete Tool

A tool for safely removing topics and their associated content from the content management system.

## Features

- Safe topic deletion
- Content handling options
- Dependency checking
- Backup creation
- Dry run mode
- Recovery support
- Batch deletion

## Usage

```bash
# Delete topic
topic-delete strategy

# Delete with content
topic-delete strategy --with-content

# Dry run (preview changes)
topic-delete strategy --dry-run

# Delete with backup
topic-delete strategy --backup

# Force delete (skip checks)
topic-delete strategy --force

# Delete multiple topics
topic-delete "strategy,business"
```

## Deletion Options

### Content Handling

- `--with-content`: Delete topic and all content
- `--move-content`: Move content to another topic
- `--archive-content`: Archive content before deletion
- `--keep-content`: Keep content but remove topic

### Safety Features

- `--backup`: Create backup before deletion
- `--dry-run`: Preview changes without applying
- `--force`: Skip dependency checks
- `--confirm`: Require confirmation

## Deletion Process

1. **Pre-deletion Checks**

   - Verify topic exists
   - Check for dependencies
   - Validate permissions
   - Create backup (if requested)

2. **Content Processing**

   - Handle associated content
   - Update references
   - Clean up assets
   - Update search index

3. **Topic Removal**
   - Remove topic files
   - Update parent topics
   - Clean up metadata
   - Update navigation

## Recovery

Deleted topics can be recovered from backups:

```bash
# List available backups
topic-delete --list-backups

# Restore from backup
topic-delete --restore "backup-2024-03-18"
```

## Backup Structure

```
.backups/
  └── [timestamp]/
      ├── topics/
      │   └── [topic_id]/
      ├── content/
      │   └── [topic_id]/
      └── metadata.json
```
