use std::path::PathBuf;
use anyhow::Result;
use benchmark_analyze::{collect_results, generate_report, output_json_report, output_markdown_report};

pub fn analyze_benchmarks(
    baseline: Option<PathBuf>,
    current: PathBuf,
    threshold: f64,
    report: PathBuf,
    json: bool,
    verbose: bool,
) -> Result<()> {
    // Collect benchmark results
    let current_results = collect_results(&current)?;
    let baseline_results = if let Some(baseline) = &baseline {
        Some(collect_results(baseline)?)
    } else {
        None
    };

    // Generate report
    let report_data = generate_report(
        &current_results,
        baseline_results.as_ref().map(|v| &**v),
        threshold,
    );

    // Output report
    if json {
        output_json_report(&report_data, &report)?;
    } else {
        output_markdown_report(&report_data, &report)?;
    }

    Ok(())
}