use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_disable_current_line_with_double_slash() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with inline disable comment
    let kql_content = r#".create table user_data (id: int, name: string) // klint-disable table-naming
.create table another_bad_name (id: int, value: string)
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
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("user_data").not()); // user_data should be ignored
}

#[test]
fn test_disable_current_line_with_double_dash() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with SQL-style inline disable comment
    let kql_content = r#".create table user_data (id: int, name: string) -- klint-disable table-naming
.create table another_bad_name (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("user_data").not()); // user_data should be ignored
}

#[test]
fn test_disable_next_line() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with next-line disable comment
    let kql_content = r#"// klint-disable-next-line table-naming
.create table user_data (id: int, name: string)

.create table another_bad_name (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("user_data").not()); // user_data should be ignored
}

#[test]
fn test_disable_all_rules() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with disable-all comment
    let kql_content = r#".create table user_data (id: int, name: string) // klint-disable
.create table another_bad_name (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("user_data").not()); // user_data should be ignored
}

#[test]
fn test_disable_with_reason() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with disable comment and reason
    let kql_content = r#".create table temp_user_data (id: int, name: string) // klint-disable table-naming -- Legacy table name required for compatibility
.create table another_bad_name (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("temp_user_data").not()); // temp_user_data should be ignored
}

#[test]
fn test_disable_specific_rule_only() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with specific rule disable
    let kql_content = r#".create table user_data (id: int, name: string) // klint-disable some-other-rule
.create table another_data (id: int, value: string) // klint-disable table-naming
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to first table
        .stdout(predicate::str::contains("user_data")) // user_data should still be flagged
        .stdout(predicate::str::contains("another_data").not()); // another_data should be ignored
}

#[test]
fn test_disable_does_not_affect_fix_mode() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with mixed disable comments
    let kql_content = r#".create table user_data (id: int, name: string) // klint-disable table-naming
.create table order_data (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint with --fix flag
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--fix")
        .arg("test.kql")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fixed 1 violations")); // Only order_data should be fixed
    
    // Verify file content
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // user_data should remain unchanged (disabled)
    assert!(fixed_content.contains("user_data"), "user_data should remain unchanged");
    
    // order_data should be fixed to OrderData
    assert!(fixed_content.contains("OrderData"), "order_data should be fixed to OrderData");
    assert!(!fixed_content.contains("order_data"), "old order_data name should be replaced");
}

#[test]
fn test_disable_in_preview_mode() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with disable comment
    let kql_content = r#".create table user_data (id: int, name: string) // klint-disable table-naming
.create table order_data (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint with --preview flag
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--preview")
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to order_data
        .stdout(predicate::str::contains("order_data → OrderData")) // Should show preview for order_data
        .stdout(predicate::str::contains("user_data").not()); // Should not show preview for disabled user_data
}

#[test]
fn test_go_style_nolint_current_line() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with Go-style nolint comment
    let kql_content = r#".create table user_data (id: int, name: string) //nolint:klint
.create table another_bad_name (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("user_data").not()); // user_data should be ignored
}

#[test]
fn test_go_style_nolint_specific_rule() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with Go-style nolint for specific rule
    let kql_content = r#".create table user_data (id: int, name: string) //nolint:table-naming
.create table another_bad_name (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("user_data").not()); // user_data should be ignored
}

#[test]
fn test_go_style_nolint_multiple_rules() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with Go-style nolint for multiple rules
    let kql_content = r#".create table user_data (id: int, name: string) //nolint:table-naming,other-rule
.create table another_bad_name (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to second table only
        .stdout(predicate::str::contains("another_bad_name"))
        .stdout(predicate::str::contains("user_data").not()); // user_data should be ignored
}

#[test]
fn test_mixed_comment_styles() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with mixed comment styles
    let kql_content = r#".create table user_data (id: int, name: string) // klint-disable table-naming
.create table temp_data (id: int, value: string) //nolint:table-naming
.create table order_data (id: int, item: string) -- klint-disable table-naming
.create table should_fail (id: int, value: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Run klint
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to should_fail table only
        .stdout(predicate::str::contains("should_fail"))
        .stdout(predicate::str::contains("user_data").not())
        .stdout(predicate::str::contains("temp_data").not())
        .stdout(predicate::str::contains("order_data").not()); // All disabled tables should be ignored
}