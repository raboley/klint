//! Auto-fix implementation for violations

use anyhow::Result;
use tracing::{debug, info};

use crate::case_detector::convert_case;
use crate::linter::{LintResult, Violation};

/// Fixer for automatically correcting violations
pub struct Fixer;

impl Fixer {
    /// Fix all violations in a lint result
    pub fn fix(lint_result: &LintResult) -> Result<String> {
        let mut content = lint_result.content.clone();
        
        // Sort violations by position (reverse order to maintain positions)
        let mut violations = lint_result.violations.clone();
        violations.sort_by(|a, b| b.line.cmp(&a.line).then(b.column.cmp(&a.column)));
        
        for violation in violations {
            content = Self::fix_violation(&content, &violation)?;
        }
        
        info!("Fixed {} violations", lint_result.violations.len());
        Ok(content)
    }

    /// Fix a single violation
    fn fix_violation(content: &str, violation: &Violation) -> Result<String> {
        debug!(
            "Fixing table '{}' from {} to {}",
            violation.table_name,
            violation.current_case.to_string(),
            violation.expected_case.to_string()
        );
        
        let fixed_name = convert_case(
            &violation.table_name,
            violation.current_case,
            violation.expected_case,
        )?;
        
        // Use smart replacement to avoid changing comments and strings
        let fixed_content = Self::smart_replace(content, &violation.table_name, &fixed_name);
        
        Ok(fixed_content)
    }

    /// Smart replacement that avoids comments and quoted strings
    fn smart_replace(content: &str, old_name: &str, new_name: &str) -> String {
        let mut result = String::new();
        let mut chars = content.chars().peekable();
        let mut in_string = false;
        let mut in_comment = false;
        let mut in_single_line_comment = false;
        let mut current_word = String::new();
        let mut quote_char = '\0';
        
        while let Some(ch) = chars.next() {
            // Handle string literals
            if !in_comment && !in_single_line_comment {
                if (ch == '"' || ch == '\'') && !in_string {
                    in_string = true;
                    quote_char = ch;
                    result.push_str(&Self::process_word(&current_word, old_name, new_name));
                    current_word.clear();
                    result.push(ch);
                    continue;
                } else if in_string && ch == quote_char {
                    in_string = false;
                    result.push(ch);
                    continue;
                }
            }
            
            // Handle comments
            if !in_string {
                if ch == '/' && chars.peek() == Some(&'/') {
                    in_single_line_comment = true;
                    result.push_str(&Self::process_word(&current_word, old_name, new_name));
                    current_word.clear();
                } else if ch == '/' && chars.peek() == Some(&'*') {
                    in_comment = true;
                    result.push_str(&Self::process_word(&current_word, old_name, new_name));
                    current_word.clear();
                } else if in_comment && ch == '*' && chars.peek() == Some(&'/') {
                    in_comment = false;
                    result.push(ch);
                    chars.next(); // consume the '/'
                    result.push('/');
                    continue;
                } else if in_single_line_comment && ch == '\n' {
                    in_single_line_comment = false;
                }
            }
            
            // If we're in a comment or string, just add the character
            if in_comment || in_single_line_comment || in_string {
                result.push(ch);
                continue;
            }
            
            // Handle word boundaries for normal code
            if ch.is_alphanumeric() || ch == '_' {
                current_word.push(ch);
            } else {
                result.push_str(&Self::process_word(&current_word, old_name, new_name));
                current_word.clear();
                result.push(ch);
            }
        }
        
        // Process any remaining word
        result.push_str(&Self::process_word(&current_word, old_name, new_name));
        
        result
    }
    
    /// Process a word and replace if it matches the old name
    fn process_word(word: &str, old_name: &str, new_name: &str) -> String {
        if word == old_name {
            new_name.to_string()
        } else {
            word.to_string()
        }
    }

    /// Preview fixes without applying them
    pub fn preview_fixes(lint_result: &LintResult) -> Vec<FixPreview> {
        lint_result
            .violations
            .iter()
            .filter_map(|violation| {
                convert_case(
                    &violation.table_name,
                    violation.current_case,
                    violation.expected_case,
                )
                .ok()
                .map(|fixed_name| FixPreview {
                    original: violation.table_name.clone(),
                    fixed: fixed_name,
                    line: violation.line,
                })
            })
            .collect()
    }
}

/// Preview of a fix
#[derive(Debug, Clone)]
pub struct FixPreview {
    pub original: String,
    pub fixed: String,
    pub line: usize,
}