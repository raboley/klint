//! Output formatting module

pub mod terminal;
pub mod json;
pub mod report;

use anyhow::Result;

use crate::linter::LintResult;

/// Output format type
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Terminal,
    Json,
}

/// Trait for output formatters
pub trait OutputFormatter {
    /// Format a single lint result for display
    /// 
    /// # Arguments
    /// 
    /// * `result` - The lint result to format
    /// 
    /// # Returns
    /// 
    /// Formatted string representation of the result
    fn format_result(&self, result: &LintResult) -> Result<String>;
    
    /// Format multiple lint results for display
    /// 
    /// # Arguments
    /// 
    /// * `results` - The lint results to format
    /// 
    /// # Returns
    /// 
    /// Formatted string representation of all results
    fn format_results(&self, results: &[LintResult]) -> Result<String>;
    
    /// Format a summary of lint results
    /// 
    /// # Arguments
    /// 
    /// * `results` - The lint results to summarize
    /// 
    /// # Returns
    /// 
    /// Formatted summary string
    fn format_summary(&self, results: &[LintResult]) -> Result<String>;
}

/// Create an output formatter based on the format type
/// 
/// # Arguments
/// 
/// * `format` - The desired output format
/// 
/// # Returns
/// 
/// A boxed trait object implementing OutputFormatter
pub fn create_formatter(format: OutputFormat) -> Box<dyn OutputFormatter> {
    match format {
        OutputFormat::Terminal => Box::new(terminal::TerminalFormatter::new()),
        OutputFormat::Json => Box::new(json::JsonFormatter::new()),
    }
}