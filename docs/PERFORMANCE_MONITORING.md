# Performance Monitoring System

This document describes the performance monitoring system implemented for the image processing components of the Writing project. The system provides comprehensive benchmarking, visualization, and regression detection for the `image-optimize` and `image-build` crates.

## Overview

The performance monitoring system consists of:

1. **Benchmark Suites**: Comprehensive benchmarks that test various aspects of image processing performance
2. **CI/CD Integration**: Automated performance testing through GitHub Actions
3. **Performance Dashboard**: Interactive visualization of performance metrics
4. **Regression Detection**: Automatic detection and reporting of performance regressions

## Benchmark Suites

### Standard Benchmarks

Each crate includes standard benchmarks that measure core functionality:

- **image-optimize**: Basic image optimization operations
- **image-build**: Image building and processing for content

### Advanced Benchmarks

Advanced benchmarks provide deeper insights into performance characteristics:

#### Image Patterns

Tests performance with different image content types:
- Gradient images
- Checkerboard patterns
- Noise patterns
- Photo-like images

This helps identify how the algorithm performs with different image characteristics.

#### Resize Algorithms

Tests different resize algorithms:
- Nearest neighbor
- Triangle (bilinear)
- CatmullRom
- Gaussian
- Lanczos3

#### Quality vs. Image Type

Measures how quality settings affect performance across different image types.

#### Memory Usage

Benchmarks memory consumption with different image resolutions:
- 1280x720 (HD)
- 1920x1080 (Full HD)
- 2560x1440 (QHD)
- 3840x2160 (4K)

#### Parallel Processing

Tests performance with batch processing of multiple images.

#### Format-Specific Optimizations

Tests performance of format-specific operations:
- WebP encoding with different quality settings
- AVIF encoding with different quality settings

## CI/CD Integration

Performance testing is integrated into the CI/CD pipeline through GitHub Actions:

### Workflow: `performance-monitoring.yml`

This workflow:
1. Runs on pushes to `main` that affect image processing code
2. Runs on pull requests that affect image processing code
3. Runs daily at midnight

The workflow performs:
- Benchmark runs with different feature combinations
- Performance regression detection
- Dashboard generation and deployment

## Performance Dashboard

The performance dashboard provides visual insights into performance metrics:

### Features

- **Time Series Tracking**: Performance trends over time
- **Format Comparisons**: Performance across different image formats
- **Size Variant Impact**: Performance impact of different image sizes
- **Quality Impact**: Performance impact of different quality settings
- **Feature Combinations**: Performance of different feature combinations

### Accessing the Dashboard

The dashboard is deployed to GitHub Pages and can be accessed at:
`https://<username>.github.io/<repository>/dashboard/`

## Regression Detection

The system automatically detects performance regressions:

1. Benchmarks are compared against previous baselines
2. Regressions beyond a 10% threshold trigger alerts
3. GitHub issues are created with detailed performance reports
4. The dashboard displays regression alerts

## Running Benchmarks Locally

To run benchmarks locally:

```bash
# Run standard benchmarks for image-optimize
cd tools/image-optimize
cargo bench

# Run advanced benchmarks for image-optimize
cargo bench --bench advanced_scenarios

# Run standard benchmarks for image-build
cd ../image-build
cargo bench

# Run advanced benchmarks for image-build
cargo bench --bench advanced_scenarios
```

### Feature-Specific Benchmarks

To run benchmarks with specific features:

```bash
# Run with no optional features
cargo bench --no-default-features

# Run with WebP support only
cargo bench --no-default-features --features webp

# Run with AVIF support only
cargo bench --no-default-features --features avif

# Run with all features
cargo bench --all-features
```

## Interpreting Results

Benchmark results are stored in the `target/criterion` directory and include:

- HTML reports with interactive charts
- Raw data in JSON format
- Summary statistics

The most important metrics to monitor are:
- Mean execution time
- Standard deviation
- Throughput (operations per second)

## Adding New Benchmarks

To add new benchmarks:

1. Create a new benchmark function in the appropriate benchmark file
2. Add the function to the `criterion_group!` macro
3. Ensure feature flags are properly handled for conditional benchmarks

Example:

```rust
fn bench_new_feature(c: &mut Criterion) {
    let mut group = c.benchmark_group("new_feature");
    
    // Benchmark setup and execution
    
    group.finish();
}

criterion_group!(benches, existing_bench, bench_new_feature);
```

## Performance Optimization Tips

When optimizing image processing performance:

1. **Profile First**: Use the benchmarks to identify bottlenecks
2. **Image Content Matters**: Different image patterns may require different optimizations
3. **Quality vs. Speed**: Lower quality settings generally improve performance
4. **Resize Algorithm Selection**: Choose the appropriate algorithm for your use case
5. **Batch Processing**: Consider parallel processing for multiple images
6. **Memory Management**: Monitor memory usage, especially for large images

## Troubleshooting

### Common Issues

- **Benchmark Variability**: System load can affect benchmark results. Run benchmarks multiple times for more reliable data.
- **Feature Flag Confusion**: Ensure you're testing with the correct feature flags enabled.
- **Dashboard Data Issues**: Check that the performance data is being properly generated and processed.

### Getting Help

If you encounter issues with the performance monitoring system:

1. Check the GitHub Actions logs for errors
2. Verify that benchmarks run correctly locally
3. Open an issue with detailed information about the problem 

## Analyzing Benchmark Results

The project includes a benchmark analysis script that can help you analyze results and detect regressions:

```bash
# Basic usage
./tools/scripts/analyze_benchmarks.py

# Compare with a baseline
./tools/scripts/analyze_benchmarks.py --baseline path/to/baseline/results --current path/to/current/results

# Adjust regression threshold
./tools/scripts/analyze_benchmarks.py --threshold 5.0  # 5% threshold

# Generate JSON output
./tools/scripts/analyze_benchmarks.py --json --report benchmark_report.json
```

The script generates a detailed report that includes:
- Summary statistics
- Performance regressions (if comparing with a baseline)
- Performance improvements (if comparing with a baseline)
- Detailed results for each benchmark

### Example Report

```markdown
# Benchmark Analysis Report

Generated: 2023-06-15 14:30:45

## Summary

- Total benchmarks: 24
- Average execution time: 125.78 ns
- Fastest benchmark: image_patterns/pattern/gradient (45.23 ns)
- Slowest benchmark: memory_usage/resolution/3840x2160 (512.67 ns)

## Performance Regressions

| Benchmark | Baseline (ns) | Current (ns) | Change (%) |
|-----------|--------------|--------------|------------|
| webp_specific/webp_quality/60 | 98.45 | 112.34 | +14.11 |
| quality_vs_image_type/photo_quality/95 | 145.67 | 162.89 | +11.82 |

## Performance Improvements

| Benchmark | Baseline (ns) | Current (ns) | Change (%) |
|-----------|--------------|--------------|------------|
| resize_algorithms/algorithm/Lanczos3 | 87.56 | 75.23 | -14.08 |
| parallel_processing/batch_size/8 | 432.12 | 378.45 | -12.42 |

## Detailed Results

| Benchmark | Mean (ns) | Std Dev (ns) | Std Dev (%) |
|-----------|-----------|--------------|-------------|
| image_patterns/pattern/checkerboard | 67.45 | 2.34 | 3.47 |
| image_patterns/pattern/gradient | 45.23 | 1.12 | 2.48 |
| image_patterns/pattern/noise | 89.67 | 3.56 | 3.97 |
| image_patterns/pattern/photo | 102.34 | 4.23 | 4.13 |
| ... | ... | ... | ... |
``` 

## Conclusion

The performance monitoring system provides a comprehensive solution for tracking, analyzing, and visualizing the performance of image processing functionality in the Writing project. By implementing this system, we've achieved:

1. **Data-Driven Optimization**: Performance metrics guide optimization efforts, focusing on areas with the greatest impact.

2. **Regression Prevention**: Automated detection of performance regressions helps maintain performance standards over time.

3. **Feature Impact Analysis**: Understanding how different features and configurations affect performance helps make informed decisions about feature enablement.

4. **Quality Assurance**: Comprehensive benchmarks ensure that image processing meets performance requirements across various scenarios.

5. **Transparency**: The performance dashboard provides clear visibility into performance metrics for all stakeholders.

By continuing to use and enhance this system, the project can maintain high performance standards while adding new features and capabilities. 