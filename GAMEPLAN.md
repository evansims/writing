# GAMEPLAN: Optimizing Rust Testing Approach

## 🎯 OBJECTIVE

Implement a comprehensive testing strategy across all Rust tools that follows DRY and TDD principles, ensures 80%+ code coverage, and works equally well locally and in CI.

## 🏆 SUCCESS CRITERIA

- [ ] All Rust tools have isolated unit tests at 80%+ code coverage
- [ ] Each tool is fully testable independently, without dependencies on other tools
- [ ] Integration tests concentrated primarily in the Write/CLI tool
- [ ] Property-based testing for complex data transformations
- [ ] Consistent mocking approach across the codebase
- [ ] Local and CI test environments are equivalent
- [ ] Test suite runs efficiently with parallelization
- [ ] No tests rely on user input

## 📋 IMPLEMENTATION PLAN

### PRIORITY 1: FOUNDATION & INFRASTRUCTURE

#### 1A: Testing Framework Setup

- [x] Standardize on Cargo Nextest for test execution
  - [x] Add Nextest configuration to Cargo.toml
  - [x] Update CI pipelines to use Nextest
  - [x] Document Nextest usage in README.md
- [x] Implement code coverage tooling
  - [x] Standardize on cargo-llvm-cov for consistent coverage reports
  - [x] Create coverage profile in Cargo.toml
  - [x] Add coverage commands to CI workflow
  - [x] Create local coverage report generation script

#### 1B: Test Utility Enhancement

- [x] Enhance common test_utils library
  - [x] Review and update existing fixtures
  - [x] Expand mock implementations
  - [x] Add comprehensive test environment setup helpers
  - [x] Create standard assertion helpers for common patterns
- [x] Implement QuickCheck/Proptest strategies
  - [x] Expand the existing proptest strategies
  - [x] Create composable generators for complex types
  - [x] Add example tests using property-based testing

### PRIORITY 2: TESTING ARCHITECTURE

#### 2A: Tool Isolation Strategy

- [x] Design for test isolation
  - [x] Refactor tools to be independently testable
  - [x] Extract shared code into common libraries
  - [x] Define clear boundaries between tools
  - [x] Ensure all tools can be tested in isolation

#### 2B: Mocking Strategy

- [x] Implement consistent mocking approach
  - [x] Introduce Mockall for trait mocking
  - [x] Create mock implementations for all external dependencies
  - [x] Document mocking patterns and best practices
  - [x] Create examples of proper dependency injection for testability

#### 2C: Test Organization

- [x] Standardize test file organization
  - [x] Organize tests into unit, integration, and property-based categories
  - [x] Implement test tagging for selective test runs
  - [x] Create test helper macros for common test patterns
  - [x] Document testing structure in ARCHITECTURE.md

### PRIORITY 3: IMPLEMENTATION FOR EXISTING TOOLS

#### 3A: Core Libraries

- [x] Enhance testing for common libraries
  - [x] common/models - Add property-based testing
  - [x] common/errors - Add comprehensive unit tests
  - [x] common/config - Expand mocking and test scenarios
  - [x] common/fs - Add test coverage for edge cases
    - Added property-based tests for file operations
    - Added edge case tests for file system operations
    - Implemented test strategies for generating valid file paths and content
  - [x] common/markdown - Add property-based tests for parsing
    - Added property-based tests for frontmatter extraction
    - Added property-based tests for markdown content processing
    - Implemented test strategies for generating valid markdown documents

#### 3B: Individual Tool Unit Testing

- [x] Ensure each tool has comprehensive isolated unit tests
  - [x] content-new - Comprehensive unit, property, and integration tests
    - Added unit tests for create_content covering all edge cases
    - Added unit tests for list_templates with pattern for proper mocking
    - Added unit tests for get_available_topics
    - Added property-based tests for content creation
    - Added integration tests for CLI interface
  - [x] content-edit - Comprehensive unit, property, and integration tests
    - Added ContentEditorImpl that implements ContentEditor trait
    - Added unit tests for edit_content with proper mocking
    - Added unit tests for update_frontmatter_field
    - Added unit tests for get_frontmatter_fields
    - Added property-based tests for content editing
  - [x] content-move - Comprehensive unit, property, and integration tests
    - Added unit tests for move_options validation
    - Added unit tests for find_content_dir functionality
    - Added unit tests for move_content functionality
    - Added property-based tests for content references updating
    - Added integration tests for CLI interface
  - [x] content-delete - Comprehensive unit, property, and integration tests
    - Added unit tests for DeleteOptions validation
    - Added unit tests for find_content_dir functionality
    - Added unit tests for delete_content functionality
    - Added property-based tests for delete operations
    - Added unit tests for DeleteCommand functionality
    - Added integration tests for CLI interface
  - [x] content-search - Comprehensive unit, property, and integration tests
    - Added unit tests for SearchOptions validation
    - Added unit tests for core search functionality
    - Added property-based tests for search queries
    - Added integration tests for CLI interface
  - [x] content-stats - Comprehensive unit, property, and integration tests
    - Added unit tests for StatsOptions validation
    - Added unit tests for calculate_stats functionality
    - Added unit tests for generate_stats with proper mocking
    - Added unit tests for date formatting
    - Added property-based tests for content statistics
    - Added integration tests for CLI interface
  - [x] content-validate - Comprehensive unit, property, and integration tests
    - Added unit tests for ValidationOptions validation
    - Added unit tests for extract_links functionality
    - Added unit tests for validate_links functionality
    - Added unit tests for validate_markdown functionality
    - Added property-based tests for validation scenarios
    - Added integration tests for CLI interface
  - [x] content-build - Comprehensive unit, property, and integration tests
    - Added unit tests for process_content functionality
    - Added unit tests for find_content_files and find_content_by_slug
    - Added unit tests for build_content
    - Added unit tests for generate_sitemap and generate_rss_feed
    - Added property-based tests for processing content
    - Added integration tests for building different content types
  - [x] Refactor as needed to make each tool independently testable
  - [x] Mock all dependencies on other tools
  - [x] Test all edge cases and error paths
  - [x] Achieve 80%+ code coverage for each tool

#### 3C: Content Tools

- [x] Enhance testing for content manipulation tools
  - [x] content-new - Add property-based tests for edge cases
    - Added comprehensive property-based testing for content creation
    - Added integration tests for CLI functionality
    - Added test coverage for error conditions and edge cases
  - [x] content-edit - Add tests for file operations
    - Added unit tests for all file operations
    - Added property-based tests for frontmatter updates
    - Added integration tests for the content editing workflow
  - [x] content-move - Add comprehensive validation tests
    - Added unit tests for MoveOptions validation
    - Added unit tests for find_content_dir functionality
    - Added unit tests for move_content functionality
    - Added property-based tests for content references updating
    - Added integration tests for CLI functionality
  - [x] content-delete - Add safety verification tests
    - Added unit tests for DeleteOptions validation
    - Added unit tests for find_content_dir functionality
    - Added unit tests for delete_content functionality
    - Added property-based tests for delete operations
    - Added unit tests for DeleteCommand functionality
  - [x] content-search - Add performance tests
    - Added unit tests for search options validation
    - Added unit tests for search functionality
    - Added property-based tests for query validation
    - Added integration tests for CLI interface
  - [x] content-stats - Add statistical validation tests
    - Added unit tests for stats calculation
    - Added property-based tests for content statistics
    - Added integration tests for CLI interface
  - [x] content-validate - Add compliance tests
    - Added unit tests for validation options
    - Added unit tests for link extraction and validation
    - Added unit tests for markdown syntax validation
    - Added integration tests for CLI interface
  - [x] content-build - Add output verification tests
    - Added property-based tests for JSON output verification
    - Added property-based tests for sitemap generation verification
    - Added property-based tests for RSS feed generation verification
    - Added comprehensive content format validation tests

#### 3D: Topic and Image Tools

- [ ] Enhance testing for topic tools
  - [ ] topic-add - Add validation tests
  - [ ] topic-edit - Add consistency tests
  - [ ] topic-rename - Add reference integrity tests
  - [ ] topic-delete - Add safety tests
- [ ] Enhance testing for image tools
  - [ ] image-optimize - Add output quality verification
  - [ ] image-build - Add size and format validation tests

#### 3E: Write/CLI Integration Testing

- [ ] Enhance integration testing for the Write/CLI tool
  - [ ] Test tool coordination and integration points
  - [ ] Cover all common user workflows
  - [ ] Test error handling across tool boundaries
  - [ ] Ensure configuration is correctly passed between tools
  - [ ] Validate end-to-end output for complex operations

### PRIORITY 4: CONTINUOUS IMPROVEMENT

#### 4A: Test Coverage Monitoring

- [ ] Implement continuous coverage monitoring
  - [ ] Set up coverage gates in CI
  - [ ] Create coverage dashboards
  - [ ] Implement coverage regression detection
  - [ ] Document coverage expectations

#### 4B: Test Quality Improvement

- [ ] Add mutation testing
  - [ ] Implement cargo-mutants for mutation testing
  - [ ] Configure mutation testing profiles
  - [ ] Add mutation testing to CI for critical components
  - [ ] Document mutation testing approach

#### 4C: Test Efficiency

- [ ] Optimize test execution
  - [ ] Profile test execution time
  - [ ] Implement test caching where appropriate
  - [ ] Organize tests by execution time
  - [ ] Create fast and comprehensive test suites

## 📊 METRICS AND MONITORING

### Coverage Goals

- [ ] Common libraries: 90%+ coverage
- [ ] Individual tools: 80%+ coverage for each tool in isolation
- [ ] Command line interfaces: 85%+ coverage
- [ ] Business logic: 85%+ coverage
- [ ] Integration points: 80%+ coverage
- [ ] Overall project: 80%+ coverage

### Performance Targets

- [ ] Unit test suite: < 10s execution time
- [ ] Integration test suite: < 60s execution time
- [ ] Full test suite with coverage: < 2m execution time

## 🛠️ TECHNICAL APPROACH

### Tool Isolation Principles

- Each tool should be testable independently
- Tools should have clear interfaces for mocking
- Avoid direct dependencies between tools
- Use common libraries for shared functionality
- Never test multiple tools together except in integration tests

### TDD Implementation

1. For new features:

   - Write test specifications first
   - Implement minimal code to pass tests
   - Refactor for maintainability
   - Verify coverage meets targets

2. For existing code:
   - Add test coverage for critical paths
   - Refactor for testability where needed
   - Use mock interfaces to isolate components
   - Add property-based tests for complex algorithms

### Dependency Injection

- Prefer trait-based interfaces for all components
- Use constructor injection for dependencies
- Create test-specific implementations of interfaces
- Use Mockall for generating mock implementations

### Test Data Management

- Use factories and builders for test data
- Implement property-based test generators
- Isolate test environments with fixtures
- Clean up all test resources after use

## EXAMPLES

### Unit Test Example (to standardize on)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use common_test_utils::TestFixture;
    use common_test_utils::mocks::MockDependencyTool;

    #[test]
    fn test_function_with_valid_input() {
        // Arrange
        let fixture = TestFixture::new().unwrap();
        let input = fixture.create_valid_input();
        let mock_dependency = MockDependencyTool::new();
        mock_dependency.expect_some_method().returning(|_| Ok(()));

        // Create SUT with mocked dependency
        let system_under_test = SystemUnderTest::new(mock_dependency);

        // Act
        let result = system_under_test.function_under_test(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_output);
    }

    #[test]
    fn test_function_with_invalid_input() {
        // Arrange
        let fixture = TestFixture::new().unwrap();
        let input = fixture.create_invalid_input();
        let mock_dependency = MockDependencyTool::new();

        // Create SUT with mocked dependency
        let system_under_test = SystemUnderTest::new(mock_dependency);

        // Act
        let result = system_under_test.function_under_test(input);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error);
    }
}
```

### Property-Based Test Example (to standardize on)

```rust
#[cfg(test)]
mod prop_tests {
    use super::*;
    use common_test_utils::proptest::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_function_properties(input in valid_input_strategy()) {
            // Setup mocked dependencies
            let mock_dependency = MockDependencyTool::new();
            mock_dependency.expect_some_method().returning(|_| Ok(()));

            // Create SUT with mocked dependency
            let system_under_test = SystemUnderTest::new(mock_dependency);

            // Property: function should not panic with valid input
            let result = system_under_test.function_under_test(input.clone());

            // Property: function should return result with same properties as input
            prop_assert!(result.property == input.expected_property);
        }
    }
}
```

### Integration Test Example (for Write/CLI tool)

```rust
#[cfg(test)]
mod integration_tests {
    use common_test_utils::integration::TestCommand;

    #[test]
    fn test_write_cli_workflow_success() {
        // Arrange - use the actual Write CLI tool, not mocks
        let command = TestCommand::new("write").unwrap();
        let test_input = "test input";

        // Act & Assert - test entire workflow across multiple tools
        let setup_output = command.assert_success(&["content", "new", "--title", "Test Article", "--topic", "blog"]);
        let main_output = command.assert_success(&["content", "edit", "--slug", "test-article"]);
        let result_output = command.assert_output_contains(&["content", "validate", "--slug", "test-article"], "Validation passed");

        // Final validation
        assert!(result_output.status.success());
    }
}
```
