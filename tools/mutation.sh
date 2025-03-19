#!/usr/bin/env bash
set -e

# Check if cargo-mutants is installed
if ! command -v cargo-mutants &> /dev/null; then
    echo "cargo-mutants is not installed. Installing..."
    cargo install cargo-mutants
fi

# Function to print help message
print_help() {
    echo "Usage: $0 [OPTION] [CRATE]"
    echo ""
    echo "Run mutation tests to verify test quality"
    echo ""
    echo "Options:"
    echo "  all        Run mutation tests on all crates (default)"
    echo "  [crate]    Run mutation tests on the specified crate"
    echo "  help       Display this help message"
    echo ""
    echo "Examples:"
    echo "  $0                      # Run mutation tests on all crates"
    echo "  $0 content-new          # Run mutation tests only on content-new"
    echo "  $0 common/fs            # Run mutation tests only on common/fs"
}

# Handle help option
if [ "$1" = "help" ]; then
    print_help
    exit 0
fi

# Create output directory if it doesn't exist
mkdir -p target/mutation

echo "Running mutation tests..."

# If a specific crate is provided, run mutation tests on that crate
if [ -n "$1" ] && [ "$1" != "all" ]; then
    CRATE=$1
    echo "Running mutation tests on $CRATE..."

    cd $CRATE
    cargo mutants

    echo "Mutation testing complete for $CRATE"
    exit 0
fi

# Otherwise, run mutation tests on all crates
echo "Running mutation tests on all crates..."

# Get all workspace members
MEMBERS=$(grep "members" Cargo.toml -A 30 | grep -o '"[^"]*"' | tr -d '"' | grep -v '^$')

# Create a summary file
SUMMARY_FILE="target/mutation/summary.txt"
echo "Mutation Testing Summary" > $SUMMARY_FILE
echo "======================" >> $SUMMARY_FILE
echo "Date: $(date)" >> $SUMMARY_FILE
echo "" >> $SUMMARY_FILE

# Run mutation tests on each crate
for member in $MEMBERS; do
    echo "Running mutation tests on $member..."

    # Skip if the directory doesn't exist
    if [ ! -d "$member" ]; then
        echo "Skipping $member (directory not found)"
        continue
    fi

    # Run mutation tests
    cd $member
    cargo mutants || true
    cd - > /dev/null

    # Extract summary for the report
    echo "* $member:" >> $SUMMARY_FILE
    if [ -f "$member/target/mutants/output.txt" ]; then
        grep "Score:" "$member/target/mutants/output.txt" >> $SUMMARY_FILE || echo "  No score found" >> $SUMMARY_FILE
    else
        echo "  No mutation results found" >> $SUMMARY_FILE
    fi
    echo "" >> $SUMMARY_FILE
done

echo "Mutation testing complete"
echo "Summary report: $SUMMARY_FILE"

# Print overall summary
echo ""
echo "Mutation Testing Results Summary:"
echo "================================"
cat $SUMMARY_FILE