use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use colored::*;

mod lib;
use lib::{collect_results, generate_report, output_json_report, output_markdown_report};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory containing baseline benchmark results
    #[arg(short, long)]
    baseline: Option<PathBuf>,

    /// Directory containing current benchmark results
    #[arg(short, long)]
    current: PathBuf,

    /// Regression threshold percentage
    #[arg(short, long, default_value = "10")]
    threshold: f64,

    /// Output report file
    #[arg(short, long, default_value = "benchmark_report.md")]
    report: PathBuf,

    /// Output JSON format
    #[arg(short, long)]
    json: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Collect benchmark results
    let current_results = collect_results(&args.current)?;
    let baseline_results = if let Some(baseline) = &args.baseline {
        Some(collect_results(baseline)?)
    } else {
        None
    };

    // Generate report
    let report = generate_report(&current_results, baseline_results.as_ref().map(|v| &**v), args.threshold);

    // Output report
    if args.json {
        output_json_report(&report, &args.report)?;
    } else {
        output_markdown_report(&report, &args.report)?;
    }

    Ok(())
}
