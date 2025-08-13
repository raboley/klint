use klint::{Config, Linter};

#[test]
fn test_table_naming_rule_disabled() {
    let config = Config::builder()
        .table_naming("PascalCase")
        .disable_rule("table-naming")
        .build();
    
    let linter = Linter::new(config);
    
    // This content has snake_case tables but should pass because rule is disabled
    let content = r#"
.create table user_profiles (
    user_id: string,
    username: string
)

.create table order_details (
    order_id: string,
    total: real
)
"#;
    
    let result = linter.lint_content(content, None).unwrap();
    
    // Should have no violations because table-naming rule is disabled
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_table_naming_rule_enabled() {
    let config = Config::builder()
        .table_naming("PascalCase")
        .build();
    
    let linter = Linter::new(config);
    
    // This content has snake_case tables and should fail with PascalCase expected
    let content = r#"
.create table user_profiles (
    user_id: string,
    username: string
)

.create table order_details (
    order_id: string,
    total: real
)
"#;
    
    let result = linter.lint_content(content, None).unwrap();
    
    // Should have violations because table-naming rule is enabled
    assert_eq!(result.violations.len(), 2);
    assert_eq!(result.violations[0].table_name, "user_profiles");
    assert_eq!(result.violations[1].table_name, "order_details");
}

#[test]
fn test_multiple_rules_disabled() {
    let config = Config::builder()
        .table_naming("snake_case")
        .disable_rules(vec!["table-naming", "future-rule"])
        .build();
    
    let linter = Linter::new(config);
    
    // PascalCase tables should pass because table-naming is disabled
    let content = r#"
.create table UserProfiles (
    UserId: string,
    Username: string
)
"#;
    
    let result = linter.lint_content(content, None).unwrap();
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_rule_disabling_config_methods() {
    let config = Config::builder()
        .disable_rule("table-naming")
        .build();
    
    // Test the is_rule_disabled method
    assert!(config.is_rule_disabled("table-naming"));
    assert!(!config.is_rule_disabled("other-rule"));
}

#[test]
fn test_disabled_rule_precedence_over_excluded_tables() {
    // When a rule is disabled, it should take precedence over excluded_tables
    let config = Config::builder()
        .table_naming("PascalCase")
        .exclude_table("user_profiles")  // This table is excluded
        .disable_rule("table-naming")    // But the whole rule is disabled
        .build();
    
    let linter = Linter::new(config);
    
    // Test with the excluded table and non-excluded table
    let content = r#"
.create table user_profiles (
    user_id: string
)

.create table order_details (
    order_id: string
)
"#;
    
    let result = linter.lint_content(content, None).unwrap();
    
    // Should have no violations because the rule is disabled entirely
    assert_eq!(result.violations.len(), 0);
}

#[test]
fn test_integration_with_sample_config() {
    // Test loading the actual no-table-checks sample config
    let config_path = std::path::Path::new("samples/table_name/configured/no-table-checks/.klint-config.yml");
    
    if config_path.exists() {
        let config = Config::from_file(config_path).unwrap();
        
        // Verify the configuration was loaded correctly
        assert!(config.is_rule_disabled("table-naming"));
        
        let linter = Linter::new(config);
        
        // Test with mixed naming conventions - should all pass
        let content = r#"
.create table user_profiles (user_id: string)
.create table OrderDetails (OrderId: string)  
.create table SYSTEM_METRICS (VALUE: real)
"#;
        
        let result = linter.lint_content(content, None).unwrap();
        assert_eq!(result.violations.len(), 0);
    }
}

#[test]
fn test_partial_disabled_sample_config() {
    // Test the new partial-disabled sample config
    let config_path = std::path::Path::new("samples/table_name/configured/partial-disabled/.klint-config.yml");
    
    if config_path.exists() {
        let config = Config::from_file(config_path).unwrap();
        
        // Verify the configuration was loaded correctly
        assert!(config.is_rule_disabled("table-naming"));
        
        let linter = Linter::new(config);
        
        // Load the sample file and verify it passes
        let sample_file = std::path::Path::new("samples/table_name/configured/partial-disabled/mixed_tables.kql");
        if sample_file.exists() {
            let content = std::fs::read_to_string(sample_file).unwrap();
            let result = linter.lint_content(&content, None).unwrap();
            assert_eq!(result.violations.len(), 0);
        }
    }
}