# Test Standardization

Project to standardize test organization across all Rust tools.

## Standardized Structure

- Unit tests in `tests/unit/`
- Integration tests in `tests/integration/`
- Property tests in `tests/property/`

## Implementation

1. **Reorganized Test Files**

   - Moved embedded tests to dedicated test files
   - Updated test file naming for consistency
   - Restructured test directory hierarchies

2. **Automation Tools**
   - `standardize_tests.sh`: Automates test reorganization
   - `standardize_tests_test.sh`: Verifies automation functionality

## Process

1. Run the standardization script to reorganize tests
2. Verify tests still pass after reorganization
3. Update Cargo.toml files if needed
4. Review test coverage for gaps
