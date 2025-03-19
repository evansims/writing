#!/usr/bin/env bash
set -e

# Function to print help message
print_help() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Analyze and optimize test execution"
    echo ""
    echo "Options:"
    echo "  analyze    Analyze test execution time (default)"
    echo "  fast       Run the fast test suite only"
    echo "  cache      Setup test caching"
    echo "  clean      Clean test cache"
    echo "  help       Display this help message"
    echo ""
    echo "Examples:"
    echo "  $0                # Analyze test execution time"
    echo "  $0 fast           # Run fast tests only"
    echo "  $0 cache          # Setup test caching"
}

# Handle help option
if [ "$1" = "help" ]; then
    print_help
    exit 0
fi

ACTION=${1:-"analyze"}

# Create output directory if it doesn't exist
mkdir -p target/test-optimization

case "$ACTION" in
    "analyze")
        echo "Analyzing test execution time..."

        # Run tests with timing
        cargo nextest run --workspace --all-features --profile local --test-threads 1 --no-fail-fast --json | tee target/test-optimization/test-output.json

        # Extract and sort test times
        echo "Extracting test times..."
        cat target/test-optimization/test-output.json |
            grep '"event":"test_finished"' |
            jq -r '.metadata.duration_ms | tostring + " " + .metadata.name' |
            sort -n > target/test-optimization/test-times.txt

        # Create slow and fast test lists
        echo "Creating test lists by execution time..."

        # Get total number of tests
        TOTAL_TESTS=$(wc -l < target/test-optimization/test-times.txt)
        FAST_THRESHOLD=$(awk -v total=$TOTAL_TESTS 'BEGIN { print total * 0.7 }' | xargs printf "%.0f")

        head -n $FAST_THRESHOLD target/test-optimization/test-times.txt > target/test-optimization/fast-tests.txt
        tail -n +$(($FAST_THRESHOLD + 1)) target/test-optimization/test-times.txt > target/test-optimization/slow-tests.txt

        # Extract test names
        cat target/test-optimization/fast-tests.txt | awk '{$1=""; print $0}' | sed 's/^ //g' > target/test-optimization/fast-test-names.txt
        cat target/test-optimization/slow-tests.txt | awk '{$1=""; print $0}' | sed 's/^ //g' > target/test-optimization/slow-test-names.txt

        # Create fast-tests.toml
        echo "Creating fast-tests.toml configuration..."
        echo "# Fast tests configuration (auto-generated)" > .config/nextest/fast-tests.toml
        echo "[test-groups]" >> .config/nextest/fast-tests.toml
        echo 'fast = { filters = ["test(fast)"] }' >> .config/nextest/fast-tests.toml

        # Create test_organization.md with statistics
        echo "Creating test statistics report..."
        TOTAL_TIME=$(awk '{sum += $1} END {print sum}' target/test-optimization/test-times.txt)
        FAST_TIME=$(awk '{sum += $1} END {print sum}' target/test-optimization/fast-tests.txt)
        SLOW_TIME=$(awk '{sum += $1} END {print sum}' target/test-optimization/slow-tests.txt)

        echo "# Test Execution Time Analysis" > target/test-optimization/analysis.md
        echo "" >> target/test-optimization/analysis.md
        echo "Analysis date: $(date)" >> target/test-optimization/analysis.md
        echo "" >> target/test-optimization/analysis.md
        echo "## Summary" >> target/test-optimization/analysis.md
        echo "" >> target/test-optimization/analysis.md
        echo "| Category | Count | Time (ms) | Percentage |" >> target/test-optimization/analysis.md
        echo "|----------|-------|-----------|------------|" >> target/test-optimization/analysis.md
        echo "| Total    | $TOTAL_TESTS | $TOTAL_TIME | 100% |" >> target/test-optimization/analysis.md
        FAST_PCT=$(awk -v fast=$FAST_TIME -v total=$TOTAL_TIME 'BEGIN { printf "%.1f", (fast/total)*100 }')
        echo "| Fast     | $FAST_THRESHOLD | $FAST_TIME | $FAST_PCT% |" >> target/test-optimization/analysis.md
        SLOW_PCT=$(awk -v slow=$SLOW_TIME -v total=$TOTAL_TIME 'BEGIN { printf "%.1f", (slow/total)*100 }')
        SLOW_COUNT=$(($TOTAL_TESTS - $FAST_THRESHOLD))
        echo "| Slow     | $SLOW_COUNT | $SLOW_TIME | $SLOW_PCT% |" >> target/test-optimization/analysis.md
        echo "" >> target/test-optimization/analysis.md

        echo "## Slowest Tests" >> target/test-optimization/analysis.md
        echo "" >> target/test-optimization/analysis.md
        echo "| Test | Time (ms) |" >> target/test-optimization/analysis.md
        echo "|------|-----------|" >> target/test-optimization/analysis.md
        tail -n 10 target/test-optimization/test-times.txt |
            awk '{print "| " $2 " | " $1 " |"}' >> target/test-optimization/analysis.md

        # Print report
        echo "Test execution time analysis complete!"
        echo "Report saved to: target/test-optimization/analysis.md"
        echo ""
        echo "Fast test list: target/test-optimization/fast-test-names.txt"
        echo "Slow test list: target/test-optimization/slow-test-names.txt"
        echo ""
        echo "Total tests: $TOTAL_TESTS"
        echo "Fast tests: $FAST_THRESHOLD ($FAST_PCT% of total time)"
        echo "Slow tests: $SLOW_COUNT ($SLOW_PCT% of total time)"
        ;;

    "fast")
        echo "Running fast test suite..."

        # Check if fast-tests.toml exists
        if [ ! -f ".config/nextest/fast-tests.toml" ]; then
            echo "Fast test configuration not found. Run '$0 analyze' first."
            exit 1
        fi

        # Run only fast tests
        cargo nextest run --workspace --all-features --profile local --test-groups fast
        ;;

    "cache")
        echo "Setting up test caching..."

        # Install cargo-cache-action if needed
        if ! command -v cargo-cache-action &> /dev/null; then
            cargo install cargo-cache-action
        fi

        # Create cache configuration
        mkdir -p .cargo
        echo "[cache]" > .cargo/config.toml
        echo "enable = true" >> .cargo/config.toml
        echo "dir = \"target/test-cache\"" >> .cargo/config.toml

        echo "Test caching set up successfully!"
        echo "Use 'cargo test' as usual, results will be cached."
        ;;

    "clean")
        echo "Cleaning test cache..."
        rm -rf target/test-cache
        echo "Test cache cleaned successfully!"
        ;;

    *)
        echo "Unknown option: $ACTION"
        print_help
        exit 1
        ;;
esac