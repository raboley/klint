//! Core linting engine

pub mod rules;
pub mod fixer;

use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use crate::case_detector::{detect_case, NamingCase};
use crate::config::Config;
use crate::parser::{Parser, TableStatement};

/// Main linter struct
pub struct Linter {
    config: Config,
}

impl Linter {
    /// Create a new linter with the given configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Lint content and return violations
    pub fn lint_content(&self, content: &str, file_path: Option<&PathBuf>) -> Result<LintResult> {
        let file_name = file_path
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("<content>");
        
        debug!("Linting file: {}", file_name);
        
        let parser = Parser::new(content.to_string());
        let statements = parser.parse()?;
        
        let mut violations = Vec::new();
        
        for statement in statements {
            if let Some(violation) = self.check_table_name(&statement)? {
                violations.push(violation);
            }
        }
        
        info!("Found {} violations in {}", violations.len(), file_name);
        
        Ok(LintResult {
            file_path: file_path.cloned(),
            violations,
            content: content.to_string(),
        })
    }

    /// Check a single table name for violations
    fn check_table_name(&self, statement: &TableStatement) -> Result<Option<Violation>> {
        // Skip if table is excluded
        if self.config.is_table_excluded(&statement.table_name) {
            debug!("Skipping excluded table: {}", statement.table_name);
            return Ok(None);
        }
        
        let current_case = detect_case(&statement.table_name);
        let expected_case = self.config.naming_case();
        
        if current_case != expected_case {
            let violation = Violation {
                line: statement.line,
                column: statement.column,
                table_name: statement.table_name.clone(),
                current_case,
                expected_case,
                message: format!(
                    "Table '{}' uses {} but should use {}",
                    statement.table_name,
                    current_case.to_string(),
                    expected_case.to_string()
                ),
            };
            
            warn!("{}", violation.message);
            return Ok(Some(violation));
        }
        
        Ok(None)
    }
}

/// Result of linting a file
#[derive(Debug, Clone)]
pub struct LintResult {
    pub file_path: Option<PathBuf>,
    pub violations: Vec<Violation>,
    pub content: String,
}

impl LintResult {
    /// Check if there are any violations
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }
}

/// A single linting violation
#[derive(Debug, Clone)]
pub struct Violation {
    pub line: usize,
    pub column: usize,
    pub table_name: String,
    pub current_case: NamingCase,
    pub expected_case: NamingCase,
    pub message: String,
}