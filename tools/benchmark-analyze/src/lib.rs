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

