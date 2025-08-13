use klint::parser::{Parser, StatementType};

#[test]
fn test_parse_create_table_statements() {
    let kql_content = r#"
// Sample KQL file
.create table UserLogs (
    Level: string,
    Timestamp: datetime,
    UserId: string
)

.create table error-logs (
    ErrorCode: int,
    ErrorMessage: string
)

// Comment
.create table SYSTEM_METRICS (
    MetricName: string,
    Value: real
)
"#;

    let parser = Parser::new(kql_content.to_string());
    let statements = parser.parse().expect("Failed to parse KQL");
    
    assert_eq!(statements.len(), 3, "Expected 3 table statements but got {}", statements.len());
    
    // Check first table
    let first = &statements[0];
    assert_eq!(first.table_name, "UserLogs");
    assert_eq!(first.statement_type, StatementType::CreateTable);
    assert_eq!(first.line, 3); // Line number should match
    
    // Check second table  
    let second = &statements[1];
    assert_eq!(second.table_name, "error-logs");
    assert_eq!(second.statement_type, StatementType::CreateTable);
    
    // Check third table
    let third = &statements[2];
    assert_eq!(third.table_name, "SYSTEM_METRICS");
    assert_eq!(third.statement_type, StatementType::CreateTable);
}