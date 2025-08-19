//! Diagnostic generation for LSP
//!
//! Converts linting violations into LSP diagnostics that can be displayed
//! in code editors with proper ranges, messages, and severity levels.

use crate::lsp::document::Document;
use crate::{configuration::Config, linter::{Linter, Violation}, file_utils};
use anyhow::Result;
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

/// Generates LSP diagnostics for KQL documents
pub struct DiagnosticGenerator {
    linter: Linter,
}

impl DiagnosticGenerator {
    /// Create a new diagnostic generator
    pub fn new(config: Config) -> Result<Self> {
        let linter = Linter::new(config);
        Ok(Self { linter })
    }

    /// Generate diagnostics for a document
    pub fn generate_diagnostics(&mut self, document: &Document) -> Result<Vec<Diagnostic>> {
        // Only process KQL files
        if !file_utils::is_kql_document(&document.uri, Some(&document.language_id)) {
            return Ok(Vec::new());
        }

        // Use the shared Linter to detect violations
        let lint_result = self.linter.lint_content(&document.text, None)?;
        
        // Convert violations to LSP diagnostics
        Ok(violations_to_diagnostics(&lint_result.violations, document))
    }

    /// Update configuration
    pub fn update_config(&mut self, config: Config) {
        self.linter = Linter::new(config);
    }
}

/// Convert klint violations to LSP diagnostics (for compatibility)
pub fn violations_to_diagnostics(
    violations: &[Violation],
    document: &Document,
) -> Vec<Diagnostic> {
    violations
        .iter()
        .filter_map(|violation| violation_to_diagnostic(violation, document))
        .collect()
}

/// Convert a single violation to an LSP diagnostic
fn violation_to_diagnostic(violation: &Violation, document: &Document) -> Option<Diagnostic> {
    // For now, we'll use line-based positioning since the existing Violation
    // structure uses line numbers. In the future, we could enhance this
    // with byte-accurate positioning.
    let line = violation.line.saturating_sub(1) as u32; // Convert to 0-based
    
    // Try to find the table name in the line for precise positioning
    let lines: Vec<&str> = document.text.lines().collect();
    if let Some(line_text) = lines.get(line as usize) {
        if let Some(start_col) = line_text.find(&violation.table_name) {
            let end_col = start_col + violation.table_name.len();
            
            let range = Range::new(
                Position::new(line, start_col as u32),
                Position::new(line, end_col as u32),
            );

            return Some(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(lsp_types::NumberOrString::String("klint-table-naming".to_string())),
                source: Some("klint".to_string()),
                message: violation.message.clone(),
                related_information: None,
                tags: None,
                data: None,
                code_description: None,
            });
        }
    }

    // Fallback: highlight the entire line
    Some(Diagnostic {
        range: Range::new(Position::new(line, 0), Position::new(line, u32::MAX)),
        severity: Some(DiagnosticSeverity::WARNING),
        code: Some(lsp_types::NumberOrString::String("klint-table-naming".to_string())),
        source: Some("klint".to_string()),
        message: violation.message.clone(),
        related_information: None,
        tags: None,
        data: None,
        code_description: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::ConfigBuilder;
    use lsp_types::Url;

    #[test]
    fn test_diagnostic_generation() {
        let config = ConfigBuilder::new()
            .table_naming("PascalCase")
            .build();

        let mut generator = DiagnosticGenerator::new(config).unwrap();

        let document = Document::new(
            Url::parse("file:///test.kql").unwrap(),
            "kql".to_string(),
            1,
            ".create table snake_case_table (id: int)".to_string(),
        );

        let diagnostics = generator.generate_diagnostics(&document).unwrap();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::WARNING));
        assert!(diagnostics[0].message.contains("snake_case_table"));
        assert!(diagnostics[0].message.contains("PascalCase"));
    }

    #[test]
    fn test_no_diagnostics_for_correct_naming() {
        let config = ConfigBuilder::new()
            .table_naming("PascalCase")
            .build();

        let mut generator = DiagnosticGenerator::new(config).unwrap();

        let document = Document::new(
            Url::parse("file:///test.kql").unwrap(),
            "kql".to_string(),
            1,
            ".create table PascalCaseTable (id: int)".to_string(),
        );

        let diagnostics = generator.generate_diagnostics(&document).unwrap();
        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_excluded_tables_no_diagnostics() {
        let config = ConfigBuilder::new()
            .table_naming("PascalCase")
            .exclude_tables(vec!["legacy_table".to_string()])
            .build();

        let mut generator = DiagnosticGenerator::new(config).unwrap();

        let document = Document::new(
            Url::parse("file:///test.kql").unwrap(),
            "kql".to_string(),
            1,
            ".create table legacy_table (id: int)".to_string(),
        );

        let diagnostics = generator.generate_diagnostics(&document).unwrap();
        assert_eq!(diagnostics.len(), 0);
    }
}