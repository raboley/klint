//! Inline disable comments parser and handler
//!
//! This module handles parsing and applying inline disable comments in KQL files.
//! Supports patterns like:
//! - `//nolint:` (disable all rules)
//! - `//nolint:table-naming` (disable specific rule)
//! - `//nolint:table-naming,other-rule` (disable multiple rules)

use std::collections::HashSet;
use tracing::debug;

/// A parsed disable comment directive
#[derive(Debug, Clone)]
pub struct DisableDirective {
    pub line: usize,
    pub rules: Option<HashSet<String>>, // None means disable all rules
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
    
    // Check for nolint patterns
    if let Some(nolint_part) = extract_nolint(comment) {
        return parse_nolint_directive(nolint_part, line_num);
    }
    
    None
}

/// Extract nolint portion from comment
fn extract_nolint(comment: &str) -> Option<&str> {
    let comment = comment.trim();
    comment.strip_prefix("nolint:")
}

/// Parse nolint directive
fn parse_nolint_directive(nolint_part: &str, line_num: usize) -> Option<DisableDirective> {
    let nolint_part = nolint_part.trim();
    
    // Parse rules - nolint is always current line only
    let rules = if nolint_part.is_empty() {
        None // No rules specified - disable all rules
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
        rules,
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
        if directive.line == line {
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
    fn test_parse_nolint_all() {
        let content = ".create table user_data (id: int) //nolint:";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
        assert_eq!(directives[0].rules, None); // All rules disabled
    }

    #[test]
    fn test_parse_nolint_specific() {
        let content = ".create table user_data (id: int) //nolint:table-naming";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
        assert_eq!(directives[0].rules, Some(["table-naming".to_string()].into_iter().collect()));
    }

    #[test]
    fn test_parse_nolint_multiple() {
        let content = ".create table user_data (id: int) //nolint:table-naming,other-rule";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].rules, Some(["table-naming".to_string(), "other-rule".to_string()].into_iter().collect()));
    }

    #[test]
    fn test_parse_sql_style_comment() {
        let content = ".create table user_data (id: int) -- nolint:table-naming";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
    }

    #[test]
    fn test_is_rule_disabled() {
        let directives = vec![
            DisableDirective {
                line: 1,
                rules: Some(["table-naming".to_string()].into_iter().collect()),
            },
            DisableDirective {
                line: 3,
                rules: None, // All rules
            },
        ];
        
        // Line 1 - specific rule disabled
        assert!(is_rule_disabled(&directives, 1, "table-naming"));
        assert!(!is_rule_disabled(&directives, 1, "other-rule"));
        
        // Line 3 - all rules disabled
        assert!(is_rule_disabled(&directives, 3, "table-naming"));
        assert!(is_rule_disabled(&directives, 3, "any-rule"));
        
        // Line 2 - no rules disabled
        assert!(!is_rule_disabled(&directives, 2, "table-naming"));
    }

    #[test]
    fn test_no_disable_comments() {
        let content = ".create table user_data (id: int) // regular comment";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 0);
    }

    #[test]
    fn test_multiple_lines() {
        let content = "// First line\n.create table user_data (id: int) //nolint:table-naming\n.create table other_data (id: int) //nolint:";
        let directives = parse_disable_comments(content);
        
        assert_eq!(directives.len(), 2);
        assert_eq!(directives[0].line, 2);
        assert_eq!(directives[1].line, 3);
    }
}