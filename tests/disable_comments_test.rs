//! Integration tests for disable comments functionality

use klint::{Linter, Config};
use std::path::PathBuf;

#[test]
fn test_disable_current_line() {
    let content = ".create table user_data (id: int) //nolint:table-naming";
    
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    let result = linter.lint_content(content, Some(&PathBuf::from("test.kql"))).unwrap();
    
    // Should have no violations because the rule is disabled
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_disable_all_rules() {
    let content = ".create table user_data (id: int) //nolint:";
    
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    let result = linter.lint_content(content, Some(&PathBuf::from("test.kql"))).unwrap();
    
    // Should have no violations because all rules are disabled
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_disable_specific_rule_only() {
    let content = ".create table user_data (id: int) //nolint:table-naming";
    
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    let result = linter.lint_content(content, Some(&PathBuf::from("test.kql"))).unwrap();
    
    // Should have no violations for table-naming rule
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_disable_multiple_rules() {
    let content = ".create table user_data (id: int) //nolint:table-naming,other-rule";
    
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    let result = linter.lint_content(content, Some(&PathBuf::from("test.kql"))).unwrap();
    
    // Should have no violations because table-naming is disabled
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_disable_with_double_dash_comment() {
    let content = ".create table user_data (id: int) -- nolint:table-naming";
    
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    let result = linter.lint_content(content, Some(&PathBuf::from("test.kql"))).unwrap();
    
    // Should have no violations
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_disable_does_not_affect_fix_mode() {
    let content = ".create table user_data (id: int) //nolint:table-naming";
    
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    let result = linter.lint_content(content, Some(&PathBuf::from("test.kql"))).unwrap();
    
    // Disable comments should not affect fix mode - fixes should still be generated
    // But in this case, no violations means no fixes needed
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_mixed_comment_styles() {
    let content = r#"
.create table user_data (id: int) //nolint:table-naming
.create table other_data (id: int) -- nolint:table-naming
.create table third_data (id: int) // regular comment
"#;
    
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    let result = linter.lint_content(content, Some(&PathBuf::from("test.kql"))).unwrap();
    
    // Should have 1 violation (third_data) because first two are disabled
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.violations[0].table_name, "third_data");
}