use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use walkdir::WalkDir;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub mean: f64,
    pub std_dev: f64,
    pub iterations: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub regressions: Vec<Regression>,
    pub improvements: Vec<Improvement>,
    pub unchanged: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Regression {
    pub name: String,
    pub baseline: f64,
    pub current: f64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Improvement {
    pub name: String,
    pub baseline: f64,
    pub current: f64,
    pub percentage: f64,
}

pub fn collect_results(dir: &PathBuf) -> Result<Vec<BenchmarkResult>> {
    let mut results = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".json"))
    {
        let contents = fs::read_to_string(entry.path())
            .with_context(|| format!("Failed to read {}", entry.path().display()))?;

        let result: BenchmarkResult = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse {}", entry.path().display()))?;

        results.push(result);
    }

    Ok(results)
}

pub fn generate_report(
    current: &[BenchmarkResult],
    baseline: Option<&[BenchmarkResult]>,
    threshold: f64,
) -> Report {
    let mut report = Report {
        regressions: Vec::new(),
        improvements: Vec::new(),
        unchanged: Vec::new(),
    };

    if let Some(baseline) = baseline {
        for current_result in current {
            if let Some(baseline_result) = baseline.iter().find(|b| b.name == current_result.name) {
                let percentage = ((current_result.mean - baseline_result.mean) / baseline_result.mean) * 100.0;

                if percentage.abs() < threshold {
                    report.unchanged.push(current_result.name.clone());
                } else if percentage > 0.0 {
                    report.regressions.push(Regression {
                        name: current_result.name.clone(),
                        baseline: baseline_result.mean,
                        current: current_result.mean,
                        percentage,
                    });
                } else {
                    report.improvements.push(Improvement {
                        name: current_result.name.clone(),
                        baseline: baseline_result.mean,
                        current: current_result.mean,
                        percentage: percentage.abs(),
                    });
                }
            }
        }
    }

    report
}

pub fn output_json_report(report: &Report, path: &PathBuf) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn output_markdown_report(report: &Report, path: &PathBuf) -> Result<()> {
    let mut markdown = String::new();

    // Add header
    markdown.push_str("# Benchmark Analysis Report\n\n");

    // Add regressions section
    if !report.regressions.is_empty() {
        markdown.push_str("## Performance Regressions\n\n");
        markdown.push_str("| Benchmark | Baseline | Current | Change |\n");
        markdown.push_str("|-----------|----------|---------|--------|\n");

        for regression in &report.regressions {
            markdown.push_str(&format!(
                "| {} | {:.2} | {:.2} | {:.1}% |\n",
                regression.name,
                regression.baseline,
                regression.current,
                regression.percentage
            ));
        }
        markdown.push('\n');
    }

    // Add improvements section
    if !report.improvements.is_empty() {
        markdown.push_str("## Performance Improvements\n\n");
        markdown.push_str("| Benchmark | Baseline | Current | Change |\n");
        markdown.push_str("|-----------|----------|---------|--------|\n");

        for improvement in &report.improvements {
            markdown.push_str(&format!(
                "| {} | {:.2} | {:.2} | -{:.1}% |\n",
                improvement.name,
                improvement.baseline,
                improvement.current,
                improvement.percentage
            ));
        }
        markdown.push('\n');
    }

    // Add unchanged section
    if !report.unchanged.is_empty() {
        markdown.push_str("## Unchanged Benchmarks\n\n");
        for name in &report.unchanged {
            markdown.push_str(&format!("- {}\n", name));
        }
        markdown.push('\n');
    }

    fs::write(path, markdown)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_benchmark(name: &str, mean: f64) -> BenchmarkResult {
        BenchmarkResult {
            name: name.to_string(),
            mean,
            std_dev: 0.1,
            iterations: 100,
        }
    }

    fn write_benchmark_json(dir: &PathBuf, result: &BenchmarkResult) -> Result<()> {
        let json = serde_json::to_string_pretty(result)?;
        let mut file = File::create(dir.join(format!("{}.json", result.name)))?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_collect_results() -> Result<()> {
        let dir = tempdir()?;
        let result = create_test_benchmark("test_bench", 100.0);
        write_benchmark_json(&dir.path().to_path_buf(), &result)?;

        let results = collect_results(&dir.path().to_path_buf())?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test_bench");
        assert_eq!(results[0].mean, 100.0);

        Ok(())
    }

    #[test]
    fn test_collect_results_empty_dir() -> Result<()> {
        let dir = tempdir()?;
        let results = collect_results(&dir.path().to_path_buf())?;
        assert!(results.is_empty());
        Ok(())
    }

    #[test]
    fn test_collect_results_invalid_json() -> Result<()> {
        let dir = tempdir()?;
        let mut file = File::create(dir.path().join("invalid.json"))?;
        file.write_all(b"{invalid json}")?;

        let result = collect_results(&dir.path().to_path_buf());
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_generate_report() -> Result<()> {
        let current = vec![
            create_test_benchmark("fast", 90.0),
            create_test_benchmark("slow", 110.0),
            create_test_benchmark("same", 100.0),
        ];

        let baseline = vec![
            create_test_benchmark("fast", 100.0),
            create_test_benchmark("slow", 100.0),
            create_test_benchmark("same", 100.0),
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
            create_test_benchmark("test1", 100.0),
            create_test_benchmark("test2", 200.0),
        ];

        let report = generate_report(&current, None, 5.0);

        assert!(report.improvements.is_empty());
        assert!(report.regressions.is_empty());
        assert!(report.unchanged.is_empty());

        Ok(())
    }

    #[test]
    fn test_generate_report_different_names() -> Result<()> {
        let current = vec![create_test_benchmark("test1", 100.0)];
        let baseline = vec![create_test_benchmark("test2", 100.0)];

        let report = generate_report(&current, Some(&baseline), 5.0);

        assert!(report.improvements.is_empty());
        assert!(report.regressions.is_empty());
        assert!(report.unchanged.is_empty());

        Ok(())
    }

    #[test]
    fn test_output_formats() -> Result<()> {
        let report = Report {
            regressions: vec![Regression {
                name: "test".to_string(),
                baseline: 100.0,
                current: 110.0,
                percentage: 10.0,
            }],
            improvements: vec![],
            unchanged: vec![],
        };

        let dir = tempdir()?;
        let json_path = dir.path().join("report.json");
        let md_path = dir.path().join("report.md");

        output_json_report(&report, &json_path)?;
        output_markdown_report(&report, &md_path)?;

        // Verify JSON output
        let json_content = fs::read_to_string(&json_path)?;
        let parsed_json: serde_json::Value = serde_json::from_str(&json_content)?;

        assert_eq!(parsed_json["regressions"][0]["name"], "test");
        assert_eq!(parsed_json["regressions"][0]["baseline"], 100.0);
        assert_eq!(parsed_json["regressions"][0]["current"], 110.0);
        assert_eq!(parsed_json["regressions"][0]["percentage"], 10.0);

        // Verify Markdown output
        let md_content = fs::read_to_string(&md_path)?;
        assert!(md_content.contains("# Benchmark Analysis Report"));
        assert!(md_content.contains("## Performance Regressions"));
        assert!(md_content.contains("| test | 100.00 | 110.00 | 10.0% |"));

        Ok(())
    }

    #[test]
    fn test_output_formats_empty_report() -> Result<()> {
        let report = Report {
            regressions: vec![],
            improvements: vec![],
            unchanged: vec![],
        };

        let dir = tempdir()?;
        let json_path = dir.path().join("report.json");
        let md_path = dir.path().join("report.md");

        output_json_report(&report, &json_path)?;
        output_markdown_report(&report, &md_path)?;

        // Verify JSON output
        let json_content = fs::read_to_string(&json_path)?;
        let parsed_json: serde_json::Value = serde_json::from_str(&json_content)?;

        assert!(parsed_json["regressions"].as_array().unwrap().is_empty());
        assert!(parsed_json["improvements"].as_array().unwrap().is_empty());
        assert!(parsed_json["unchanged"].as_array().unwrap().is_empty());

        // Verify Markdown output
        let md_content = fs::read_to_string(&md_path)?;
        assert!(md_content.contains("# Benchmark Analysis Report"));
        assert!(!md_content.contains("## Performance Regressions"));
        assert!(!md_content.contains("## Performance Improvements"));
        assert!(!md_content.contains("## Unchanged Benchmarks"));

        Ok(())
    }
}