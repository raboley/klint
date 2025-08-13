//! Abstract Syntax Tree definitions for KQL statements

/// AST node for a KQL management command
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    CreateTable {
        name: String,
        columns: Vec<Column>,
        properties: Vec<Property>,
        location: Location,
    },
    CreateTableBasedOn {
        name: String,
        source_table: String,
        if_not_exists: bool,
        properties: Vec<Property>,
        location: Location,
    },
}

/// Column definition in KQL
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: String,
}

/// Table property (used in WITH clause)
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: String,
}

/// Source location information
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}