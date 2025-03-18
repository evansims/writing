# Topic Edit Tool

Tool for modifying existing topics in the content management system.

## Features

- Interactive editing
- Metadata updates
- Validation checks
- Backup creation
- Dry run mode

## Usage

```bash
# Interactive mode
./topic-edit

# Direct edit
./topic-edit --id "strategy" --title "Business Strategy" --description "Updated description"
```

## Editable Fields

The following fields can be modified:

- Title
- Description
- Icon
- Parent topic
- Custom metadata

## Validation

- Topic existence check
- ID format validation
- Title and description length
- Parent topic validation
- Icon validation

## Safety Features

- Automatic backup creation
- Validation checks
- Confirmation prompts
- Dry run capability
- Recovery support

## Change History

The tool maintains a history of changes:

```yaml
history:
  - timestamp: "2024-03-18T12:00:00Z"
    field: "title"
    old: "Strategy"
    new: "Strategic Planning"
  - timestamp: "2024-03-18T12:00:00Z"
    field: "description"
    old: "Old description"
    new: "Updated description"
```
