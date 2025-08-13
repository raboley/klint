//! Inline disable comments parser and handler
//!
//! This module handles parsing and applying inline disable comments in KQL files.
//! Supports patterns like:
//! - `// klint-disable table-naming`
//! - `-- klint-disable table-naming`  
//! - `// klint-disable-next-line table-naming`
//! - `// klint-disable` (disable all rules)
//! - `//nolint:klint` (Go-style, disable all rules)
//! - `//nolint:table-naming` (Go-style, specific rule)

use std::collections::HashSet;
use tracing::debug;

/// Types of disable directives
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisableType {
    /// Disable rules for the current line
    CurrentLine,
    /// Disable rules for the next line
    NextLine,
}

/// A parsed disable comment directive
#[derive(Debug, Clone)]
pub struct DisableDirective {
    pub line: usize,
    pub disable_type: DisableType,
    pub rules: Option<HashSet<String>>, // None means disable all rules
    pub reason: Option<String>,
}

/// Parse disable comments from KQL content
/// 
/// # Arguments
/// 
/// * `content` - The KQL file content to parse
/// 
/// # Returns
/// 
/// A vector of disable directives found in the content
pub fn parse_disable_comments(content: &str) -> Vec<DisableDirective> {
    let mut directives = Vec::new();
    
    for (line_idx, line) in content.lines().enumerate() {
        let line_num = line_idx + 1;
        
        // Check for disable comments in both // and -- style
        if let Some(directive) = parse_line_for_disable(line, line_num) {
            directives.push(directive);
        }
    }
    
    debug!("Found {} disable directives", directives.len());
    directives
}

/// Parse a single line for disable comments
fn parse_line_for_disable(line: &str, line_num: usize) -> Option<DisableDirective> {
    // Look for both // and -- comment styles
    let comment_start = if let Some(pos) = line.find("//") {
        pos
    } else if let Some(pos) = line.find("--") {
        pos
    } else {
        return None;
    };
    
    let comment = &line[comment_start + 2..].trim();
    
    // Check for klint-disable patterns
    if let Some((disable_part, is_next_line)) = extract_klint_disable(comment) {
        return parse_disable_directive(disable_part, line_num, is_next_line);
    }
    
    // Check for Go-style nolint patterns
    if let Some(nolint_part) = extract_nolint(comment) {
        return parse_nolint_directive(nolint_part, line_num);
    }
    
    None
}

/// Extract klint-disable portion from comment
fn extract_klint_disable(comment: &str) -> Option<(&str, bool)> {
    let comment = comment.trim();
    
    if let Some(stripped) = comment.strip_prefix("klint-disable-next-line") {
        Some((stripped, true))
    } else {
        comment.strip_prefix("klint-disable").map(|stripped| (stripped, false))
    }
}

/// Extract nolint portion from Go-style comment
fn extract_nolint(comment: &str) -> Option<&str> {
    let comment = comment.trim();
    comment.strip_prefix("nolint:")
}

/// Parse the disable directive from the extracted portion
fn parse_disable_directive(disable_part: &str, line_num: usize, is_next_line: bool) -> Option<DisableDirective> {
    let disable_part = disable_part.trim();
    
    // Determine disable type
    let disable_type = if is_next_line {
        DisableType::NextLine
    } else {
        DisableType::CurrentLine
    };
    
    // Parse rules and reason
    let (rules_part, reason) = if let Some(reason_pos) = disable_part.find("--") {
        let rules = disable_part[..reason_pos].trim();
        let reason_text = disable_part[reason_pos + 2..].trim();
        (rules, if reason_text.is_empty() { None } else { Some(reason_text.to_string()) })
    } else {
        (disable_part.trim(), None)
    };
    
    // Clean up rules part (remove any extra whitespace)
    let rules_part = rules_part.trim();
    
    // Parse specific rules
    let rules = if rules_part.is_empty() {
        None // Disable all rules
    } else {
        let rule_set: HashSet<String> = rules_part
            .split(',')
            .map(|rule| rule.trim().to_string())
            .filter(|rule| !rule.is_empty())
            .collect();
        
        if rule_set.is_empty() {
            None
        } else {
            Some(rule_set)
        }
    };
    
    debug!("Parsed disable directive on line {}: {:?} rules, {:?} type", 
           line_num, rules, disable_type);
    
    Some(DisableDirective {
        line: line_num,
        disable_type,
        rules,
        reason,
    })
}

/// Parse Go-style nolint directive
fn parse_nolint_directive(nolint_part: &str, line_num: usize) -> Option<DisableDirective> {
    let nolint_part = nolint_part.trim();
    
    // Parse rules - nolint is always current line only
    let rules = if nolint_part.is_empty() || nolint_part == "klint" {
        None // No rules specified or "nolint:klint" - disable all rules
    } else {
        // Parse comma-separated rules
        let rule_set: HashSet<String> = nolint_part
            .split(',')
            .map(|rule| rule.trim().to_string())
            .filter(|rule| !rule.is_empty())
            .collect();
        
        if rule_set.is_empty() {
            None
        } else {
            Some(rule_set)
        }
    };
    
    debug!("Parsed nolint directive on line {}: {:?} rules", line_num, rules);
    
    Some(DisableDirective {
        line: line_num,
        disable_type: DisableType::CurrentLine,
        rules,
        reason: None, // nolint doesn't support reasons
    })
}

/// Check if a rule should be disabled for a given line
/// 
/// # Arguments
/// 
/// * `directives` - All disable directives in the file
/// * `line` - The line number to check
/// * `rule` - The rule name to check (e.g., "table-naming")
/// 
/// # Returns
/// 
/// True if the rule is disabled for the given line
pub fn is_rule_disabled(directives: &[DisableDirective], line: usize, rule: &str) -> bool {
    for directive in directives {
        let applies_to_line = match directive.disable_type {
            DisableType::CurrentLine => directive.line == line,
            DisableType::NextLine => directive.line + 1 == line,
        };
        
        if applies_to_line {
            // Check if this directive disables the specific rule or all rules
            match &directive.rules {
                None => {
                    debug!("Rule '{}' disabled on line {} (all rules disabled)", rule, line);
                    return true; // All rules disabled
                }
                Some(rules) => {
                    if rules.contains(rule) {
                        debug!("Rule '{}' disabled on line {} (specific rule disabled)", rule, line);
                        return true;
                    }
                }
            }
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_current_line_disable() {
        let content = ".create table user_data (id: int) // klint-disable table-naming";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
        assert_eq!(directives[0].disable_type, DisableType::CurrentLine);
        assert_eq!(directives[0].rules, Some(["table-naming".to_string()].into_iter().collect()));
    }

    #[test]
    fn test_parse_next_line_disable() {
        let content = "// klint-disable-next-line table-naming\n.create table user_data (id: int)";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
        assert_eq!(directives[0].disable_type, DisableType::NextLine);
        assert_eq!(directives[0].rules, Some(["table-naming".to_string()].into_iter().collect()));
    }

    #[test]
    fn test_parse_disable_all() {
        let content = ".create table user_data (id: int) // klint-disable";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].rules, None); // All rules disabled
    }

    #[test]
    fn test_parse_disable_with_reason() {
        let content = ".create table user_data (id: int) // klint-disable table-naming -- Legacy compatibility";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].reason, Some("Legacy compatibility".to_string()));
    }

    #[test]
    fn test_parse_sql_style_comment() {
        let content = ".create table user_data (id: int) -- klint-disable table-naming";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].disable_type, DisableType::CurrentLine);
    }

    #[test]
    fn test_parse_go_style_nolint_all() {
        let content = ".create table user_data (id: int) //nolint:klint";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
        assert_eq!(directives[0].disable_type, DisableType::CurrentLine);
        assert_eq!(directives[0].rules, None); // All rules disabled
    }

    #[test]
    fn test_parse_go_style_nolint_specific() {
        let content = ".create table user_data (id: int) //nolint:table-naming";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
        assert_eq!(directives[0].disable_type, DisableType::CurrentLine);
        assert_eq!(directives[0].rules, Some(["table-naming".to_string()].into_iter().collect()));
    }

    #[test]
    fn test_parse_go_style_nolint_multiple() {
        let content = ".create table user_data (id: int) //nolint:table-naming,other-rule";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].rules, Some(["table-naming".to_string(), "other-rule".to_string()].into_iter().collect()));
    }

    #[test]
    fn test_parse_go_style_nolint_empty() {
        let content = ".create table user_data (id: int) //nolint:";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].rules, None); // All rules disabled
    }

    #[test]
    fn test_is_rule_disabled() {
        let directives = vec![
            DisableDirective {
                line: 1,
                disable_type: DisableType::CurrentLine,
                rules: Some(["table-naming".to_string()].into_iter().collect()),
                reason: None,
            },
            DisableDirective {
                line: 3,
                disable_type: DisableType::NextLine,
                rules: None, // All rules
                reason: None,
            },
        ];
        
        // Line 1 - specific rule disabled
        assert!(is_rule_disabled(&directives, 1, "table-naming"));
        assert!(!is_rule_disabled(&directives, 1, "other-rule"));
        
        // Line 4 - all rules disabled by next-line directive on line 3
        assert!(is_rule_disabled(&directives, 4, "table-naming"));
        assert!(is_rule_disabled(&directives, 4, "any-rule"));
        
        // Line 2 - no rules disabled
        assert!(!is_rule_disabled(&directives, 2, "table-naming"));
    }
}