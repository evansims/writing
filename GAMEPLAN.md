# GAMEPLAN: Optimizing Rust Testing Approach

## 🎯 OBJECTIVE

Implement a comprehensive testing strategy across all Rust tools that follows DRY and TDD principles, ensures 80%+ code coverage, and works equally well locally and in CI.

## 🏆 SUCCESS CRITERIA

- [ ] All Rust tools have unit tests at 80%+ code coverage
- [ ] Integration tests for all critical user workflows
- [ ] Property-based testing for complex data transformations
- [ ] Consistent mocking approach across the codebase
- [ ] Local and CI test environments are equivalent
- [ ] Test suite runs efficiently with parallelization
- [ ] No tests rely on user input

## 📋 IMPLEMENTATION PLAN

### PRIORITY 1: FOUNDATION & INFRASTRUCTURE

#### 1A: Testing Framework Setup

- [ ] Standardize on Cargo Nextest for test execution
  - [ ] Add Nextest configuration to Cargo.toml
  - [ ] Update CI pipelines to use Nextest
  - [ ] Document Nextest usage in README.md
- [ ] Implement code coverage tooling
  - [ ] Standardize on cargo-llvm-cov for consistent coverage reports
  - [ ] Create coverage profile in Cargo.toml
  - [ ] Add coverage commands to CI workflow
  - [ ] Create local coverage report generation script

#### 1B: Test Utility Enhancement

- [ ] Enhance common test_utils library
  - [ ] Review and update existing fixtures
  - [ ] Expand mock implementations
  - [ ] Add comprehensive test environment setup helpers
  - [ ] Create standard assertion helpers for common patterns
- [ ] Implement QuickCheck/Proptest strategies
  - [ ] Expand the existing proptest strategies
  - [ ] Create composable generators for complex types
  - [ ] Add example tests using property-based testing

### PRIORITY 2: TESTING ARCHITECTURE

#### 2A: Mocking Strategy

- [ ] Implement consistent mocking approach
  - [ ] Introduce Mockall for trait mocking
  - [ ] Create mock implementations for all external dependencies
  - [ ] Document mocking patterns and best practices
  - [ ] Create examples of proper dependency injection for testability

#### 2B: Test Organization

- [ ] Standardize test file organization
  - [ ] Organize tests into unit, integration, and property-based categories
  - [ ] Implement test tagging for selective test runs
  - [ ] Create test helper macros for common test patterns
  - [ ] Document testing structure in ARCHITECTURE.md

### PRIORITY 3: IMPLEMENTATION FOR EXISTING TOOLS

#### 3A: Core Libraries

- [ ] Enhance testing for common libraries
  - [ ] common/models - Add property-based testing
  - [ ] common/errors - Add comprehensive unit tests
  - [ ] common/config - Expand mocking and test scenarios
  - [ ] common/fs - Add test coverage for edge cases
  - [ ] common/markdown - Add property-based tests for parsing

#### 3B: Content Tools

- [ ] Enhance testing for content manipulation tools
  - [ ] content-new - Add property-based tests for edge cases
  - [ ] content-edit - Add tests for file operations
  - [ ] content-move - Add comprehensive validation tests
  - [ ] content-delete - Add safety verification tests
  - [ ] content-search - Add performance tests
  - [ ] content-stats - Add statistical validation tests
  - [ ] content-validate - Add compliance tests
  - [ ] content-build - Add output verification tests

#### 3C: Topic and Image Tools

- [ ] Enhance testing for topic tools
  - [ ] topic-add - Add validation tests
  - [ ] topic-edit - Add consistency tests
  - [ ] topic-rename - Add reference integrity tests
  - [ ] topic-delete - Add safety tests
- [ ] Enhance testing for image tools
  - [ ] image-optimize - Add output quality verification
  - [ ] image-build - Add size and format validation tests

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
- [ ] Command line interfaces: 85%+ coverage
- [ ] Business logic: 85%+ coverage
- [ ] Integration points: 80%+ coverage
- [ ] Overall project: 80%+ coverage

### Performance Targets

- [ ] Unit test suite: < 10s execution time
- [ ] Integration test suite: < 60s execution time
- [ ] Full test suite with coverage: < 2m execution time

## 🛠️ TECHNICAL APPROACH

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

## 🧪 EXAMPLES

### Unit Test Example (to standardize on)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use common_test_utils::TestFixture;

    #[test]
    fn test_function_with_valid_input() {
        // Arrange
        let fixture = TestFixture::new().unwrap();
        let input = fixture.create_valid_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_output);
    }

    #[test]
    fn test_function_with_invalid_input() {
        // Arrange
        let fixture = TestFixture::new().unwrap();
        let input = fixture.create_invalid_input();

        // Act
        let result = function_under_test(input);

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
            // Property: function should not panic with valid input
            let result = function_under_test(input.clone());

            // Property: function should return result with same properties as input
            prop_assert!(result.property == input.expected_property);
        }
    }
}
```

### Integration Test Example (to standardize on)

```rust
#[cfg(test)]
mod integration_tests {
    use common_test_utils::integration::TestCommand;

    #[test]
    fn test_workflow_success() {
        // Arrange
        let command = TestCommand::new("tool-name").unwrap();
        let test_input = "test input";

        // Act & Assert - test entire workflow
        let setup_output = command.assert_success(&["setup", "--option"]);
        let main_output = command.assert_success(&["main", "--with-option"]);
        let result_output = command.assert_output_contains(&["verify"], "Expected result");

        // Final validation
        assert!(result_output.status.success());
    }
}
```
