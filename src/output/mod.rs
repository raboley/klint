//! Output formatting module

pub mod terminal;
pub mod json;
pub mod report;

use anyhow::Result;
use std::path::PathBuf;

use crate::linter::LintResult;

/// Output format type
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Terminal,
    Json,
}

/// Trait for output formatters
pub trait OutputFormatter {
    /// Format a single lint result
    fn format_result(&self, result: &LintResult) -> Result<String>;
    
    /// Format multiple lint results
    fn format_results(&self, results: &[LintResult]) -> Result<String>;
    
    /// Format a summary of results
    fn format_summary(&self, results: &[LintResult]) -> Result<String>;
}

/// Create an output formatter based on the format type
pub fn create_formatter(format: OutputFormat) -> Box<dyn OutputFormatter> {
    match format {
        OutputFormat::Terminal => Box::new(terminal::TerminalFormatter::new()),
        OutputFormat::Json => Box::new(json::JsonFormatter::new()),
    }
}