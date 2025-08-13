//! Linting rule definitions

use crate::case_detector::NamingCase;

/// Severity level for violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// A linting rule
pub trait Rule {
    /// Name of the rule
    fn name(&self) -> &str;
    
    /// Description of the rule
    fn description(&self) -> &str;
    
    /// Severity of violations
    fn severity(&self) -> Severity;
}

/// Table naming convention rule
pub struct TableNamingRule {
    expected_case: NamingCase,
    severity: Severity,
}

impl TableNamingRule {
    /// Create a new table naming rule with the specified case
    pub fn new(expected_case: NamingCase) -> Self {
        Self {
            expected_case,
            severity: Severity::Error,
        }
    }
    
    /// Get the expected naming case
    pub fn expected_case(&self) -> NamingCase {
        self.expected_case
    }
}

impl Rule for TableNamingRule {
    fn name(&self) -> &str {
        "table-naming"
    }
    
    fn description(&self) -> &str {
        "Enforces consistent table naming convention"
    }
    
    fn severity(&self) -> Severity {
        self.severity
    }
}