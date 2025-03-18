# Test Standardization Project

This project standardizes the test organization across all Rust tools in the writing project. The goal is to ensure that all tests follow a consistent structure, making them easier to maintain and extend.

## What's Been Done

1. **Created a Standardized Test Structure**

   - Unit tests in `tests/unit/`
   - Integration tests in `tests/integration/`
   - Property tests in `tests/property/`

2. **Implemented Initial Reorganization**

   - Moved tests from `content-new/src/lib.rs` to `content-new/tests/unit/get_available_topics_tests.rs`
   - Moved tests from `content-edit/src/tests/mod.rs` to `content-edit/tests/unit/content_edit_tests.rs`
   - Moved `content-edit/tests/integration_test.rs` to `content-edit/tests/integration/content_edit_integration_tests.rs`
   - Removed `content-edit/src/tests` directory

3. **Created Automation Tools**

   - Added `standardize_tests.sh` script to automate test reorganization
   - Added `standardize_tests_test.sh` script to verify the automation works correctly

4. **Added Documentation**
   - Created `TEST_ORGANIZATION.md` to document the test structure standards
   - Updated `CHANGELOG.md` to reflect the changes

## What Needs to Be Done

1. **Run the Standardization Script**

   - Execute `./tools/standardize_tests.sh` to reorganize all tests across the codebase
   - Verify that all tests still pass after reorganization with `cargo test`

2. **Update Cargo.toml Files if Needed**

   - Some crates may need to update their `[[test]]` sections in `Cargo.toml`

3. **Review and Refine Test Coverage**

   - Identify areas with insufficient test coverage
   - Add tests to cover critical functionality

4. **Ensure CI/CD Integration**
   - Make sure CI/CD pipelines recognize the new test structure
   - Adjust test runners if necessary

## How to Use the Standardization Script

```bash
# Ensure the script is executable
chmod +x tools/standardize_tests.sh

# Run the script
./tools/standardize_tests.sh

# Test that everything still works
cargo test
```

## Testing the Script

If you want to test the standardization script without affecting the real codebase:

```bash
# Ensure the test script is executable
chmod +x tools/standardize_tests_test.sh

# Run the test script
./tools/standardize_tests_test.sh
```

This will create a temporary test environment and verify that the standardization script works correctly.
