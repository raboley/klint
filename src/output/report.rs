//! File report generation

use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::info;

use crate::linter::LintResult;

/// Generate a report file
pub fn generate_report<P: AsRef<Path>>(
    path: P,
    results: &[LintResult],
    format: ReportFormat,
) -> Result<()> {
    let content = match format {
        ReportFormat::Text => generate_text_report(results)?,
        ReportFormat::Markdown => generate_markdown_report(results)?,
    };
    
    fs::write(&path, content)?;
    info!("Report written to: {:?}", path.as_ref());
    
    Ok(())
}

/// Report format
pub enum ReportFormat {
    Text,
    Markdown,
}

fn generate_text_report(results: &[LintResult]) -> Result<String> {
    let mut report = String::new();
    
    report.push_str("KQL Linter Report\n");
    report.push_str("=================\n\n");
    
    for result in results {
        if result.has_violations() {
            if let Some(path) = &result.file_path {
                report.push_str(&format!("File: {}\n", path.display()));
            }
            
            for violation in &result.violations {
                report.push_str(&format!(
                    "  Line {}: Table '{}' uses {} but should use {}\n",
                    violation.line,
                    violation.table_name,
                    violation.current_case.to_string(),
                    violation.expected_case.to_string()
                ));
            }
            
            report.push('\n');
        }
    }
    
    // Summary
    let total_files = results.len();
    let files_with_violations = results.iter().filter(|r| r.has_violations()).count();
    let total_violations: usize = results.iter().map(|r| r.violations.len()).sum();
    
    report.push_str("Summary\n");
    report.push_str("-------\n");
    report.push_str(&format!("Files checked: {}\n", total_files));
    report.push_str(&format!("Files with violations: {}\n", files_with_violations));
    report.push_str(&format!("Total violations: {}\n", total_violations));
    
    Ok(report)
}

fn generate_markdown_report(results: &[LintResult]) -> Result<String> {
    let mut report = String::new();
    
    report.push_str("# KQL Linter Report\n\n");
    
    if results.iter().any(|r| r.has_violations()) {
        report.push_str("## Violations\n\n");
        
        for result in results {
            if result.has_violations() {
                if let Some(path) = &result.file_path {
                    report.push_str(&format!("### {}\n\n", path.display()));
                }
                
                report.push_str("| Line | Column | Table Name | Current Case | Expected Case |\n");
                report.push_str("|------|--------|------------|--------------|---------------|\n");
                
                for violation in &result.violations {
                    report.push_str(&format!(
                        "| {} | {} | `{}` | {} | {} |\n",
                        violation.line,
                        violation.column,
                        violation.table_name,
                        violation.current_case.to_string(),
                        violation.expected_case.to_string()
                    ));
                }
                
                report.push('\n');
            }
        }
    }
    
    // Summary
    let total_files = results.len();
    let files_with_violations = results.iter().filter(|r| r.has_violations()).count();
    let total_violations: usize = results.iter().map(|r| r.violations.len()).sum();
    
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- **Files checked**: {}\n", total_files));
    report.push_str(&format!("- **Files with violations**: {}\n", files_with_violations));
    report.push_str(&format!("- **Total violations**: {}\n", total_violations));
    
    Ok(report)
}