# Common Utilities Crate

This crate provides a collection of common utilities used across various tools in the writing project. It serves as a shared foundation for building consistent, reliable tools.

## Modules

### `common-fs`

File system operations with proper error handling and resource management.

- File operations: read, write, copy, delete
- Directory operations: create, copy, move, delete
- Path normalization and cleanup
- Safe file handling with proper resource cleanup

[See detailed documentation](fs/README.md)

### `common-errors`

Consistent error handling and reporting mechanisms.

- Custom error types for different error categories
- Context-aware error handling
- Enhanced error reporting with file information
- Result extensions for better error handling

### `common-models`

Shared data models and structures.

- Configuration structures
- Content and metadata models
- Shared types and enums
- Serialization/deserialization support

### `common-cli`

Command-line interface utilities and argument parsing.

- Standardized CLI argument handling
- Common command-line options
- Help text generation
- Version information formatting

### `common-markdown`

Markdown parsing and manipulation utilities.

- Frontmatter extraction and parsing
- Markdown rendering
- Link validation and manipulation
- Content structure analysis

### `common-templates`

Template handling and rendering utilities.

- Template loading and parsing
- Variable substitution
- Conditional rendering
- Template validation

### `common-validation`

Input validation and sanitization utilities.

- Content validation rules
- Slug validation and normalization
- String sanitization
- Schema validation

### `common-test-utils`

Utilities for testing and mocking.

- Mock file system operations
- Test data generators
- Assertion helpers
- Test configuration

## Design Philosophy

The common utilities are designed with the following principles in mind:

1. **Consistency**: Provide consistent interfaces across all tools
2. **Reliability**: Robust error handling and resource management
3. **Testability**: Easy to test and mock
4. **Performance**: Efficient operations that scale well
5. **Usability**: Intuitive interfaces with good documentation

## Usage Examples

Check each module's README for specific usage examples. 