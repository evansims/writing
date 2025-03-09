## Refactoring Plan

### Phase 1: Library + Binary Structure ✅

- [x] Create common libraries
  - [x] common-config: Configuration loading and management
  - [x] common-fs: File system operations
  - [x] common-markdown: Markdown processing
  - [x] common-models: Shared data models
  - [x] common-errors: Error handling and utilities
  - [x] common-test-utils: Testing utilities

- [x] Refactor content tools
  - [x] content-new: Create new content
  - [x] content-edit: Edit existing content
  - [x] content-move: Move content between topics
  - [x] content-delete: Delete content
  - [x] content-build: Build content into output formats
  - [x] content-stats: Generate statistics about content

- [x] Refactor topic tools
  - [x] topic-add: Add new topics
  - [x] topic-edit: Edit topic metadata
  - [x] topic-rename: Rename topics and update paths
  - [x] topic-delete: Delete topics

- [x] Refactor image tools
  - [x] image-optimize: Optimize images for web
  - [x] image-build: Build image variants

- [x] Refactor generation tools
  - [x] toc-generate: Generate table of contents
  - [x] llms-generate: Generate LLMs files

### Phase 2: CLI Interface Improvements ✅

- [x] Group commands by category
  - [x] Content commands
  - [x] Topic commands
  - [x] Image commands
  - [x] Build commands

- [x] Improve interactive CLI experience
  - [x] Better help text
  - [x] Structured menus
  - [x] Progress indicators
  - [x] Colorized output

### Phase 3: Code Quality Improvements ✅

- [x] Add comprehensive error handling
  - [x] Create common-errors crate with custom error types
  - [x] Add error context and better error messages
  - [x] Implement utility traits for error handling
  - [x] Update common libraries to use the new error handling

- [x] Improve test coverage
  - [x] Add tests for common-config
  - [x] Add tests for common-fs
  - [x] Add tests for common-markdown
  - [x] Add tests for common-test-utils

- [x] Add documentation
  - [x] Add module-level documentation
  - [x] Add function-level documentation (existing functions already have good documentation)
  - [x] Add examples in module documentation

- [x] Refactor duplicated code
  - [x] Extract common patterns into utility functions
  - [x] Standardize error handling
  - [x] Improve code organization

### Phase 4: Feature Enhancements

- [ ] Add content templates
- [ ] Improve image processing
- [ ] Add content validation
- [ ] Add content search 