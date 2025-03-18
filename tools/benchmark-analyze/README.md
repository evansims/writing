# Benchmark Analysis Tool

Analyzes Criterion benchmark results and detects performance regressions.

## Usage

```bash
# Standalone
./target/release/benchmark-analyze --current ./target/criterion
./target/release/benchmark-analyze --baseline ./baseline --current ./target/criterion --json

# Via write CLI
./write build benchmark --current ./target/criterion
./write build benchmark --baseline ./baseline --current ./target/criterion --json
```

## Options

- `--baseline DIR`: Baseline results directory
- `--current DIR`: Current results directory
- `--threshold N`: Regression threshold % (default: 10)
- `--report FILE`: Output file (default: benchmark_report.md)
- `--json`: JSON output format
- `--verbose`: Verbose output

## Features

- Benchmark result analysis
- Performance regression detection
- Baseline comparison
- Trend analysis
- Multiple output formats
- Configurable thresholds

## Testing

The tool includes unit tests for:

- Result collection and parsing
- Report generation
- Output formatting
- Threshold calculations

Run tests with:

```bash
cargo test
```
