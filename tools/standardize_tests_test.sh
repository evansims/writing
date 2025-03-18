#!/bin/bash

# Test script for standardize_tests.sh
# This script verifies that the standardize_tests.sh script works correctly

# Create a test directory
TEST_DIR="/tmp/rust_test_standardization"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/tools"

# Create a test tool with inline tests
mkdir -p "$TEST_DIR/tools/test-tool/src"
cat > "$TEST_DIR/tools/test-tool/src/lib.rs" << 'EOF'
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 2), 3);
    }
}
EOF

# Create a test tool with tests in src/tests
mkdir -p "$TEST_DIR/tools/test-tool2/src/tests"
cat > "$TEST_DIR/tools/test-tool2/src/lib.rs" << 'EOF'
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}
EOF

cat > "$TEST_DIR/tools/test-tool2/src/tests/mod.rs" << 'EOF'
#[cfg(test)]
mod multiply_tests;

#[cfg(test)]
mod divide_tests;
EOF

cat > "$TEST_DIR/tools/test-tool2/src/tests/multiply_tests.rs" << 'EOF'
use crate::multiply;

#[test]
fn test_multiply() {
    assert_eq!(multiply(2, 3), 6);
}
EOF

cat > "$TEST_DIR/tools/test-tool2/src/tests/divide_tests.rs" << 'EOF'
use crate::divide;

#[test]
fn test_divide() {
    assert_eq!(divide(6, 2), 3);
}
EOF

# Create a test tool with integration tests in tests/
mkdir -p "$TEST_DIR/tools/test-tool3/tests"
cat > "$TEST_DIR/tools/test-tool3/src/lib.rs" << 'EOF'
pub fn power(a: i32, b: i32) -> i32 {
    let mut result = 1;
    for _ in 0..b {
        result *= a;
    }
    result
}
EOF

cat > "$TEST_DIR/tools/test-tool3/tests/integration_test.rs" << 'EOF'
use test_tool3::power;

#[test]
fn test_power() {
    assert_eq!(power(2, 3), 8);
}
EOF

# Copy the standardize_tests.sh script to the test directory
cp "$(pwd)/tools/standardize_tests.sh" "$TEST_DIR/"

# Run the standardize_tests.sh script
pushd "$TEST_DIR"
./standardize_tests.sh
popd

# Check the results
echo "Checking test-tool (inline tests)..."
if [ -f "$TEST_DIR/tools/test-tool/tests/unit/tests_tests.rs" ]; then
    echo "  [PASS] Unit tests extracted from lib.rs"
else
    echo "  [FAIL] Unit tests not extracted from lib.rs"
fi

if [ -d "$TEST_DIR/tools/test-tool/tests/integration" ] && [ -d "$TEST_DIR/tools/test-tool/tests/property" ]; then
    echo "  [PASS] Standard directory structure created"
else
    echo "  [FAIL] Standard directory structure not created"
fi

echo "Checking test-tool2 (src/tests)..."
if [ -f "$TEST_DIR/tools/test-tool2/tests/unit/multiply_tests.rs" ] && [ -f "$TEST_DIR/tools/test-tool2/tests/unit/divide_tests.rs" ]; then
    echo "  [PASS] Unit tests moved from src/tests"
else
    echo "  [FAIL] Unit tests not moved from src/tests"
fi

if [ ! -d "$TEST_DIR/tools/test-tool2/src/tests" ]; then
    echo "  [PASS] src/tests directory removed"
else
    echo "  [FAIL] src/tests directory not removed"
fi

echo "Checking test-tool3 (integration tests)..."
if [ -f "$TEST_DIR/tools/test-tool3/tests/integration/integration_test.rs" ]; then
    echo "  [PASS] Integration tests moved to standard location"
else
    echo "  [FAIL] Integration tests not moved to standard location"
fi

echo "Test complete!"