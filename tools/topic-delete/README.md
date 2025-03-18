# Topic Delete Tool

Tool for safely removing topics from the content management system.

## Usage

```bash
# Interactive mode
./topic-delete

# Direct deletion
./topic-delete --id "strategy"
```

## Features

- Interactive deletion
- Confirmation prompts
- Validation checks
- Dependency checking
- Backup creation

## Validation

- Topic existence check
- Content dependency check
- Child topic check
- Backup verification

## Safety Features

- Automatic backup creation
- Dependency validation
- Confirmation prompts
- Dry run capability
- Recovery support

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
