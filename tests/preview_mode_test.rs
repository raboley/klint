use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_preview_mode_shows_proposed_fixes() {
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

let results = user_profiles
| join kind=inner order_items on user_id
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
    
    // Run klint without --fix flag (check mode with preview)
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--preview")
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to violations
        .stdout(predicate::str::contains("user_profiles → UserProfiles"))
        .stdout(predicate::str::contains("order_items → OrderItems"));
    
    // Verify file was NOT modified
    let unchanged_content = fs::read_to_string(&kql_path).expect("Failed to read file");
    assert_eq!(kql_content, unchanged_content, "File should not be modified in preview mode");
}

#[test]
fn test_preview_mode_with_json_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with naming violations  
    let kql_content = r#".create table test_data (id: int, value: string)"#;
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with JSON output
    let config_content = r#"
rules:
  table_naming: PascalCase
output:
  format: json
"#;
    
    let config_path = temp_dir.path().join(".klint.yml");
    fs::write(&config_path, config_content).expect("Failed to write config file");
    
    // Run klint with preview and JSON output
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--preview")
        .arg("--output")
        .arg("json")
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to violations
        .stdout(predicate::str::contains("\"preview\":"))
        .stdout(predicate::str::contains("\"original\": \"test_data\""))
        .stdout(predicate::str::contains("\"fixed\": \"TestData\""));
}

#[test]
fn test_preview_mode_shows_no_fixes_when_compliant() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file that's already compliant
    let kql_content = r#".create table UserProfiles (id: int, name: string)"#;
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint with preview mode
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--preview")
        .arg("test.kql")
        .assert()
        .success() // Should succeed - no violations
        .stdout(predicate::str::contains("No fixes needed"));
}

#[test]
fn test_preview_mode_respects_exclusions() {
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
    
    // Run klint with preview mode
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--preview")
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to user_data violation
        .stdout(predicate::str::contains("user_data → UserData"))
        .stdout(predicate::str::contains("temp_data").not()); // temp_data should not appear in preview
}