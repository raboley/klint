use anyhow::Result;
use klint::Config;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_valid_config_passes_validation() {
    let config_content = r#"
rules:
  table_naming: PascalCase
  excluded_tables: ["temp_*"]
output:
  format: terminal
  colors: true
"#;
    
    let config: Config = serde_yaml::from_str(config_content).expect("Failed to parse config");
    assert!(config.validate().is_ok(), "Valid config should pass validation");
}

#[test]
fn test_invalid_naming_case_fails_validation() {
    let config_content = r#"
rules:
  table_naming: InvalidCase
output:
  format: terminal
"#;
    
    let config: Config = serde_yaml::from_str(config_content).expect("Failed to parse config");
    let result = config.validate();
    
    assert!(result.is_err(), "Config with invalid naming case should fail validation");
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Invalid table naming convention"), "Error should mention invalid naming convention");
    assert!(error.contains("InvalidCase"), "Error should mention the invalid value");
    assert!(error.contains("PascalCase"), "Error should suggest valid options");
}

#[test]
fn test_invalid_output_format_fails_validation() {
    let config_content = r#"
rules:
  table_naming: PascalCase
output:
  format: invalid_format
"#;
    
    let config: Config = serde_yaml::from_str(config_content).expect("Failed to parse config");
    let result = config.validate();
    
    assert!(result.is_err(), "Config with invalid output format should fail validation");
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Invalid output format"), "Error should mention invalid output format");
    assert!(error.contains("invalid_format"), "Error should mention the invalid value");
}

#[test]
fn test_multiple_validation_errors() {
    let config_content = r#"
rules:
  table_naming: BadCase
output:
  format: bad_format
"#;
    
    let config: Config = serde_yaml::from_str(config_content).expect("Failed to parse config");
    let result = config.validate();
    
    assert!(result.is_err(), "Config with multiple errors should fail validation");
    let error = result.unwrap_err().to_string();
    assert!(error.contains("BadCase"), "Error should mention invalid naming case");
    assert!(error.contains("bad_format"), "Error should mention invalid output format");
}

#[test]
fn test_config_from_file_validates() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("test.yml");
    
    // Create invalid config file
    let invalid_content = r#"
rules:
  table_naming: InvalidCase
output:
  format: terminal
"#;
    
    fs::write(&config_path, invalid_content)?;
    
    let result = Config::from_file_validated(&config_path);
    assert!(result.is_err(), "Loading invalid config should fail");
    
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Configuration validation failed"), "Error should mention validation failure");
    
    Ok(())
}

#[test]
fn test_excluded_tables_validation() {
    let config_content = r#"
rules:
  table_naming: PascalCase
  excluded_tables: 
    - ""  # Empty string should be invalid
    - "valid_table"
output:
  format: terminal
"#;
    
    let config: Config = serde_yaml::from_str(config_content).expect("Failed to parse config");
    let result = config.validate();
    
    assert!(result.is_err(), "Config with empty excluded table name should fail validation");
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Empty table name"), "Error should mention empty table name");
}