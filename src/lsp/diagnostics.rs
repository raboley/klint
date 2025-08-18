//! Diagnostic generation for LSP
//!
//! Converts linting violations into LSP diagnostics that can be displayed
//! in code editors with proper ranges, messages, and severity levels.

use crate::lsp::document::Document;
use crate::lsp::tree_sitter_integration::{KqlParser, TableReference, TableReferenceType};
use crate::{case_detector, configuration::Config, linter::Violation, NamingCase};
use anyhow::Result;
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

/// Generates LSP diagnostics for KQL documents
pub struct DiagnosticGenerator {
    parser: KqlParser,
    config: Config,
}

impl DiagnosticGenerator {
    /// Create a new diagnostic generator
    pub fn new(config: Config) -> Result<Self> {
        let parser = KqlParser::new()?;
        Ok(Self { parser, config })
    }

    /// Generate diagnostics for a document
    pub fn generate_diagnostics(&mut self, document: &Document) -> Result<Vec<Diagnostic>> {
        // Only process KQL files
        if !self.is_kql_document(document) {
            return Ok(Vec::new());
        }

        let table_references = self.parser.parse_and_extract(&document.text)?;
        let mut diagnostics = Vec::new();

        for table_ref in table_references {
            // Only lint table definitions, not usages (per PRD requirements)
            if table_ref.reference_type != TableReferenceType::Definition {
                continue;
            }

            // Check if table should be excluded
            if self.config.is_table_excluded(&table_ref.name) {
                continue;
            }

            // Detect current case and check against expected convention
            let detected_case = case_detector::detect_case(&table_ref.name);
            let expected_case = self.config.naming_case();

            if detected_case != expected_case {
                let diagnostic = self.create_violation_diagnostic(
                    document,
                    &table_ref,
                    Some(detected_case),
                    expected_case,
                )?;
                diagnostics.push(diagnostic);
            }
        }

        Ok(diagnostics)
    }

    /// Check if document is a KQL file
    fn is_kql_document(&self, document: &Document) -> bool {
        matches!(document.language_id.as_str(), "kql" | "kusto")
            || document.uri.path().ends_with(".kql")
            || document.uri.path().ends_with(".kusto")
    }

    /// Create a diagnostic for a naming violation
    fn create_violation_diagnostic(
        &self,
        document: &Document,
        table_ref: &TableReference,
        detected_case: Option<NamingCase>,
        expected_case: NamingCase,
    ) -> Result<Diagnostic> {
        let range = document.byte_range_to_lsp_range(table_ref.start_byte, table_ref.end_byte);

        let message = match detected_case {
            Some(current_case) => {
                let suggested_name = case_detector::convert_case(&table_ref.name, current_case, expected_case)?;
                format!(
                    "Table '{}' uses {} but {} is expected. Consider renaming to '{}'",
                    table_ref.name,
                    current_case,
                    expected_case,
                    suggested_name
                )
            }
            None => {
                format!(
                    "Table '{}' doesn't follow any recognized naming convention. Expected: {}",
                    table_ref.name, expected_case
                )
            }
        };

        Ok(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(lsp_types::NumberOrString::String("klint-table-naming".to_string())),
            code_description: None,
            source: Some("klint".to_string()),
            message,
            related_information: None,
            tags: None,
            data: None,
        })
    }

    /// Update configuration
    pub fn update_config(&mut self, config: Config) {
        self.config = config;
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