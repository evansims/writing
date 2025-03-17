#!/usr/bin/env python3

import json
import glob
import os
from datetime import datetime
from statistics import mean, stdev
from typing import Dict, List, Any

def load_criterion_data(file_path: str) -> Dict[str, Any]:
    """Load and parse a Criterion benchmark result file."""
    with open(file_path) as f:
        return json.load(f)

def extract_metric(data: Dict[str, Any], metric: str = "mean") -> float:
    """Extract a specific metric from Criterion benchmark data."""
    return float(data["estimates"][metric]["point_estimate"])

def process_format_benchmarks(result_files: List[str]) -> Dict[str, Dict[str, List[float]]]:
    """Process benchmarks for different image formats."""
    format_data = {
        "jpeg": {"dates": [], "values": []},
        "webp": {"dates": [], "values": []},
        "avif": {"dates": [], "values": []}
    }
    
    for file in result_files:
        if "process_" not in file:
            continue
            
        data = load_criterion_data(file)
        date = datetime.fromtimestamp(os.path.getmtime(file)).isoformat()
        
        if "jpeg" in file:
            format_data["jpeg"]["dates"].append(date)
            format_data["jpeg"]["values"].append(extract_metric(data))
        elif "webp" in file:
            format_data["webp"]["dates"].append(date)
            format_data["webp"]["values"].append(extract_metric(data))
        elif "avif" in file:
            format_data["avif"]["dates"].append(date)
            format_data["avif"]["values"].append(extract_metric(data))
    
    return format_data

def process_size_benchmarks(result_files: List[str]) -> Dict[str, List[float]]:
    """Process benchmarks for different image sizes."""
    size_data = {
        "labels": ["single_size", "two_sizes", "four_sizes"],
        "values": []
    }
    
    for file in result_files:
        if "size_variants" not in file:
            continue
            
        data = load_criterion_data(file)
        size_data["values"].append(extract_metric(data))
    
    return size_data

def process_quality_benchmarks(result_files: List[str]) -> Dict[str, Any]:
    """Process benchmarks for different quality settings."""
    quality_data = {
        "formats": ["jpeg", "webp", "avif"],
        "qualities": [60, 75, 85, 95],
        "values": []
    }
    
    for format_name in quality_data["formats"]:
        format_values = []
        for quality in quality_data["qualities"]:
            matching_files = [f for f in result_files if f"quality_{quality}" in f and format_name in f]
            if matching_files:
                data = load_criterion_data(matching_files[0])
                format_values.append(extract_metric(data))
            else:
                format_values.append(None)
        quality_data["values"].append(format_values)
    
    return quality_data

def calculate_statistics(data: List[float]) -> Dict[str, float]:
    """Calculate basic statistics for a series of measurements."""
    return {
        "mean": mean(data),
        "stddev": stdev(data) if len(data) > 1 else 0,
        "min": min(data),
        "max": max(data)
    }

def detect_regressions(current: float, baseline: float, threshold: float = 0.1) -> bool:
    """Detect if current performance is significantly worse than baseline."""
    return (current - baseline) / baseline > threshold

def main():
    # Find all Criterion result files
    optimize_files = glob.glob("performance-data/image-optimize/**/estimates.json", recursive=True)
    build_files = glob.glob("performance-data/image-build/**/estimates.json", recursive=True)
    
    # Process data for each crate
    dashboard_data = {
        "image-optimize": {
            "formats": process_format_benchmarks(optimize_files),
            "sizes": process_size_benchmarks(optimize_files),
            "quality": process_quality_benchmarks(optimize_files)
        },
        "image-build": {
            "formats": process_format_benchmarks(build_files),
            "sizes": process_size_benchmarks(build_files),
            "quality": process_quality_benchmarks(build_files)
        }
    }
    
    # Calculate statistics and detect regressions
    for crate in dashboard_data:
        stats = {}
        for format_name, data in dashboard_data[crate]["formats"].items():
            if data["values"]:
                stats[format_name] = calculate_statistics(data["values"])
                
                # Detect regressions
                if len(data["values"]) > 1:
                    baseline = mean(data["values"][:-1])  # Use mean of previous values as baseline
                    current = data["values"][-1]
                    if detect_regressions(current, baseline):
                        print(f"WARNING: Performance regression detected in {crate} {format_name}")
                        print(f"  Baseline: {baseline:.2f}ms")
                        print(f"  Current:  {current:.2f}ms")
        
        dashboard_data[crate]["statistics"] = stats
    
    # Save processed data
    os.makedirs("dashboard/performance-data", exist_ok=True)
    with open("dashboard/performance-data/latest.json", "w") as f:
        json.dump(dashboard_data, f, indent=2)
    
    # Generate summary report
    with open("dashboard/performance-data/summary.md", "w") as f:
        f.write("# Performance Summary\n\n")
        for crate in dashboard_data:
            f.write(f"## {crate}\n\n")
            for format_name, stats in dashboard_data[crate]["statistics"].items():
                f.write(f"### {format_name}\n")
                f.write(f"- Mean: {stats['mean']:.2f}ms\n")
                f.write(f"- StdDev: {stats['stddev']:.2f}ms\n")
                f.write(f"- Range: {stats['min']:.2f}ms - {stats['max']:.2f}ms\n\n")

if __name__ == "__main__":
    main() 