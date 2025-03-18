# Topic Rename Tool

Tool for renaming topics in the content management system.

## Usage

```bash
# Interactive mode
./topic-rename

# Direct rename
./topic-rename --old-id "strategy" --new-id "business-strategy"
```

## Features

- Interactive renaming
- Validation checks
- Metadata updates
- Backup creation
- Dry run mode

## Validation

- Topic existence check
- New ID format validation
- Duplicate ID check
- Content dependency check
- Child topic check

## Safety Features

- Automatic backup creation
- Validation checks
- Confirmation prompts
- Dry run capability
- Recovery support

## Rename Process

1. **Pre-rename Checks**

   - Verify source topic exists
   - Check target name availability
   - Validate permissions
   - Create backup (if requested)

2. **Path Updates**

   - Update topic directory
   - Update content paths
   - Update asset paths
   - Update references

3. **Post-rename Tasks**
   - Update build cache
   - Regenerate TOC
   - Update search index
   - Validate links

## Reference Updates

The tool automatically updates:

- Content references
- Asset references
- Navigation paths
- Search indices
- Build configurations

## Recovery

Renamed topics can be restored from backups:

```bash
# List available backups
topic-rename --list-backups

# Restore from backup
topic-rename --restore "backup-2024-03-18"
```

## Backup Structure

```
.backups/
  └── [timestamp]/
      ├── topics/
      │   ├── old/
      │   │   └── [old_topic_id]/
      │   └── new/
      │       └── [new_topic_id]/
      ├── content/
      │   └── [topic_id]/
      └── metadata.json
```
