//! Unit tests extracted from lib.rs

use benchmark_analyze::*;
use std::path::PathBuf;
use std::fs;
use anyhow::Result;
use tempfile::tempdir;

mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_basic_benchmarks() {
        let benchmark = BenchmarkResult {
            name: "bench_test".to_string(),
            mean: 123.45,
            std_dev: 1.23,
            iterations: 1000,
        };

        assert_eq!(benchmark.name, "bench_test");
        assert_eq!(benchmark.mean, 123.45);
        assert_eq!(benchmark.std_dev, 1.23);
        assert_eq!(benchmark.iterations, 1000);
    }

    fn write_benchmark_json(dir: &PathBuf, result: &BenchmarkResult) -> Result<()> {
        let json_path = dir.join(format!("{}.json", result.name));
        let json = serde_json::to_string_pretty(result)?;
        fs::write(json_path, json)?;
        Ok(())
    }

    #[test]
    fn test_collect_results() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().to_path_buf();

        // Create benchmark results
        let result1 = BenchmarkResult {
            name: "test1".to_string(),
            mean: 10.0,
            std_dev: 1.0,
            iterations: 100,
        };

        write_benchmark_json(&dir_path, &result1)?;

        let results = collect_results(&dir_path)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test1");
        assert_eq!(results[0].mean, 10.0);

        Ok(())
    }

    #[test]
    fn test_collect_results_empty_dir() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().to_path_buf();

        let results = collect_results(&dir_path)?;
        assert_eq!(results.len(), 0);

        Ok(())
    }

    #[test]
    fn test_collect_results_invalid_json() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().to_path_buf();

        // Create invalid JSON file
        let invalid_path = dir_path.join("invalid.json");
        fs::write(invalid_path, "not valid json")?;

        let result = collect_results(&dir_path);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_generate_report() -> Result<()> {
        let current = vec![
            BenchmarkResult {
                name: "fast".to_string(),
                mean: 90.0,
                std_dev: 1.0,
                iterations: 100,
            },
            BenchmarkResult {
                name: "slow".to_string(),
                mean: 110.0,
                std_dev: 1.0,
                iterations: 100,
            },
            BenchmarkResult {
                name: "same".to_string(),
                mean: 100.0,
                std_dev: 1.0,
                iterations: 100,
            },
        ];

        let baseline = vec![
            BenchmarkResult {
                name: "fast".to_string(),
                mean: 100.0,
                std_dev: 1.0,
                iterations: 100,
            },
            BenchmarkResult {
                name: "slow".to_string(),
                mean: 100.0,
                std_dev: 1.0,
                iterations: 100,
            },
            BenchmarkResult {
                name: "same".to_string(),
                mean: 100.0,
                std_dev: 1.0,
                iterations: 100,
            },
        ];

        let report = generate_report(&current, Some(&baseline), 5.0);

        assert_eq!(report.improvements.len(), 1);
        assert_eq!(report.regressions.len(), 1);
        assert_eq!(report.unchanged.len(), 1);

        assert_eq!(report.improvements[0].name, "fast");
        assert_eq!(report.regressions[0].name, "slow");
        assert_eq!(report.unchanged[0], "same");

        Ok(())
    }

    #[test]
    fn test_generate_report_no_baseline() -> Result<()> {
        let current = vec![
            BenchmarkResult {
                name: "test1".to_string(),
                mean: 100.0,
                std_dev: 1.0,
                iterations: 100,
            },
            BenchmarkResult {
                name: "test2".to_string(),
                mean: 200.0,
                std_dev: 1.0,
                iterations: 100,
            },
        ];

        let report = generate_report(&current, None, 5.0);

        assert!(report.improvements.is_empty());
        assert!(report.regressions.is_empty());
        assert!(report.unchanged.is_empty());

        Ok(())
    }

    #[test]
    fn test_generate_report_different_names() -> Result<()> {
        let current = vec![
            BenchmarkResult {
                name: "test1".to_string(),
                mean: 100.0,
                std_dev: 1.0,
                iterations: 100,
            },
        ];

        let baseline = vec![
            BenchmarkResult {
                name: "test2".to_string(),
                mean: 100.0,
                std_dev: 1.0,
                iterations: 100,
            },
        ];

        let report = generate_report(&current, Some(&baseline), 5.0);

        assert!(report.improvements.is_empty());
        assert!(report.regressions.is_empty());
        assert!(report.unchanged.is_empty());

        Ok(())
    }

    #[test]
    fn test_output_formats() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().to_path_buf();

        let report = Report {
            regressions: vec![
                Regression {
                    name: "test".to_string(),
                    baseline: 100.0,
                    current: 110.0,
                    percentage: 10.0,
                }
            ],
            improvements: vec![],
            unchanged: vec![],
        };

        let json_path = dir_path.join("report.json");
        let md_path = dir_path.join("report.md");

        output_json_report(&report, &json_path)?;
        output_markdown_report(&report, &md_path)?;

        // Verify JSON output
        let json_content = fs::read_to_string(&json_path)?;
        let parsed: serde_json::Value = serde_json::from_str(&json_content)?;
        assert_eq!(parsed["regressions"][0]["name"], "test");
        assert_eq!(parsed["regressions"][0]["baseline"], 100.0);
        assert_eq!(parsed["regressions"][0]["current"], 110.0);
        assert_eq!(parsed["regressions"][0]["percentage"], 10.0);

        // Verify Markdown output
        let md_content = fs::read_to_string(&md_path)?;
        assert!(md_content.contains("# Benchmark Analysis Report"));
        assert!(md_content.contains("## Performance Regressions"));
        assert!(md_content.contains("| test | 100.00 | 110.00 | 10.0% |"));

        Ok(())
    }

    #[test]
    fn test_output_formats_empty_report() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().to_path_buf();

        let report = Report {
            regressions: vec![],
            improvements: vec![],
            unchanged: vec![],
        };

        let json_path = dir_path.join("report.json");
        let md_path = dir_path.join("report.md");

        output_json_report(&report, &json_path)?;
        output_markdown_report(&report, &md_path)?;

        // Verify JSON output
        let json_content = fs::read_to_string(&json_path)?;
        let parsed: serde_json::Value = serde_json::from_str(&json_content)?;
        assert!(parsed["regressions"].as_array().unwrap().is_empty());
        assert!(parsed["improvements"].as_array().unwrap().is_empty());
        assert!(parsed["unchanged"].as_array().unwrap().is_empty());

        // Verify Markdown output
        let md_content = fs::read_to_string(&md_path)?;
        assert!(md_content.contains("# Benchmark Analysis Report"));
        assert!(!md_content.contains("## Performance Regressions"));
        assert!(!md_content.contains("## Performance Improvements"));
        assert!(!md_content.contains("## Unchanged Benchmarks"));

        Ok(())
    }
}
