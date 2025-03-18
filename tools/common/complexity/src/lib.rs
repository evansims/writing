use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Represents a code complexity metric measurement
#[derive(Debug, Clone)]
pub struct ComplexityMetric {
    /// Name of the metric
    pub name: String,
    /// Value of the metric
    pub value: f64,
    /// Threshold for warning
    pub warning_threshold: f64,
    /// Threshold for error
    pub error_threshold: f64,
}

impl ComplexityMetric {
    /// Create a new complexity metric
    pub fn new(name: &str, value: f64, warning_threshold: f64, error_threshold: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
            warning_threshold,
            error_threshold,
        }
    }

    /// Check if the metric exceeds the warning threshold
    pub fn is_warning(&self) -> bool {
        self.value >= self.warning_threshold && self.value < self.error_threshold
    }

    /// Check if the metric exceeds the error threshold
    pub fn is_error(&self) -> bool {
        self.value >= self.error_threshold
    }

    /// Get the status of the metric
    pub fn status(&self) -> ComplexityStatus {
        if self.is_error() {
            ComplexityStatus::Error
        } else if self.is_warning() {
            ComplexityStatus::Warning
        } else {
            ComplexityStatus::Ok
        }
    }
}

/// Status of a complexity metric
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityStatus {
    /// Metric is within acceptable range
    Ok,
    /// Metric exceeds warning threshold
    Warning,
    /// Metric exceeds error threshold
    Error,
}

/// Represents complexity metrics for a function
#[derive(Debug, Clone)]
pub struct FunctionComplexity {
    /// Name of the function
    pub name: String,
    /// Line number where the function starts
    pub line: usize,
    /// Cyclomatic complexity
    pub cyclomatic_complexity: ComplexityMetric,
    /// Cognitive complexity
    pub cognitive_complexity: ComplexityMetric,
    /// Line count
    pub line_count: ComplexityMetric,
    /// Parameter count
    pub parameter_count: ComplexityMetric,
    /// Nesting depth
    pub nesting_depth: ComplexityMetric,
}

impl FunctionComplexity {
    /// Create a new function complexity measurement
    pub fn new(
        name: &str,
        line: usize,
        cyclomatic_complexity: f64,
        cognitive_complexity: f64,
        line_count: f64,
        parameter_count: f64,
        nesting_depth: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            line,
            cyclomatic_complexity: ComplexityMetric::new(
                "Cyclomatic Complexity",
                cyclomatic_complexity,
                10.0,
                20.0,
            ),
            cognitive_complexity: ComplexityMetric::new(
                "Cognitive Complexity",
                cognitive_complexity,
                15.0,
                30.0,
            ),
            line_count: ComplexityMetric::new("Line Count", line_count, 50.0, 100.0),
            parameter_count: ComplexityMetric::new("Parameter Count", parameter_count, 5.0, 8.0),
            nesting_depth: ComplexityMetric::new("Nesting Depth", nesting_depth, 3.0, 5.0),
        }
    }

    /// Get the highest severity status of all metrics
    pub fn overall_status(&self) -> ComplexityStatus {
        let metrics = [
            &self.cyclomatic_complexity,
            &self.cognitive_complexity,
            &self.line_count,
            &self.parameter_count,
            &self.nesting_depth,
        ];

        if metrics.iter().any(|m| m.is_error()) {
            ComplexityStatus::Error
        } else if metrics.iter().any(|m| m.is_warning()) {
            ComplexityStatus::Warning
        } else {
            ComplexityStatus::Ok
        }
    }

    /// Get a list of metrics that exceed thresholds
    pub fn exceeding_metrics(&self) -> Vec<&ComplexityMetric> {
        let metrics = [
            &self.cyclomatic_complexity,
            &self.cognitive_complexity,
            &self.line_count,
            &self.parameter_count,
            &self.nesting_depth,
        ];

        metrics
            .iter()
            .filter(|m| m.is_warning() || m.is_error())
            .copied()
            .collect()
    }
}

/// Represents complexity metrics for a file
#[derive(Debug, Clone)]
pub struct FileComplexity {
    /// Path to the file
    pub path: PathBuf,
    /// Function complexity measurements
    pub functions: Vec<FunctionComplexity>,
    /// Overall file metrics
    pub metrics: HashMap<String, ComplexityMetric>,
}

impl FileComplexity {
    /// Create a new file complexity measurement
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            functions: Vec::new(),
            metrics: HashMap::new(),
        }
    }

    /// Add a function complexity measurement
    pub fn add_function(&mut self, function: FunctionComplexity) {
        self.functions.push(function);
    }

    /// Add a file-level metric
    pub fn add_metric(&mut self, name: &str, value: f64, warning_threshold: f64, error_threshold: f64) {
        self.metrics.insert(
            name.to_string(),
            ComplexityMetric::new(name, value, warning_threshold, error_threshold),
        );
    }

    /// Get the highest severity status of all functions and metrics
    pub fn overall_status(&self) -> ComplexityStatus {
        let function_status = self.functions.iter().map(|f| f.overall_status());
        let metric_status = self.metrics.values().map(|m| m.status());

        let all_statuses = function_status.chain(metric_status);

        if all_statuses.clone().any(|s| s == ComplexityStatus::Error) {
            ComplexityStatus::Error
        } else if all_statuses.any(|s| s == ComplexityStatus::Warning) {
            ComplexityStatus::Warning
        } else {
            ComplexityStatus::Ok
        }
    }

    /// Get functions that exceed complexity thresholds
    pub fn complex_functions(&self) -> Vec<&FunctionComplexity> {
        self.functions
            .iter()
            .filter(|f| f.overall_status() != ComplexityStatus::Ok)
            .collect()
    }

    /// Get metrics that exceed thresholds
    pub fn exceeding_metrics(&self) -> Vec<&ComplexityMetric> {
        self.metrics
            .values()
            .filter(|m| m.is_warning() || m.is_error())
            .collect()
    }
}

/// Represents complexity metrics for a codebase
#[derive(Debug, Clone)]
pub struct CodebaseComplexity {
    /// File complexity measurements
    pub files: Vec<FileComplexity>,
    /// Overall codebase metrics
    pub metrics: HashMap<String, ComplexityMetric>,
}

impl CodebaseComplexity {
    /// Create a new codebase complexity measurement
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            metrics: HashMap::new(),
        }
    }

    /// Add a file complexity measurement
    pub fn add_file(&mut self, file: FileComplexity) {
        self.files.push(file);
    }

    /// Add a codebase-level metric
    pub fn add_metric(&mut self, name: &str, value: f64, warning_threshold: f64, error_threshold: f64) {
        self.metrics.insert(
            name.to_string(),
            ComplexityMetric::new(name, value, warning_threshold, error_threshold),
        );
    }

    /// Get the highest severity status of all files and metrics
    pub fn overall_status(&self) -> ComplexityStatus {
        let file_status = self.files.iter().map(|f| f.overall_status());
        let metric_status = self.metrics.values().map(|m| m.status());

        let all_statuses = file_status.chain(metric_status);

        if all_statuses.clone().any(|s| s == ComplexityStatus::Error) {
            ComplexityStatus::Error
        } else if all_statuses.any(|s| s == ComplexityStatus::Warning) {
            ComplexityStatus::Warning
        } else {
            ComplexityStatus::Ok
        }
    }

    /// Get files that exceed complexity thresholds
    pub fn complex_files(&self) -> Vec<&FileComplexity> {
        self.files
            .iter()
            .filter(|f| f.overall_status() != ComplexityStatus::Ok)
            .collect()
    }

    /// Get metrics that exceed thresholds
    pub fn exceeding_metrics(&self) -> Vec<&ComplexityMetric> {
        self.metrics
            .values()
            .filter(|m| m.is_warning() || m.is_error())
            .collect()
    }

    /// Generate a summary report of complexity metrics
    pub fn summary_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# Complexity Metrics Summary\n\n");

        // Overall status
        let status = match self.overall_status() {
            ComplexityStatus::Ok => "✅ OK",
            ComplexityStatus::Warning => "⚠️ WARNING",
            ComplexityStatus::Error => "❌ ERROR",
        };
        report.push_str(&format!("Overall Status: {}\n\n", status));

        // Codebase metrics
        if !self.metrics.is_empty() {
            report.push_str("## Codebase Metrics\n\n");
            report.push_str("| Metric | Value | Warning Threshold | Error Threshold | Status |\n");
            report.push_str("|--------|-------|-------------------|-----------------|--------|\n");

            for metric in self.metrics.values() {
                let status = match metric.status() {
                    ComplexityStatus::Ok => "✅",
                    ComplexityStatus::Warning => "⚠️",
                    ComplexityStatus::Error => "❌",
                };
                report.push_str(&format!(
                    "| {} | {:.2} | {:.2} | {:.2} | {} |\n",
                    metric.name, metric.value, metric.warning_threshold, metric.error_threshold, status
                ));
            }
            report.push_str("\n");
        }

        // Complex files
        let complex_files = self.complex_files();
        if !complex_files.is_empty() {
            report.push_str("## Files Exceeding Complexity Thresholds\n\n");

            for file in complex_files {
                let status = match file.overall_status() {
                    ComplexityStatus::Ok => "✅",
                    ComplexityStatus::Warning => "⚠️",
                    ComplexityStatus::Error => "❌",
                };
                report.push_str(&format!("### {} {}\n\n", status, file.path.display()));

                // File metrics
                if !file.metrics.is_empty() {
                    report.push_str("#### File Metrics\n\n");
                    report.push_str("| Metric | Value | Warning Threshold | Error Threshold | Status |\n");
                    report.push_str("|--------|-------|-------------------|-----------------|--------|\n");

                    for metric in file.metrics.values() {
                        let status = match metric.status() {
                            ComplexityStatus::Ok => "✅",
                            ComplexityStatus::Warning => "⚠️",
                            ComplexityStatus::Error => "❌",
                        };
                        report.push_str(&format!(
                            "| {} | {:.2} | {:.2} | {:.2} | {} |\n",
                            metric.name, metric.value, metric.warning_threshold, metric.error_threshold, status
                        ));
                    }
                    report.push_str("\n");
                }

                // Complex functions
                let complex_functions = file.complex_functions();
                if !complex_functions.is_empty() {
                    report.push_str("#### Functions Exceeding Complexity Thresholds\n\n");

                    for function in complex_functions {
                        let status = match function.overall_status() {
                            ComplexityStatus::Ok => "✅",
                            ComplexityStatus::Warning => "⚠️",
                            ComplexityStatus::Error => "❌",
                        };
                        report.push_str(&format!("##### {} {} (line {})\n\n", status, function.name, function.line));

                        report.push_str("| Metric | Value | Warning Threshold | Error Threshold | Status |\n");
                        report.push_str("|--------|-------|-------------------|-----------------|--------|\n");

                        let exceeding_metrics = function.exceeding_metrics();
                        for metric in exceeding_metrics {
                            let status = match metric.status() {
                                ComplexityStatus::Ok => "✅",
                                ComplexityStatus::Warning => "⚠️",
                                ComplexityStatus::Error => "❌",
                            };
                            report.push_str(&format!(
                                "| {} | {:.2} | {:.2} | {:.2} | {} |\n",
                                metric.name, metric.value, metric.warning_threshold, metric.error_threshold, status
                            ));
                        }
                        report.push_str("\n");
                    }
                }
            }
        }

        report
    }
}

/// Default implementation for creating an empty codebase complexity
impl Default for CodebaseComplexity {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to calculate a recommended target for complexity reduction
pub fn recommend_complexity_target(current: f64, threshold: f64) -> f64 {
    if current <= threshold {
        // Already below threshold, no reduction needed
        return current;
    }

    // Recommend a target that is 10% below the threshold or 20% below current, whichever is less
    let target_based_on_threshold = threshold * 0.9;
    let target_based_on_current = current * 0.8;

    target_based_on_threshold.min(target_based_on_current)
}

/// Helper function to generate recommendations for complexity reduction
pub fn generate_complexity_recommendations(function: &FunctionComplexity) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Check cyclomatic complexity
    if function.cyclomatic_complexity.is_warning() || function.cyclomatic_complexity.is_error() {
        recommendations.push(format!(
            "Reduce cyclomatic complexity (currently {:.1}) by extracting conditions into helper functions or simplifying logic.",
            function.cyclomatic_complexity.value
        ));
    }

    // Check cognitive complexity
    if function.cognitive_complexity.is_warning() || function.cognitive_complexity.is_error() {
        recommendations.push(format!(
            "Reduce cognitive complexity (currently {:.1}) by simplifying control structures and reducing nesting.",
            function.cognitive_complexity.value
        ));
    }

    // Check line count
    if function.line_count.is_warning() || function.line_count.is_error() {
        recommendations.push(format!(
            "Break up function (currently {:.0} lines) into smaller, focused functions.",
            function.line_count.value
        ));
    }

    // Check parameter count
    if function.parameter_count.is_warning() || function.parameter_count.is_error() {
        recommendations.push(format!(
            "Reduce parameter count (currently {:.0}) by grouping related parameters into structs or using builder pattern.",
            function.parameter_count.value
        ));
    }

    // Check nesting depth
    if function.nesting_depth.is_warning() || function.nesting_depth.is_error() {
        recommendations.push(format!(
            "Reduce nesting depth (currently {:.0}) by extracting inner blocks into helper functions or using early returns.",
            function.nesting_depth.value
        ));
    }

    recommendations
}