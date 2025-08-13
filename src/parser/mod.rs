//! KQL Parser module for extracting table names from KQL management commands

pub mod lexer;
pub mod ast;

use anyhow::Result;
use tracing::trace;

/// Parser for KQL management commands
pub struct Parser {
    content: String,
}

impl Parser {
    /// Create a new parser with the given KQL content
    /// 
    /// # Arguments
    /// 
    /// * `content` - The KQL content to parse
    /// 
    /// # Returns
    /// 
    /// A new Parser instance
    pub fn new(content: String) -> Self {
        Self { content }
    }

    /// Parse the content and extract table management commands
    /// 
    /// Currently supports `.create table` commands with simple pattern matching.
    /// 
    /// # Returns
    /// 
    /// A vector of TableStatement objects representing found table definitions
    pub fn parse(&self) -> Result<Vec<TableStatement>> {
        trace!("Starting to parse KQL content");
        
        let mut statements = Vec::new();
        let lines = self.content.lines().enumerate();
        
        for (line_num, line) in lines {
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            
            // Look for .create table commands
            if trimmed.starts_with(".create table ") {
                if let Some(table_name) = self.extract_table_name(trimmed) {
                    statements.push(TableStatement {
                        statement_type: StatementType::CreateTable,
                        table_name,
                        line: line_num + 1,
                        column: 1,
                    });
                }
            }
        }
        
        trace!("Found {} table statements", statements.len());
        Ok(statements)
    }
    
    /// Extract table name from a .create table command
    fn extract_table_name(&self, line: &str) -> Option<String> {
        // Simple regex-like extraction for ".create table TableName ("
        let after_create = line.strip_prefix(".create table ")?;
        let table_name = after_create
            .split_whitespace()
            .next()?
            .trim_end_matches('(')
            .to_string();
            
        if table_name.is_empty() {
            None
        } else {
            Some(table_name)
        }
    }
}

/// Represents a table management command in KQL
#[derive(Debug, Clone, PartialEq)]
pub struct TableStatement {
    pub statement_type: StatementType,
    pub table_name: String,
    pub line: usize,
    pub column: usize,
}

/// Type of table management command
#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    CreateTable,
    CreateTableBasedOn,
}