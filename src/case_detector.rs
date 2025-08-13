//! Case detection and conversion module for identifying naming conventions

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::str::FromStr;
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
    /// Convert to convert_case::Case for use with the convert_case crate
    /// 
    /// # Returns
    /// 
    /// Some(Case) for known naming cases, None for Unknown case
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


    /// Convert to display string representation
    /// 
    /// # Returns
    /// 
    /// Static string representation of the naming case
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

/// Detect the naming case of a string by analyzing its patterns
/// 
/// # Arguments
/// 
/// * `name` - The string to analyze
/// 
/// # Returns
/// 
/// The detected NamingCase, or Unknown if the pattern is not recognized
/// 
/// # Examples
/// 
/// ```
/// use klint::case_detector::{detect_case, NamingCase};
/// 
/// assert_eq!(detect_case("user_name"), NamingCase::SnakeCase);
/// assert_eq!(detect_case("USER_NAME"), NamingCase::ScreamingSnakeCase);
/// assert_eq!(detect_case("UserName"), NamingCase::PascalCase);
/// ```
pub fn detect_case(name: &str) -> NamingCase {
    debug!("Detecting case for: {}", name);
    
    // First check for pure patterns (no mixed cases)
    let has_underscores = name.contains('_');
    let has_hyphens = name.contains('-');
    let has_uppercase = name.chars().any(|c| c.is_uppercase());
    let has_lowercase = name.chars().any(|c| c.is_lowercase());
    
    // If we have multiple separators or complex mixing, it's likely Unknown
    if has_underscores && has_hyphens {
        debug!("Mixed separators detected, returning Unknown");
        return NamingCase::Unknown;
    }
    
    // SCREAMING_SNAKE_CASE: all uppercase with underscores, no lowercase
    if has_underscores && has_uppercase && !has_lowercase && name.chars().all(|c| c.is_uppercase() || c == '_') {
        return NamingCase::ScreamingSnakeCase;
    }
    
    // snake_case: all lowercase with underscores, no uppercase
    if has_underscores && has_lowercase && !has_uppercase && name.chars().all(|c| c.is_lowercase() || c == '_') {
        return NamingCase::SnakeCase;
    }
    
    // kebab-case: all lowercase with hyphens, no uppercase or underscores
    if has_hyphens && has_lowercase && !has_uppercase && !has_underscores && name.chars().all(|c| c.is_lowercase() || c == '-') {
        return NamingCase::KebabCase;
    }
    
    // If we have hyphens but mixed case, it's complex - return Unknown
    if has_hyphens && (has_uppercase || has_underscores) {
        debug!("Complex mixed case with hyphens detected, returning Unknown");
        return NamingCase::Unknown;
    }
    
    // If we have underscores with mixed case (not pure snake/screaming), it's complex
    if has_underscores && has_uppercase && has_lowercase {
        debug!("Complex mixed case with underscores detected, returning Unknown");
        return NamingCase::Unknown;
    }
    
    // PascalCase: starts with uppercase, has more uppercase, no separators
    if !has_underscores && !has_hyphens && name.chars().next().map_or(false, |c| c.is_uppercase()) && name.chars().skip(1).any(|c| c.is_uppercase()) {
        return NamingCase::PascalCase;
    }
    
    // camelCase: starts with lowercase, has uppercase, no separators
    if !has_underscores && !has_hyphens && name.chars().next().map_or(false, |c| c.is_lowercase()) && name.chars().skip(1).any(|c| c.is_uppercase()) {
        return NamingCase::CamelCase;
    }
    
    // Single word starting with uppercase (no separators, no internal uppercase)
    if !has_underscores && !has_hyphens && name.chars().next().map_or(false, |c| c.is_uppercase()) 
        && name.chars().skip(1).all(|c| c.is_lowercase()) {
        return NamingCase::PascalCase;
    }
    
    // Single word all lowercase (no separators)
    if !has_underscores && !has_hyphens && name.chars().all(|c| c.is_lowercase()) {
        return NamingCase::SnakeCase;
    }
    
    // Anything else is Unknown
    debug!("No clear pattern detected, returning Unknown");
    NamingCase::Unknown
}

impl FromStr for NamingCase {
    type Err = anyhow::Error;

    /// Parse a naming case from a string representation
    /// 
    /// # Arguments
    /// 
    /// * `s` - String representation of the naming case
    /// 
    /// # Returns
    /// 
    /// The corresponding NamingCase, or an error if not recognized
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::str::FromStr;
    /// use klint::case_detector::NamingCase;
    /// 
    /// let case: NamingCase = "snake_case".parse().unwrap();
    /// assert_eq!(case, NamingCase::SnakeCase);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "snake_case" | "snake" => Ok(NamingCase::SnakeCase),
            "screaming_snake_case" | "screaming_snake" | "upper_snake" => Ok(NamingCase::ScreamingSnakeCase),
            "pascalcase" | "pascal" | "uppercamel" => Ok(NamingCase::PascalCase),
            "camelcase" | "camel" | "lowercamel" => Ok(NamingCase::CamelCase),
            "kebab-case" | "kebab" | "dash" => Ok(NamingCase::KebabCase),
            _ => Err(anyhow!("Unknown naming case: {}", s)),
        }
    }
}

/// Convert a string from one naming case to another
/// 
/// # Arguments
/// 
/// * `input` - The string to convert
/// * `_from` - The source naming case (currently unused)
/// * `to` - The target naming case
/// 
/// # Returns
/// 
/// The converted string, or an error if conversion is not possible
/// 
/// # Examples
/// 
/// ```
/// use klint::case_detector::{convert_case, NamingCase};
/// 
/// let result = convert_case("user_name", NamingCase::SnakeCase, NamingCase::PascalCase).unwrap();
/// assert_eq!(result, "UserName");
/// ```
pub fn convert_case(input: &str, _from: NamingCase, to: NamingCase) -> Result<String> {
    // If the target case is unknown, we can't convert
    let target_case = to.to_convert_case()
        .ok_or_else(|| anyhow!("Cannot convert to unknown case"))?;
    
    // Use the convert_case crate for conversion
    Ok(input.to_case(target_case))
}

/// Check if a string can be converted between naming cases
/// 
/// # Arguments
/// 
/// * `_input` - The string to check (currently unused)
/// * `from` - The source naming case
/// * `to` - The target naming case
/// 
/// # Returns
/// 
/// True if conversion is possible, false if target case is Unknown
pub fn can_convert(_input: &str, from: NamingCase, to: NamingCase) -> bool {
    // Can't convert to unknown, but can convert from unknown
    if to == NamingCase::Unknown {
        return false;
    }
    
    // Same case - no conversion needed
    if from == to {
        return true;
    }
    
    // Can convert from Unknown to any known case, and between known cases
    true
}