#!/usr/bin/env bash
set -e

REPORT_TYPE=${1:-"summary"}

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "cargo-llvm-cov is not installed. Installing..."
    cargo install cargo-llvm-cov
fi

# Check if cargo-nextest is installed
if ! command -v cargo-nextest &> /dev/null; then
    echo "cargo-nextest is not installed. Installing..."
    cargo install cargo-nextest
fi

# Function to print help message
print_help() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Generate test coverage reports"
    echo ""
    echo "Options:"
    echo "  summary    Generate a coverage summary (default)"
    echo "  html       Generate an HTML coverage report"
    echo "  lcov       Generate an LCOV coverage report"
    echo "  open       Generate HTML report and open it in the browser"
    echo "  help       Display this help message"
    echo ""
    echo "Examples:"
    echo "  $0                     # Generate summary report"
    echo "  $0 html                # Generate HTML report"
    echo "  $0 open                # Generate HTML report and open it"
    echo "  $0 lcov                # Generate LCOV report"
}

# Handle help option
if [ "$REPORT_TYPE" = "help" ]; then
    print_help
    exit 0
fi

# Create output directory if it doesn't exist
mkdir -p target/coverage

echo "Generating coverage report for the entire workspace..."
echo "This might take a while, please be patient."

case "$REPORT_TYPE" in
    "summary")
        # Generate coverage summary
        cargo llvm-cov nextest --workspace --all-features --profile coverage
        ;;
    "html")
        # Generate HTML report
        cargo llvm-cov nextest --workspace --all-features --profile coverage --html
        echo "HTML report generated at: $(pwd)/target/llvm-cov/html/index.html"
        ;;
    "lcov")
        # Generate LCOV report
        cargo llvm-cov nextest --workspace --all-features --profile coverage --lcov --output-path target/coverage/lcov.info
        echo "LCOV report generated at: $(pwd)/target/coverage/lcov.info"
        ;;
    "open")
        # Generate HTML report and open it
        cargo llvm-cov nextest --workspace --all-features --profile coverage --html
        echo "HTML report generated at: $(pwd)/target/llvm-cov/html/index.html"

        # Open report in the browser (platform-specific)
        case "$(uname -s)" in
            "Darwin")
                open target/llvm-cov/html/index.html
                ;;
            "Linux")
                if command -v xdg-open &> /dev/null; then
                    xdg-open target/llvm-cov/html/index.html
                else
                    echo "Could not automatically open the report. Please open it manually."
                fi
                ;;
            "MINGW"*|"MSYS"*|"CYGWIN"*)
                start target/llvm-cov/html/index.html
                ;;
            *)
                echo "Could not automatically open the report. Please open it manually."
                ;;
        esac
        ;;
    *)
        echo "Unknown option: $REPORT_TYPE"
        print_help
        exit 1
        ;;
esac

# Print coverage statistics
echo "Coverage Statistics:"
cargo llvm-cov report --summary-only

# Check coverage threshold
COVERAGE_PCT=$(cargo llvm-cov report --summary-only | grep "TOTAL" | awk '{print $4}' | tr -d '%')
echo "----------------------------------"
echo "Overall coverage: $COVERAGE_PCT%"
if (( $(echo "$COVERAGE_PCT < 80" | bc -l) )); then
    echo "⚠️  WARNING: Coverage below 80% threshold"
    exit 1
else
    echo "✅ Coverage meets or exceeds 80% threshold"
fi