#!/usr/bin/env python3
"""
Benchmark Analysis Tool

This script analyzes Criterion benchmark results and detects performance regressions.
It can be used to compare current benchmark results with a baseline or to analyze
trends over time.

Usage:
    python analyze_benchmarks.py [options]

Options:
    --baseline DIR    Directory containing baseline benchmark results
    --current DIR     Directory containing current benchmark results
    --threshold N     Regression threshold percentage (default: 10)
    --report FILE     Output report file (default: benchmark_report.md)
    --json            Output JSON format instead of Markdown
    --verbose         Enable verbose output
"""

import argparse
import json
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any, Tuple, Optional


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(description="Analyze benchmark results and detect regressions")
    parser.add_argument("--baseline", help="Directory containing baseline benchmark results")
    parser.add_argument("--current", help="Directory containing current benchmark results")
    parser.add_argument("--threshold", type=float, default=10.0, help="Regression threshold percentage (default: 10)")
    parser.add_argument("--report", default="benchmark_report.md", help="Output report file (default: benchmark_report.md)")
    parser.add_argument("--json", action="store_true", help="Output JSON format instead of Markdown")
    parser.add_argument("--verbose", action="store_true", help="Enable verbose output")
    
    return parser.parse_args()


def find_criterion_results(directory: str) -> List[Path]:
    """Find all Criterion benchmark result files in a directory."""
    if not os.path.exists(directory):
        print(f"Error: Directory {directory} does not exist")
        sys.exit(1)
        
    result_files = list(Path(directory).glob("**/estimates.json"))
    if not result_files:
        print(f"Warning: No benchmark results found in {directory}")
        
    return result_files


def parse_benchmark_results(result_files: List[Path]) -> Dict[str, Any]:
    """Parse benchmark results from Criterion JSON files."""
    results = {}
    
    for file_path in result_files:
        # Extract benchmark name from path
        # Format is typically: target/criterion/benchmark_name/estimates.json
        parts = file_path.parts
        if "criterion" in parts:
            criterion_index = parts.index("criterion")
            if criterion_index + 1 < len(parts):
                benchmark_name = parts[criterion_index + 1]
                
                # Handle nested benchmark groups
                if criterion_index + 2 < len(parts) and parts[criterion_index + 2] != "estimates.json":
                    benchmark_name = f"{benchmark_name}/{parts[criterion_index + 2]}"
                
                try:
                    with open(file_path, 'r') as f:
                        data = json.load(f)
                        results[benchmark_name] = data
                except json.JSONDecodeError:
                    print(f"Error: Failed to parse {file_path}")
                except Exception as e:
                    print(f"Error reading {file_path}: {e}")
    
    return results


def compare_results(baseline: Dict[str, Any], current: Dict[str, Any], threshold: float) -> Tuple[List[Dict], List[Dict]]:
    """Compare baseline and current results to detect regressions and improvements."""
    regressions = []
    improvements = []
    
    for benchmark, current_data in current.items():
        if benchmark in baseline:
            baseline_data = baseline[benchmark]
            
            # Extract mean values
            try:
                baseline_mean = float(baseline_data["estimates"]["mean"]["point_estimate"])
                current_mean = float(current_data["estimates"]["mean"]["point_estimate"])
                
                # Calculate percentage change
                percent_change = ((current_mean - baseline_mean) / baseline_mean) * 100
                
                if percent_change > threshold:
                    # Regression detected
                    regressions.append({
                        "benchmark": benchmark,
                        "baseline": baseline_mean,
                        "current": current_mean,
                        "change": percent_change
                    })
                elif percent_change < -threshold:
                    # Improvement detected
                    improvements.append({
                        "benchmark": benchmark,
                        "baseline": baseline_mean,
                        "current": current_mean,
                        "change": percent_change
                    })
            except (KeyError, TypeError) as e:
                print(f"Error comparing {benchmark}: {e}")
        else:
            print(f"Warning: Benchmark {benchmark} not found in baseline")
    
    return regressions, improvements


def analyze_single_results(results: Dict[str, Any]) -> Dict[str, Any]:
    """Analyze a single set of benchmark results."""
    analysis = {
        "benchmarks": {},
        "summary": {
            "total": len(results),
            "fastest": {"name": "", "time": float('inf')},
            "slowest": {"name": "", "time": 0},
            "average": 0
        }
    }
    
    total_time = 0
    
    for benchmark, data in results.items():
        try:
            mean = float(data["estimates"]["mean"]["point_estimate"])
            std_dev = float(data["estimates"]["std_dev"]["point_estimate"])
            
            analysis["benchmarks"][benchmark] = {
                "mean": mean,
                "std_dev": std_dev,
                "std_dev_percent": (std_dev / mean) * 100 if mean > 0 else 0
            }
            
            total_time += mean
            
            # Update fastest/slowest
            if mean < analysis["summary"]["fastest"]["time"]:
                analysis["summary"]["fastest"] = {"name": benchmark, "time": mean}
            
            if mean > analysis["summary"]["slowest"]["time"]:
                analysis["summary"]["slowest"] = {"name": benchmark, "time": mean}
                
        except (KeyError, TypeError) as e:
            print(f"Error analyzing {benchmark}: {e}")
    
    if results:
        analysis["summary"]["average"] = total_time / len(results)
    
    return analysis


def generate_markdown_report(analysis: Dict[str, Any], regressions: Optional[List[Dict]] = None, 
                           improvements: Optional[List[Dict]] = None) -> str:
    """Generate a Markdown report from the analysis results."""
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    report = f"# Benchmark Analysis Report\n\n"
    report += f"Generated: {now}\n\n"
    
    # Summary section
    report += "## Summary\n\n"
    report += f"- Total benchmarks: {analysis['summary']['total']}\n"
    report += f"- Average execution time: {analysis['summary']['average']:.2f} ns\n"
    report += f"- Fastest benchmark: {analysis['summary']['fastest']['name']} ({analysis['summary']['fastest']['time']:.2f} ns)\n"
    report += f"- Slowest benchmark: {analysis['summary']['slowest']['name']} ({analysis['summary']['slowest']['time']:.2f} ns)\n\n"
    
    # Regressions section (if comparing)
    if regressions is not None:
        report += "## Performance Regressions\n\n"
        
        if regressions:
            report += "| Benchmark | Baseline (ns) | Current (ns) | Change (%) |\n"
            report += "|-----------|--------------|--------------|------------|\n"
            
            for reg in sorted(regressions, key=lambda x: x["change"], reverse=True):
                report += f"| {reg['benchmark']} | {reg['baseline']:.2f} | {reg['current']:.2f} | +{reg['change']:.2f} |\n"
        else:
            report += "No performance regressions detected.\n"
        
        report += "\n"
    
    # Improvements section (if comparing)
    if improvements is not None:
        report += "## Performance Improvements\n\n"
        
        if improvements:
            report += "| Benchmark | Baseline (ns) | Current (ns) | Change (%) |\n"
            report += "|-----------|--------------|--------------|------------|\n"
            
            for imp in sorted(improvements, key=lambda x: x["change"]):
                report += f"| {imp['benchmark']} | {imp['baseline']:.2f} | {imp['current']:.2f} | {imp['change']:.2f} |\n"
        else:
            report += "No performance improvements detected.\n"
        
        report += "\n"
    
    # Detailed results
    report += "## Detailed Results\n\n"
    report += "| Benchmark | Mean (ns) | Std Dev (ns) | Std Dev (%) |\n"
    report += "|-----------|-----------|--------------|-------------|\n"
    
    for benchmark, data in sorted(analysis["benchmarks"].items()):
        report += f"| {benchmark} | {data['mean']:.2f} | {data['std_dev']:.2f} | {data['std_dev_percent']:.2f} |\n"
    
    return report


def main():
    args = parse_args()
    
    # Find and parse benchmark results
    if args.current:
        print(f"Analyzing current benchmark results from {args.current}")
        current_files = find_criterion_results(args.current)
        current_results = parse_benchmark_results(current_files)
        current_analysis = analyze_single_results(current_results)
    else:
        # Default to image-optimize and image-build target directories
        optimize_dir = "tools/image-optimize/target/criterion"
        build_dir = "tools/image-build/target/criterion"
        
        print(f"No current directory specified, looking in {optimize_dir} and {build_dir}")
        
        optimize_files = find_criterion_results(optimize_dir)
        build_files = find_criterion_results(build_dir)
        
        current_files = optimize_files + build_files
        current_results = parse_benchmark_results(current_files)
        current_analysis = analyze_single_results(current_results)
    
    # Compare with baseline if provided
    regressions = None
    improvements = None
    
    if args.baseline:
        print(f"Comparing with baseline benchmark results from {args.baseline}")
        baseline_files = find_criterion_results(args.baseline)
        baseline_results = parse_benchmark_results(baseline_files)
        
        regressions, improvements = compare_results(
            baseline_results, current_results, args.threshold)
        
        if regressions:
            print(f"Found {len(regressions)} performance regressions")
            if args.verbose:
                for reg in regressions:
                    print(f"  {reg['benchmark']}: {reg['baseline']:.2f} ns -> {reg['current']:.2f} ns ({reg['change']:.2f}%)")
        else:
            print("No performance regressions detected")
        
        if improvements:
            print(f"Found {len(improvements)} performance improvements")
            if args.verbose:
                for imp in improvements:
                    print(f"  {imp['benchmark']}: {imp['baseline']:.2f} ns -> {imp['current']:.2f} ns ({imp['change']:.2f}%)")
    
    # Generate report
    if args.json:
        report_data = {
            "analysis": current_analysis,
            "timestamp": datetime.now().isoformat()
        }
        
        if regressions is not None:
            report_data["regressions"] = regressions
        
        if improvements is not None:
            report_data["improvements"] = improvements
        
        with open(args.report, 'w') as f:
            json.dump(report_data, f, indent=2)
    else:
        report = generate_markdown_report(current_analysis, regressions, improvements)
        
        with open(args.report, 'w') as f:
            f.write(report)
    
    print(f"Report written to {args.report}")


if __name__ == "__main__":
    main() 