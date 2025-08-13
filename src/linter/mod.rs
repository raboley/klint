//! Core linting engine

pub mod rules;
pub mod fixer;

use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use crate::case_detector::{detect_case, NamingCase};
use crate::configuration::Config;
use crate::disable_comments::{parse_disable_comments, is_rule_disabled, DisableDirective};
use crate::parser::{Parser, TableStatement};

/// Main linter struct
pub struct Linter {
    config: Config,
}

impl Linter {
    /// Create a new linter with the given configuration
    /// 
    /// # Arguments
    /// 
    /// * `config` - The linting configuration to use
    /// 
    /// # Returns
    /// 
    /// A new Linter instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Lint KQL content and return any naming violations found
    /// 
    /// # Arguments
    /// 
    /// * `content` - The KQL content to lint
    /// * `file_path` - Optional path to the file being linted (for reporting)
    /// 
    /// # Returns
    /// 
    /// A LintResult containing any violations found
    pub fn lint_content(&self, content: &str, file_path: Option<&PathBuf>) -> Result<LintResult> {
        let file_name = file_path
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("<content>");
        
        debug!("Linting file: {}", file_name);
        
        // Parse disable comments first
        let disable_directives = parse_disable_comments(content);
        
        let parser = Parser::new(content.to_string());
        let statements = parser.parse()?;
        
        let mut violations = Vec::new();
        
        for statement in statements {
            if let Some(violation) = self.check_table_name(&statement, &disable_directives)? {
                violations.push(violation);
            }
        }
        
        info!("Found {} violations in {} (after applying disable directives)", violations.len(), file_name);
        
        Ok(LintResult {
            file_path: file_path.cloned(),
            violations,
            content: content.to_string(),
            disable_directives,
        })
    }

    /// Check a single table name for violations
    fn check_table_name(&self, statement: &TableStatement, disable_directives: &[DisableDirective]) -> Result<Option<Violation>> {
        // Skip if table is excluded
        if self.config.is_table_excluded(&statement.table_name) {
            debug!("Skipping excluded table: {}", statement.table_name);
            return Ok(None);
        }
        
        // Check if table-naming rule is disabled for this line
        if is_rule_disabled(disable_directives, statement.line, "table-naming") {
            debug!("Skipping table '{}' on line {} due to disable directive", statement.table_name, statement.line);
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
                    current_case,
                    expected_case
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
    pub disable_directives: Vec<DisableDirective>,
}

impl LintResult {
    /// Check if there are any violations in this result
    /// 
    /// # Returns
    /// 
    /// True if violations were found, false otherwise
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