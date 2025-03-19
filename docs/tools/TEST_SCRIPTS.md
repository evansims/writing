# Test Scripts

Scripts for advanced testing functionality.

## coverage.sh

Script for generating code coverage reports.

**Usage**:

- `./coverage.sh`: Generate summary report
- `./coverage.sh html`: Generate HTML report
- `./coverage.sh open`: Generate and open HTML report
- `./coverage.sh lcov`: Generate LCOV format report

## mutation.sh

Script for mutation testing to validate test effectiveness.

**Usage**:

- `./mutation.sh`: Run mutation tests on critical components
- `./mutation.sh --package package_name`: Test specific package

## optimize-tests.sh

Script for optimizing test execution performance.

**Usage**:

- `./optimize-tests.sh`: Analyze and optimize test performance
- `./optimize-tests.sh --report`: Generate performance report
- `./optimize-tests.sh --cache`: Setup test caching
