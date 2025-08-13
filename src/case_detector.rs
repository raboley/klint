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
/// True if conversion is possible, false if either case is Unknown
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