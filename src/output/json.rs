//! JSON output formatter

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::linter::LintResult;
use crate::output::OutputFormatter;

/// JSON output formatter
pub struct JsonFormatter;

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Serialize, Deserialize)]
struct JsonViolation {
    file: Option<String>,
    line: usize,
    column: usize,
    table_name: String,
    current_case: String,
    expected_case: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct JsonOutput {
    violations: Vec<JsonViolation>,
    summary: JsonSummary,
}

#[derive(Serialize, Deserialize)]
struct JsonSummary {
    total_files: usize,
    files_with_violations: usize,
    total_violations: usize,
}

impl OutputFormatter for JsonFormatter {
    fn format_result(&self, result: &LintResult) -> Result<String> {
        let violations: Vec<JsonViolation> = result
            .violations
            .iter()
            .map(|v| JsonViolation {
                file: result.file_path.as_ref().map(|p| p.display().to_string()),
                line: v.line,
                column: v.column,
                table_name: v.table_name.clone(),
                current_case: v.current_case.to_string().to_string(),
                expected_case: v.expected_case.to_string().to_string(),
                message: v.message.clone(),
            })
            .collect();
        
        Ok(serde_json::to_string_pretty(&violations)?)
    }

    fn format_results(&self, results: &[LintResult]) -> Result<String> {
        let mut all_violations = Vec::new();
        
        for result in results {
            for violation in &result.violations {
                all_violations.push(JsonViolation {
                    file: result.file_path.as_ref().map(|p| p.display().to_string()),
                    line: violation.line,
                    column: violation.column,
                    table_name: violation.table_name.clone(),
                    current_case: violation.current_case.to_string().to_string(),
                    expected_case: violation.expected_case.to_string().to_string(),
                    message: violation.message.clone(),
                });
            }
        }
        
        let output = JsonOutput {
            violations: all_violations,
            summary: JsonSummary {
                total_files: results.len(),
                files_with_violations: results.iter().filter(|r| r.has_violations()).count(),
                total_violations: results.iter().map(|r| r.violations.len()).sum(),
            },
        };
        
        Ok(serde_json::to_string_pretty(&output)?)
    }

    fn format_summary(&self, results: &[LintResult]) -> Result<String> {
        let summary = JsonSummary {
            total_files: results.len(),
            files_with_violations: results.iter().filter(|r| r.has_violations()).count(),
            total_violations: results.iter().map(|r| r.violations.len()).sum(),
        };
        
        Ok(serde_json::to_string_pretty(&summary)?)
    }
}