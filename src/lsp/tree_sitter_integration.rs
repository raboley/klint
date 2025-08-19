//! Tree-sitter integration for KQL parsing
//!
//! Provides functionality to parse KQL documents using tree-sitter and
//! extract table names for linting analysis.

use anyhow::{Context, Result};
use tree_sitter::{Parser, Query, QueryCursor, Tree};

/// Tree-sitter integration for KQL parsing
pub struct KqlParser {
    parser: Parser,
    query: Query,
}

impl KqlParser {
    /// Create a new KQL parser
    pub fn new() -> Result<Self> {
        let language = tree_sitter_kusto::language();
        let mut parser = Parser::new();
        parser
            .set_language(language)
            .context("Failed to set tree-sitter language")?;

        // Query to find table names in CREATE TABLE statements
        let query_source = r#"
            ; Find CREATE TABLE statements
            (create_table_command
                (identifier) @table_name)

            ; Find table references in query statements
            (query_statement
                (identifier) @table_reference)
        "#;

        let query =
            Query::new(language, query_source).context("Failed to create tree-sitter query")?;

        Ok(Self { parser, query })
    }

    /// Parse KQL text and return the syntax tree
    fn parse(&mut self, text: &str) -> Result<Tree> {
        self.parser
            .parse(text, None)
            .context("Failed to parse KQL text")
    }

    /// Extract table names from a parsed tree
    fn extract_table_names(&self, tree: &Tree, source: &str) -> Result<Vec<TableReference>> {
        let mut cursor = QueryCursor::new();
        let captures = cursor.matches(&self.query, tree.root_node(), source.as_bytes());

        let mut table_references = Vec::new();

        for query_match in captures {
            for capture in query_match.captures {
                let node = capture.node;
                let capture_name = &self.query.capture_names()[capture.index as usize];

                let start_byte = node.start_byte();
                let end_byte = node.end_byte();
                let text = &source[start_byte..end_byte];

                let table_ref = match capture_name.as_str() {
                    "table_name" => TableReference {
                        name: text.to_string(),
                        start_byte,
                        end_byte,
                        reference_type: TableReferenceType::Definition,
                    },
                    "table_reference" => TableReference {
                        name: text.to_string(),
                        start_byte,
                        end_byte,
                        reference_type: TableReferenceType::Usage,
                    },
                    _ => continue,
                };

                table_references.push(table_ref);
            }
        }

        Ok(table_references)
    }

    /// Extract table references from KQL source code
    /// 
    /// Parses the KQL source and returns all table references found,
    /// including both table definitions (CREATE TABLE) and table usages.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The KQL source code to analyze
    /// 
    /// # Returns
    /// 
    /// A vector of TableReference objects with position information
    /// and reference type (definition or usage)
    pub fn extract_table_references(&mut self, text: &str) -> Result<Vec<TableReference>> {
        let tree = self.parse(text)?;
        self.extract_table_names(&tree, text)
    }
    
    /// Legacy method - use extract_table_references instead
    #[deprecated(since = "0.1.0", note = "Use extract_table_references for clearer naming")]
    pub fn parse_and_extract(&mut self, text: &str) -> Result<Vec<TableReference>> {
        self.extract_table_references(text)
    }
}

/// Represents a table reference found in KQL code
#[derive(Debug, Clone)]
pub struct TableReference {
    /// The table name
    pub name: String,
    /// Start byte offset in the source
    pub start_byte: usize,
    /// End byte offset in the source
    pub end_byte: usize,
    /// Type of reference (definition or usage)
    pub reference_type: TableReferenceType,
}

/// Type of table reference
#[derive(Debug, Clone, PartialEq)]
pub enum TableReferenceType {
    /// Table definition (CREATE TABLE)
    Definition,
    /// Table usage (in queries)
    Usage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create_table() {
        let mut parser = KqlParser::new().unwrap();
        let kql = ".create table MyTable (id: int, name: string)";

        let tables = parser.extract_table_references(kql).unwrap();
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].name, "MyTable");
        assert_eq!(tables[0].reference_type, TableReferenceType::Definition);
    }

    #[test]
    fn test_parse_query_statement() {
        let mut parser = KqlParser::new().unwrap();
        let kql = "MyTable | where id > 0";

        let tables = parser.extract_table_references(kql).unwrap();
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].name, "MyTable");
        assert_eq!(tables[0].reference_type, TableReferenceType::Usage);
    }

    #[test]
    fn test_parse_multiple_tables() {
        let mut parser = KqlParser::new().unwrap();
        let kql = r#"
            .create table FirstTable (
                id: int
            )
            .create table SecondTable (name: string)
            FirstTable | join SecondTable on id
        "#;

        let tables = parser.extract_table_references(kql).unwrap();

        let definitions: Vec<_> = tables
            .iter()
            .filter(|t| t.reference_type == TableReferenceType::Definition)
            .collect();
        let usages: Vec<_> = tables
            .iter()
            .filter(|t| t.reference_type == TableReferenceType::Usage)
            .collect();

        // Test that we find the table definitions
        assert_eq!(definitions.len(), 2);
        assert!(definitions.iter().any(|t| t.name == "FirstTable"));
        assert!(definitions.iter().any(|t| t.name == "SecondTable"));

        // Test that we find at least one usage
        assert!(usages.len() >= 1);
        assert!(usages.iter().any(|t| t.name == "FirstTable"));
    }
}
