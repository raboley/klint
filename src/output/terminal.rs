//! Colored terminal output formatter

use anyhow::Result;
use colored::*;

use crate::linter::LintResult;
use crate::output::OutputFormatter;

/// Terminal output formatter with colors
pub struct TerminalFormatter {
    use_colors: bool,
}

impl Default for TerminalFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalFormatter {
    pub fn new() -> Self {
        Self { use_colors: true }
    }

    pub fn without_colors() -> Self {
        Self { use_colors: false }
    }
}

impl OutputFormatter for TerminalFormatter {
    fn format_result(&self, result: &LintResult) -> Result<String> {
        let mut output = String::new();
        
        if let Some(path) = &result.file_path {
            let path_str = path.display().to_string();
            if self.use_colors {
                output.push_str(&path_str.blue().bold().to_string());
            } else {
                output.push_str(&path_str);
            }
            output.push('\n');
        }
        
        for violation in &result.violations {
            let line_info = format!("  {}:{}", violation.line, violation.column);
            let message = format!(
                " {} Table '{}' uses {} but should use {}",
                if self.use_colors { "✗".red().to_string() } else { "✗".to_string() },
                violation.table_name,
                violation.current_case.to_string(),
                violation.expected_case.to_string()
            );
            
            if self.use_colors {
                output.push_str(&line_info.yellow().to_string());
                output.push_str(&message);
            } else {
                output.push_str(&line_info);
                output.push_str(&message);
            }
            output.push('\n');
        }
        
        Ok(output)
    }

    fn format_results(&self, results: &[LintResult]) -> Result<String> {
        let mut output = String::new();
        
        for result in results {
            if result.has_violations() {
                output.push_str(&self.format_result(result)?);
                output.push('\n');
            }
        }
        
        Ok(output)
    }

    fn format_summary(&self, results: &[LintResult]) -> Result<String> {
        let total_files = results.len();
        let files_with_violations = results.iter().filter(|r| r.has_violations()).count();
        let total_violations: usize = results.iter().map(|r| r.violations.len()).sum();
        
        let files_text = if total_files == 1 {
            "1 file".to_string()
        } else {
            format!("{} files", total_files)
        };
        
        let summary = if total_violations == 0 {
            format!(
                "{} {} checked, {} found",
                if self.use_colors { "✓".green().to_string() } else { "✓".to_string() },
                files_text,
                if self.use_colors { "no violations".green().to_string() } else { "no violations".to_string() }
            )
        } else {
            let violations_text = if total_violations == 1 {
                if self.use_colors { "1 violation".red().to_string() } else { "1 violation".to_string() }
            } else if self.use_colors {
                format!("{} violations", total_violations).red().to_string()
            } else {
                format!("{} violations", total_violations)
            };
            
            let files_with_violations_text = if files_with_violations == 1 {
                "1 file".to_string()
            } else {
                format!("{} files", files_with_violations)
            };
            
            let files_checked_text = if total_files == 1 {
                "1 file checked".to_string()
            } else {
                format!("{} files checked", total_files)
            };
            
            format!(
                "{} Found {} in {} of {}",
                if self.use_colors { "✗".red().to_string() } else { "✗".to_string() },
                violations_text,
                files_with_violations_text,
                files_checked_text
            )
        };
        
        Ok(summary)
    }
}