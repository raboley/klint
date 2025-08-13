//! Case detection and conversion module for identifying naming conventions

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::convert::TryFrom;
use std::fmt;
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


}

impl fmt::Display for NamingCase {
    /// Display the naming case as its conventional string representation
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let case_name = match self {
            NamingCase::SnakeCase => "snake_case",
            NamingCase::ScreamingSnakeCase => "SCREAMING_SNAKE_CASE",
            NamingCase::PascalCase => "PascalCase",
            NamingCase::CamelCase => "camelCase",
            NamingCase::KebabCase => "kebab-case",
            NamingCase::Unknown => "unknown",
        };
        write!(f, "{}", case_name)
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
        let normalized = s.trim().to_lowercase();
        match normalized.as_str() {
            "snake_case" | "snake" => Ok(NamingCase::SnakeCase),
            "screaming_snake_case" | "screaming_snake" | "upper_snake" => Ok(NamingCase::ScreamingSnakeCase),
            "pascalcase" | "pascal" | "uppercamel" => Ok(NamingCase::PascalCase),
            "camelcase" | "camel" | "lowercamel" => Ok(NamingCase::CamelCase),
            "kebab-case" | "kebab" | "dash" => Ok(NamingCase::KebabCase),
            _ => {
                let valid_options = [
                    "snake_case", "SCREAMING_SNAKE_CASE", "PascalCase", 
                    "camelCase", "kebab-case"
                ];
                let suggestion = find_closest_match(&normalized, &valid_options);
                match suggestion {
                    Some(closest) => Err(anyhow!(
                        "Unknown naming case '{}'. Did you mean '{}'? Valid options are: {}",
                        s, closest, valid_options.join(", ")
                    )),
                    None => Err(anyhow!(
                        "Unknown naming case '{}'. Valid options are: {}",
                        s, valid_options.join(", ")
                    ))
                }
            }
        }
    }
}

impl TryFrom<&str> for NamingCase {
    type Error = anyhow::Error;

    /// Try to convert from a string slice to NamingCase
    /// 
    /// This provides a more idiomatic way to convert strings to NamingCase
    /// using the TryFrom trait pattern for fallible conversions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::convert::TryFrom;
    /// use klint::case_detector::NamingCase;
    /// 
    /// let case = NamingCase::try_from("snake_case").unwrap();
    /// assert_eq!(case, NamingCase::SnakeCase);
    /// ```
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for NamingCase {
    type Error = anyhow::Error;

    /// Try to convert from a String to NamingCase
    /// 
    /// This provides a more idiomatic way to convert owned strings to NamingCase
    /// using the TryFrom trait pattern for fallible conversions.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl From<NamingCase> for Option<Case> {
    /// Convert NamingCase to convert_case::Case for use with the convert_case crate
    /// 
    /// Returns None for Unknown case since it cannot be converted.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use klint::case_detector::NamingCase;
    /// use convert_case::Case;
    /// 
    /// let case: Option<Case> = NamingCase::SnakeCase.into();
    /// assert_eq!(case, Some(Case::Snake));
    /// 
    /// let unknown: Option<Case> = NamingCase::Unknown.into();
    /// assert_eq!(unknown, None);
    /// ```
    fn from(naming_case: NamingCase) -> Self {
        naming_case.to_convert_case()
    }
}

impl From<Case> for NamingCase {
    /// Convert from convert_case::Case to NamingCase
    /// 
    /// This provides a way to convert from the external convert_case crate
    /// types back to our internal NamingCase enum.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use klint::case_detector::NamingCase;
    /// use convert_case::Case;
    /// 
    /// let naming_case: NamingCase = Case::Snake.into();
    /// assert_eq!(naming_case, NamingCase::SnakeCase);
    /// ```
    fn from(case: Case) -> Self {
        match case {
            Case::Snake => NamingCase::SnakeCase,
            Case::ScreamingSnake => NamingCase::ScreamingSnakeCase,
            Case::Pascal => NamingCase::PascalCase,
            Case::Camel => NamingCase::CamelCase,
            Case::Kebab => NamingCase::KebabCase,
            // For any other cases that convert_case might have, map to Unknown
            _ => NamingCase::Unknown,
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
    let target_case: Option<Case> = to.into();
    let target_case = target_case
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

/// Find the closest match using a simple string distance algorithm
/// 
/// Uses a basic edit distance algorithm to find the most similar string
/// from a list of options.
fn find_closest_match<'a>(input: &str, options: &'a [&'a str]) -> Option<&'a str> {
    let input_lower = input.to_lowercase();
    
    // First try exact match ignoring case
    for option in options {
        if option.to_lowercase() == input_lower {
            return Some(option);
        }
    }
    
    // Then try substring match
    for option in options {
        let option_lower = option.to_lowercase();
        if option_lower.contains(&input_lower) || input_lower.contains(&option_lower) {
            return Some(option);
        }
    }
    
    // Finally try Levenshtein distance for close matches
    let mut best_distance = usize::MAX;
    let mut best_match = None;
    
    for option in options {
        let distance = levenshtein_distance(&input_lower, &option.to_lowercase());
        if distance < best_distance && distance <= 3 { // Only suggest if distance is reasonable
            best_distance = distance;
            best_match = Some(*option);
        }
    }
    
    best_match
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    // Initialize first row and column
    for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
        row[0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,      // deletion
                    matrix[i][j - 1] + 1       // insertion
                ),
                matrix[i - 1][j - 1] + cost    // substitution
            );
        }
    }
    
    matrix[len1][len2]
}