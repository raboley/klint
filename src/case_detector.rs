//! Case detection and conversion module for identifying naming conventions

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use tracing::debug;

/// Naming convention cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingCase {
    SnakeCase,
    ScreamingSnakeCase,
    PascalCase,
    CamelCase,
    KebabCase,
    Unknown,
}

impl NamingCase {
    /// Convert to convert_case::Case
    pub fn to_convert_case(&self) -> Option<Case> {
        match self {
            NamingCase::SnakeCase => Some(Case::Snake),
            NamingCase::ScreamingSnakeCase => Some(Case::ScreamingSnake),
            NamingCase::PascalCase => Some(Case::Pascal),
            NamingCase::CamelCase => Some(Case::Camel),
            NamingCase::KebabCase => Some(Case::Kebab),
            NamingCase::Unknown => None,
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "snake_case" | "snake" => NamingCase::SnakeCase,
            "screaming_snake_case" | "screaming_snake" | "upper_snake" => NamingCase::ScreamingSnakeCase,
            "pascalcase" | "pascal" | "uppercamel" => NamingCase::PascalCase,
            "camelcase" | "camel" | "lowercamel" => NamingCase::CamelCase,
            "kebab-case" | "kebab" | "dash" => NamingCase::KebabCase,
            _ => NamingCase::Unknown,
        }
    }

    /// Convert to display string
    pub fn to_string(&self) -> &'static str {
        match self {
            NamingCase::SnakeCase => "snake_case",
            NamingCase::ScreamingSnakeCase => "SCREAMING_SNAKE_CASE",
            NamingCase::PascalCase => "PascalCase",
            NamingCase::CamelCase => "camelCase",
            NamingCase::KebabCase => "kebab-case",
            NamingCase::Unknown => "unknown",
        }
    }
}

/// Detect the naming case of a string
pub fn detect_case(name: &str) -> NamingCase {
    debug!("Detecting case for: {}", name);
    
    // Check for various patterns
    if name.chars().all(|c| c.is_uppercase() || c == '_') && name.contains('_') {
        return NamingCase::ScreamingSnakeCase;
    }
    
    if name.chars().all(|c| c.is_lowercase() || c == '_') && name.contains('_') {
        return NamingCase::SnakeCase;
    }
    
    if name.contains('-') {
        return NamingCase::KebabCase;
    }
    
    if name.chars().next().map_or(false, |c| c.is_uppercase()) {
        // Check if it's PascalCase (has internal uppercase letters)
        if name.chars().skip(1).any(|c| c.is_uppercase()) {
            return NamingCase::PascalCase;
        }
    }
    
    if name.chars().next().map_or(false, |c| c.is_lowercase()) {
        // Check if it's camelCase (has internal uppercase letters)
        if name.chars().skip(1).any(|c| c.is_uppercase()) {
            return NamingCase::CamelCase;
        }
    }
    
    // Single word starting with uppercase
    if name.chars().next().map_or(false, |c| c.is_uppercase()) 
        && name.chars().skip(1).all(|c| c.is_lowercase()) {
        return NamingCase::PascalCase;
    }
    
    // Single word all lowercase
    if name.chars().all(|c| c.is_lowercase()) {
        return NamingCase::SnakeCase;
    }
    
    NamingCase::Unknown
}

/// Convert a string from one case to another
pub fn convert_case(input: &str, _from: NamingCase, to: NamingCase) -> Result<String> {
    // If the target case is unknown, we can't convert
    let target_case = to.to_convert_case()
        .ok_or_else(|| anyhow!("Cannot convert to unknown case"))?;
    
    // Use the convert_case crate for conversion
    Ok(input.to_case(target_case))
}

/// Check if a string can be converted between cases
pub fn can_convert(_input: &str, from: NamingCase, to: NamingCase) -> bool {
    // Can't convert from or to unknown
    if from == NamingCase::Unknown || to == NamingCase::Unknown {
        return false;
    }
    
    // Same case - no conversion needed
    if from == to {
        return true;
    }
    
    // All other conversions are theoretically possible
    true
}