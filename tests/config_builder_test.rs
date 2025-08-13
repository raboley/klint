//! Integration tests for the Config builder pattern

use klint::Config;

#[test]
fn test_config_builder_basic() {
    let config = Config::builder()
        .table_naming("snake_case")
        .build();
    
    assert_eq!(config.rules.table_naming, "snake_case");
    assert_eq!(config.output.format, "terminal"); // default
    assert!(config.output.colors); // default
}

#[test]
fn test_config_builder_full() {
    let config = Config::builder()
        .table_naming("PascalCase")
        .exclude_table("legacy_table")
        .exclude_table("old_data")
        .output_format("json")
        .colors(false)
        .exclude_pattern("*.tmp")
        .build();
    
    assert_eq!(config.rules.table_naming, "PascalCase");
    assert_eq!(config.rules.excluded_tables, vec!["legacy_table", "old_data"]);
    assert_eq!(config.output.format, "json");
    assert!(!config.output.colors);
    assert_eq!(config.exclude, vec!["*.tmp"]);
}

#[test]
fn test_config_builder_with_validation() {
    let result = Config::builder()
        .table_naming("snake_case")
        .output_format("terminal")
        .build_validated();
    
    assert!(result.is_ok());
}

#[test]
fn test_config_builder_validation_fails() {
    let result = Config::builder()
        .table_naming("invalid_case")
        .build_validated();
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Invalid table naming convention"));
}

#[test]
fn test_config_builder_bulk_operations() {
    let tables = vec!["table1", "table2", "table3"];
    let patterns = vec!["*.log", "*.tmp"];
    
    let config = Config::builder()
        .exclude_tables(tables)
        .exclude(patterns)
        .build();
    
    assert_eq!(config.rules.excluded_tables, vec!["table1", "table2", "table3"]);
    assert_eq!(config.exclude, vec!["*.log", "*.tmp"]);
}

#[test]
fn test_config_builder_chaining() {
    let config = Config::builder()
        .table_naming("camelCase")
        .exclude_table("legacy")
        .exclude_pattern("test_*")
        .colors(true)
        .output_format("terminal")
        .build();
    
    // Verify all settings were applied
    assert_eq!(config.rules.table_naming, "camelCase");
    assert_eq!(config.rules.excluded_tables, vec!["legacy"]);
    assert_eq!(config.exclude, vec!["test_*"]);
    assert!(config.output.colors);
    assert_eq!(config.output.format, "terminal");
}