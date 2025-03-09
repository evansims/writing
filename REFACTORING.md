# Refactoring Plan for Writing Tools

This document outlines the plan for refactoring the writing project to follow DRY principles and better code organization.

## Phase 1: Extracting Common Functionality into Libraries

### Completed
- [x] **Common Libraries**
  - [x] **common/models**: Library for shared data models
  - [x] **common/config**: Library for configuration management
  - [x] **common/fs**: Library for filesystem operations
  - [x] **common/markdown**: Library for markdown processing

- [x] **Content Management**
  - [x] **content-new**: Converted to library + binary
  - [x] **content-stats**: Converted to library + binary
  - [x] **content-edit**: Converted to library + binary
  - [x] **content-delete**: Converted to library + binary
  - [x] **content-move**: Converted to library + binary
  - [x] **content-build**: Converted to library + binary

- [x] **Image Management**
  - [x] **image-optimize**: Convert to library + binary
  - [x] **image-build**: Convert to library + binary

- [x] **Topic Management**
  - [x] **topic-add**: Convert to library + binary
  - [x] **topic-edit**: Convert to library + binary
  - [x] **topic-rename**: Convert to library + binary
  - [x] **topic-delete**: Convert to library + binary

- [x] **Other Tools**
  - [x] **toc-generate**: Convert to library + binary
  - [x] **llms-generate**: Convert to library + binary

## Phase 2: Consolidating CLI Interface

### Planned
- [x] Redesign CLI interface for better user experience
- [x] Add comprehensive help text and documentation
- [x] Implement interactive mode improvements
- [x] Add configuration options for CLI behavior

## Phase 3: Testing and Documentation

### Planned
- [ ] Add unit tests for all common libraries
- [ ] Add integration tests for tool functionality
- [ ] Improve error messages and error handling
- [ ] Update documentation to reflect new architecture

## Implementation Guidelines

### Library Structure

Each tool should follow this structure:

```
tools/tool-name/
├── Cargo.toml       # With both lib and bin targets
├── src/
│   ├── lib.rs       # Core functionality
│   └── main.rs      # CLI interface
```

### Refactoring Steps for Each Tool

1. **Update Cargo.toml**
   - Add description
   - Add library target
   - Add binary target
   - Add common library dependencies

2. **Create lib.rs**
   - Extract core functionality from main.rs
   - Use common libraries instead of custom implementations
   - Define clear public API
   - Add unit tests

3. **Update main.rs**
   - Use library functions
   - Focus on CLI interface and user interaction

4. **Update write/Cargo.toml**
   - Add new library dependency
   - Remove binary dependency

5. **Update write/src/tools.rs**
   - Use library functions directly where possible
   - Fall back to binary execution when needed

### Testing Guidelines

1. **Unit Tests**
   - Test each public function
   - Use temporary files for filesystem operations
   - Mock dependencies where needed

2. **Integration Tests**
   - Test CLI commands
   - Verify outputs and side effects

## Documentation Guidelines

1. **API Documentation**
   - Document all public functions and types
   - Include examples where helpful

2. **User Documentation**
   - Keep README updated
   - Update tool-specific documentation 

## Notes

* The refactoring aims to reduce code duplication and improve maintainability
* Each tool is being converted to a library + binary pattern for better code reuse
* The CLI tool ("writing") is being updated to use the libraries directly when possible
* Fallback to running the binaries is maintained for complex interactive functionality 