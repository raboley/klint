use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_init_command_creates_default_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join(".klint.yml");
    
    // Change to temp directory
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir);
    
    // Run init command
    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration file created"));
    
    // Verify file was created
    assert!(config_path.exists(), "Expected .klint.yml to be created");
    
    // Verify file contents
    let content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(content.contains("table_naming: PascalCase"), "Expected default table naming convention");
    assert!(content.contains("rules:"), "Expected rules section");
    assert!(content.contains("output:"), "Expected output section");
}

#[test]
fn test_init_command_with_force_overwrites_existing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join(".klint.yml");
    
    // Create existing config
    fs::write(&config_path, "# old config\nrules:\n  table_naming: snake_case").expect("Failed to create existing config");
    
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir);
    
    // Run init with force
    cmd.arg("init").arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration file created"));
    
    // Verify file was overwritten with default content
    let content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(content.contains("table_naming: PascalCase"), "Expected default naming convention after force");
    assert!(!content.contains("# old config"), "Old config should be overwritten");
}

#[test]
fn test_init_command_fails_without_force_when_config_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join(".klint.yml");
    
    // Create existing config
    fs::write(&config_path, "# existing config").expect("Failed to create existing config");
    
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir);
    
    // Run init without force - should fail
    cmd.arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Configuration file already exists"));
}