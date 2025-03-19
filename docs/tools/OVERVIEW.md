# Writing Tools Overview

Core tools for managing the writing project, organized in a modular architecture with shared libraries.

## Architecture

```
tools/
├── common/                 # Shared libraries
│   ├── models/             # Data structures
│   ├── config/             # Configuration
│   ├── fs/                 # Filesystem operations
│   └── markdown/           # Markdown processing
├── content-*/              # Content management
├── image-*/                # Image processing
├── topic-*/                # Topic management
└── write/                  # Main CLI tool
```

## Common Libraries

- **common-models**: Data structures used across tools
- **common-config**: Configuration loading/parsing
- **common-fs**: Filesystem operations
- **common-markdown**: Markdown processing

## Tool Structure

Each tool follows a consistent structure:

- **Binary**: Command-line interface
- **Library**: Core functionality
- **Tests**: Unit, integration, and property tests
