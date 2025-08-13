use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_fix_mode_corrects_table_names() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with naming violations
    let kql_content = r#".create table user_profiles (
    id: int,
    name: string
)

.create table order_items (
    order_id: int,
    item_name: string
)

// Some other content
let results = user_profiles
| join kind=inner order_items on user_id
| project name, item_name
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with PascalCase naming
    let config_content = r#"
rules:
  table_naming: PascalCase
output:
  format: terminal
"#;
    
    let config_path = temp_dir.path().join(".klint.yml");
    fs::write(&config_path, config_content).expect("Failed to write config file");
    
    // Run klint with --fix flag
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--fix")
        .arg("test.kql")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fixed 2 violations"));
    
    // Verify file was modified
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // Should have converted table names to PascalCase
    assert!(fixed_content.contains(".create table UserProfiles"), "Expected UserProfiles in fixed content");
    assert!(fixed_content.contains(".create table OrderItems"), "Expected OrderItems in fixed content");
    
    // Should also fix references in the query
    assert!(fixed_content.contains("UserProfiles"), "Expected UserProfiles reference in query");
    assert!(fixed_content.contains("OrderItems"), "Expected OrderItems reference in query");
    
    // Should not contain old names
    assert!(!fixed_content.contains("user_profiles"), "Should not contain old user_profiles name");
    assert!(!fixed_content.contains("order_items"), "Should not contain old order_items name");
}

#[test]
fn test_fix_mode_with_dry_run() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with naming violations  
    let kql_content = r#".create table test_data (id: int, value: string)"#;
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    let original_content = fs::read_to_string(&kql_path).expect("Failed to read original file");
    
    // Run klint with check mode (no --fix flag)
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to violations
        .stdout(predicate::str::contains("test_data")); // Should show violations
    
    // Verify file was NOT modified
    let unchanged_content = fs::read_to_string(&kql_path).expect("Failed to read unchanged file");
    assert_eq!(original_content, unchanged_content, "File should not be modified in check mode");
}

#[test]
fn test_fix_mode_handles_excluded_tables() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with naming violations
    let kql_content = r#".create table temp_data (id: int, value: string)
.create table user_data (id: int, name: string)"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config that excludes temp_* tables
    let config_content = r#"
rules:
  table_naming: PascalCase
  excluded_tables: ["temp_*"]
output:
  format: terminal
"#;
    
    let config_path = temp_dir.path().join(".klint.yml");
    fs::write(&config_path, config_content).expect("Failed to write config file");
    
    // Run klint with --fix flag
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--fix")
        .arg("test.kql")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fixed 1 violations")); // Only user_data should be fixed
    
    // Verify file was modified correctly
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // temp_data should remain unchanged (excluded)
    assert!(fixed_content.contains("temp_data"), "Expected temp_data to remain unchanged");
    
    // user_data should be fixed to UserData  
    assert!(fixed_content.contains("UserData"), "Expected UserData in fixed content");
    assert!(!fixed_content.contains("user_data"), "Should not contain old user_data name");
}

#[test]
fn test_fix_mode_preserves_non_table_content() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with various content types
    let kql_content = r#"// This is a comment about user_profiles
.create table user_data (
    id: int,
    name: string,
    email: string
)

// Query that references the table
let results = user_data
| where name contains "user_data" // This string should not be changed
| project id, name

// Another comment mentioning user_data in documentation
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint with --fix flag
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--fix")
        .arg("test.kql");
        
    // Don't assert on success/failure, just check the content
    let _ = cmd.output();
    
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // Comments should remain unchanged
    assert!(fixed_content.contains("// This is a comment about user_profiles"), 
        "Comments should not be modified");
    assert!(fixed_content.contains("// Another comment mentioning user_data"), 
        "Comments should not be modified");
    
    // String literals should remain unchanged
    assert!(fixed_content.contains(r#"name contains "user_data""#), 
        "String literals should not be modified");
    
    // Table definition and reference should be fixed
    assert!(fixed_content.contains(".create table UserData"), "Table definition should be fixed");
    assert!(fixed_content.contains("let results = UserData"), "Table reference should be fixed");
}