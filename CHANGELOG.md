# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added property-based testing for `common/models` library
- Added comprehensive unit tests for error formatting in `common/errors` library
- Added mock-based testing for configuration scenarios in `common/config` library
- Added standardized test organization structure across all tools
- Added `standardize_tests.sh` script to automate test reorganization
- Added `TEST_ORGANIZATION.md` documentation for test structure standards
- Comprehensive test suite for content-delete tool
  - Added unit tests for delete_content functionality covering all edge cases
  - Added unit tests for find_content_dir with proper error handling
  - Added property-based tests for delete operations with safety verification
  - Added unit tests for DeleteCommand with validation
  - Added integration tests for CLI functionality

### Changed

- Reorganized tests in `content-new` and `content-edit` tools to follow standard structure
- Moved embedded tests from source files to separate test modules

### Removed

### Fixed
