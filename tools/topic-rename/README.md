# Topic Rename Tool

A tool for renaming topics while maintaining content organization and references.

## Features

- Topic renaming
- Content path updates
- Reference updates
- Asset path updates
- Dry run mode
- Backup creation
- Recovery support

## Usage

```bash
# Rename topic
topic-rename strategy strategic-planning

# Dry run (preview changes)
topic-rename strategy strategic-planning --dry-run

# Rename with backup
topic-rename strategy strategic-planning --backup

# Force rename (skip checks)
topic-rename strategy strategic-planning --force

# Update references only
topic-rename strategy strategic-planning --references-only
```

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

## Safety Features

- Automatic backup creation
- Dry run capability
- Reference validation
- Link checking
- Recovery support

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
