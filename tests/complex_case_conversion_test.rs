use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_complex_mixed_cases_to_kebab_case() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with complex mixed-case table names
    let kql_content = r#".create table L4_rule_Approved-change (id: int, name: string)
.create table API_V2_UserData-Processing_Queue (id: int, value: string)  
.create table LEGACY_User_Account-Management_System (id: int, status: string)
.create table Mixed_CamelCase_snake_case-kebab-UPPER (id: int, data: string)
.create table HTMLParser_XMLProcessor-JSONBuilder_APIClient (id: int, content: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with kebab-case naming
    let config_content = r#"
rules:
  table_naming: kebab-case
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
        .stdout(predicate::str::contains("Fixed 5 violations"));
    
    // Verify file content was converted properly
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // Check each conversion (using actual convert_case crate behavior)
    assert!(fixed_content.contains("l-4-rule-approved-change"), "L4_rule_Approved-change should convert to l-4-rule-approved-change");
    assert!(fixed_content.contains("api-v-2-user-data-processing-queue"), "API_V2_UserData-Processing_Queue should convert to api-v-2-user-data-processing-queue");  
    assert!(fixed_content.contains("legacy-user-account-management-system"), "LEGACY_User_Account-Management_System should convert to legacy-user-account-management-system");
    assert!(fixed_content.contains("mixed-camel-case-snake-case-kebab-upper"), "Mixed_CamelCase_snake_case-kebab-UPPER should convert to mixed-camel-case-snake-case-kebab-upper");
    assert!(fixed_content.contains("html-parser-xml-processor-json-builder-api-client"), "HTMLParser_XMLProcessor-JSONBuilder_APIClient should convert to html-parser-xml-processor-json-builder-api-client");
    
    // Ensure original names are gone
    assert!(!fixed_content.contains("L4_rule_Approved-change"));
    assert!(!fixed_content.contains("API_V2_UserData-Processing_Queue"));
    assert!(!fixed_content.contains("LEGACY_User_Account-Management_System"));
    assert!(!fixed_content.contains("Mixed_CamelCase_snake_case-kebab-UPPER"));
    assert!(!fixed_content.contains("HTMLParser_XMLProcessor-JSONBuilder_APIClient"));
}

#[test]
fn test_complex_mixed_cases_to_snake_case() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with complex mixed-case table names
    let kql_content = r#".create table HTTPSConnection-Manager_V2 (id: int, name: string)
.create table OAuth2-Token_Refresh-Handler (id: int, token: string)
.create table WebAPI_Request-Response_Logger-Service (id: int, log: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with snake_case naming
    let config_content = r#"
rules:
  table_naming: snake_case
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
        .stdout(predicate::str::contains("Fixed 3 violations"));
    
    // Verify file content was converted properly
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // Check each conversion (using actual convert_case crate behavior)
    assert!(fixed_content.contains("https_connection_manager_v_2"), "HTTPSConnection-Manager_V2 should convert to https_connection_manager_v_2");
    assert!(fixed_content.contains("o_auth_2_token_refresh_handler"), "OAuth2-Token_Refresh-Handler should convert to o_auth_2_token_refresh_handler");
    assert!(fixed_content.contains("web_api_request_response_logger_service"), "WebAPI_Request-Response_Logger-Service should convert to web_api_request_response_logger_service");
}

#[test]
fn test_complex_mixed_cases_to_pascal_case() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with complex mixed-case table names
    let kql_content = r#".create table json_web_token-VALIDATOR_Service (id: int, name: string)
.create table database_CONNECTION-pool_MANAGER-v3 (id: int, pool_size: int)
.create table FILE_SYSTEM-watch_SERVICE-background_PROCESSOR (id: int, path: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with PascalCase naming (default)
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
        .stdout(predicate::str::contains("Fixed 3 violations"));
    
    // Verify file content was converted properly
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // Check each conversion
    assert!(fixed_content.contains("JsonWebTokenValidatorService"), "json_web_token-VALIDATOR_Service should convert to JsonWebTokenValidatorService");
    assert!(fixed_content.contains("DatabaseConnectionPoolManagerV3"), "database_CONNECTION-pool_MANAGER-v3 should convert to DatabaseConnectionPoolManagerV3");
    assert!(fixed_content.contains("FileSystemWatchServiceBackgroundProcessor"), "FILE_SYSTEM-watch_SERVICE-background_PROCESSOR should convert to FileSystemWatchServiceBackgroundProcessor");
}

#[test]
fn test_complex_mixed_cases_to_camel_case() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with complex mixed-case table names  
    let kql_content = r#".create table XML_HTTP-request_HANDLER-service (id: int, name: string)
.create table REDIS_cache_KEY-value_STORE-manager (id: int, key: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with camelCase naming
    let config_content = r#"
rules:
  table_naming: camelCase
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
    
    // Verify file content was converted properly
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // Check each conversion
    assert!(fixed_content.contains("xmlHttpRequestHandlerService"), "XML_HTTP-request_HANDLER-service should convert to xmlHttpRequestHandlerService");
    assert!(fixed_content.contains("redisCacheKeyValueStoreManager"), "REDIS_cache_KEY-value_STORE-manager should convert to redisCacheKeyValueStoreManager");
}

#[test]
fn test_complex_mixed_cases_to_screaming_snake_case() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with complex mixed-case table names
    let kql_content = r#".create table restAPI_client-CONNECTION_manager (id: int, name: string)
.create table eventLoop-PROCESSOR_service-QUEUE (id: int, event: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with SCREAMING_SNAKE_CASE naming
    let config_content = r#"
rules:
  table_naming: SCREAMING_SNAKE_CASE
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
    
    // Verify file content was converted properly
    let fixed_content = fs::read_to_string(&kql_path).expect("Failed to read fixed file");
    
    // Check each conversion
    assert!(fixed_content.contains("REST_API_CLIENT_CONNECTION_MANAGER"), "restAPI_client-CONNECTION_manager should convert to REST_API_CLIENT_CONNECTION_MANAGER");
    assert!(fixed_content.contains("EVENT_LOOP_PROCESSOR_SERVICE_QUEUE"), "eventLoop-PROCESSOR_service-QUEUE should convert to EVENT_LOOP_PROCESSOR_SERVICE_QUEUE");
}

#[test]
fn test_preview_mode_with_complex_cases() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test KQL file with very complex case
    let kql_content = r#".create table L4_rule_Approved-change_FINAL-v2_XMLParser (id: int, name: string)
"#;
    
    let kql_path = temp_dir.path().join("test.kql");
    fs::write(&kql_path, kql_content).expect("Failed to write test file");
    
    // Create config with kebab-case naming
    let config_content = r#"
rules:
  table_naming: kebab-case
output:
  format: terminal
"#;
    
    let config_path = temp_dir.path().join(".klint.yml");
    fs::write(&config_path, config_content).expect("Failed to write config file");
    
    // Run klint with --preview flag
    let mut cmd = Command::cargo_bin("klint").expect("Failed to find klint binary");
    cmd.current_dir(&temp_dir)
        .arg("--preview")
        .arg("test.kql")
        .assert()
        .failure() // Should fail due to violation
        .stdout(predicate::str::contains("L4_rule_Approved-change_FINAL-v2_XMLParser → l-4-rule-approved-change-final-v-2-xml-parser"));
}