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
- Added mutation testing with cargo-mutants
  - Created `mutation.sh` script for running mutation tests
  - Added CI workflow for mutation testing critical components
  - Configured mutation testing to generate reports and detect issues
- Added test execution optimization
  - Created `optimize-tests.sh` script for analyzing test performance
  - Added test categorization by execution time (fast vs. slow)
  - Implemented test caching for faster CI/CD pipeline execution
  - Configured optimized CI test profiles
- Added test coverage gates and monitoring
  - Enhanced coverage workflow with regression detection
  - Added dashboards for coverage visualization
  - Established minimum coverage thresholds for all components
- New `content-import` tool for importing web content as articles
  - Supports importing content from any URL
  - Automatically extracts title and main content
  - Converts HTML to Markdown format
  - Allows customization of title, tags, and draft status
  - Creates properly formatted MDX files with frontmatter
- Added text-to-speech functionality using Eleven Labs API
  - Integrated Eleven Labs SDK (version 1.54.0)
  - Added audio generation for article content
  - Implemented audio caching with checksum-based validation
  - Created API endpoints for audio retrieval and generation
  - Added an audio player component with playback controls
  - Implemented text highlighting during audio playback
  - Added diagnostic endpoints to check API status and test audio generation

### Changed

- Reorganized tests in `content-new` and `content-edit` tools to follow standard structure
- Moved embedded tests from source files to separate test modules
- Improved CI workflow with optimized test execution profiles
- Enhanced test coverage reporting with detailed metrics
- Updated Eleven Labs SDK from version 0.2.26 to 1.54.0

### Removed

### Fixed
