//! Auto-fix implementation for violations

use anyhow::Result;
use tracing::{debug, info};

use crate::case_detector::convert_case;
use crate::linter::{LintResult, Violation};

/// Fixer for automatically correcting violations
pub struct Fixer;

impl Fixer {
    /// Fix all violations in a lint result
    pub fn fix(lint_result: &LintResult) -> Result<String> {
        let mut content = lint_result.content.clone();
        
        // Sort violations by position (reverse order to maintain positions)
        let mut violations = lint_result.violations.clone();
        violations.sort_by(|a, b| b.line.cmp(&a.line).then(b.column.cmp(&a.column)));
        
        for violation in violations {
            content = Self::fix_violation(&content, &violation)?;
        }
        
        info!("Fixed {} violations", lint_result.violations.len());
        Ok(content)
    }

    /// Fix a single violation
    fn fix_violation(content: &str, violation: &Violation) -> Result<String> {
        debug!(
            "Fixing table '{}' from {} to {}",
            violation.table_name,
            violation.current_case.to_string(),
            violation.expected_case.to_string()
        );
        
        let fixed_name = convert_case(
            &violation.table_name,
            violation.current_case,
            violation.expected_case,
        )?;
        
        // Replace all occurrences of the table name
        // This is a simple implementation - in reality, we'd need to be more careful
        // to only replace actual table references, not strings or comments
        let fixed_content = content.replace(&violation.table_name, &fixed_name);
        
        Ok(fixed_content)
    }

    /// Preview fixes without applying them
    pub fn preview_fixes(lint_result: &LintResult) -> Vec<FixPreview> {
        lint_result
            .violations
            .iter()
            .filter_map(|violation| {
                convert_case(
                    &violation.table_name,
                    violation.current_case,
                    violation.expected_case,
                )
                .ok()
                .map(|fixed_name| FixPreview {
                    original: violation.table_name.clone(),
                    fixed: fixed_name,
                    line: violation.line,
                })
            })
            .collect()
    }
}

/// Preview of a fix
#[derive(Debug, Clone)]
pub struct FixPreview {
    pub original: String,
    pub fixed: String,
    pub line: usize,
}