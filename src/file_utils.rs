//! File utility functions for KQL file detection and handling

use std::path::Path;
use lsp_types::Url;

/// Check if a file is a KQL file based on extension or language ID
///
/// # Arguments
///
/// * `path` - File path to check
/// * `language_id` - Optional language identifier (for LSP documents)
///
/// # Returns
///
/// True if the file is recognized as a KQL file
pub fn is_kql_file(path: &Path, language_id: Option<&str>) -> bool {
    // Check language ID first (for LSP)
    if let Some(lang) = language_id {
        if matches!(lang, "kql" | "kusto") {
            return true;
        }
    }
    
    // Check file extension
    if let Some(extension) = path.extension() {
        matches!(extension.to_str(), Some("kql") | Some("kusto"))
    } else {
        false
    }
}

/// Check if a URI is a KQL file (for LSP documents)
///
/// # Arguments
///
/// * `uri` - Document URI
/// * `language_id` - Optional language identifier
///
/// # Returns
///
/// True if the document is recognized as a KQL file
pub fn is_kql_document(uri: &Url, language_id: Option<&str>) -> bool {
    // Check language ID first
    if let Some(lang) = language_id {
        if matches!(lang, "kql" | "kusto") {
            return true;
        }
    }
    
    // Check URI path extension
    let path = uri.path();
    path.ends_with(".kql") || path.ends_with(".kusto")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_is_kql_file_by_extension() {
        let kql_file = PathBuf::from("test.kql");
        let kusto_file = PathBuf::from("test.kusto");
        let non_kql_file = PathBuf::from("test.sql");
        
        assert!(is_kql_file(&kql_file, None));
        assert!(is_kql_file(&kusto_file, None));
        assert!(!is_kql_file(&non_kql_file, None));
    }
    
    #[test]
    fn test_is_kql_file_by_language_id() {
        let any_file = PathBuf::from("test.txt");
        
        assert!(is_kql_file(&any_file, Some("kql")));
        assert!(is_kql_file(&any_file, Some("kusto")));
        assert!(!is_kql_file(&any_file, Some("sql")));
    }
    
    #[test]
    fn test_is_kql_document() {
        let kql_uri = Url::parse("file:///test.kql").unwrap();
        let kusto_uri = Url::parse("file:///test.kusto").unwrap();
        let sql_uri = Url::parse("file:///test.sql").unwrap();
        
        assert!(is_kql_document(&kql_uri, None));
        assert!(is_kql_document(&kusto_uri, None));
        assert!(!is_kql_document(&sql_uri, None));
        
        // Language ID should override extension
        assert!(is_kql_document(&sql_uri, Some("kql")));
    }
}