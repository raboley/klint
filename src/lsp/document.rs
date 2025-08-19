//! Document management for LSP
//!
//! Handles document lifecycle, content synchronization, and provides
//! access to document text for analysis.

use lsp_types::{Position, Range, TextDocumentContentChangeEvent, Url};
use std::collections::HashMap;

/// Represents a text document managed by the LSP server
#[derive(Debug, Clone)]
pub struct Document {
    /// The document URI
    pub uri: Url,
    /// The document version
    pub version: i32,
    /// The current text content
    pub text: String,
    /// Language identifier (should be "kql" or "kusto")
    pub language_id: String,
}

impl Document {
    /// Create a new document
    pub fn new(uri: Url, language_id: String, version: i32, text: String) -> Self {
        Self {
            uri,
            version,
            text,
            language_id,
        }
    }

    /// Apply content changes to the document
    pub fn apply_changes(&mut self, version: i32, changes: Vec<TextDocumentContentChangeEvent>) {
        self.version = version;
        
        for change in changes {
            if let Some(range) = change.range {
                // Incremental change
                self.apply_incremental_change(range, &change.text);
            } else {
                // Full document change
                self.text = change.text;
            }
        }
    }

    /// Apply an incremental change to the document
    fn apply_incremental_change(&mut self, range: Range, new_text: &str) {
        let start_index = self.position_to_index(range.start);
        let end_index = self.position_to_index(range.end);
        
        let mut content = self.text.chars().collect::<Vec<_>>();
        content.splice(start_index..end_index, new_text.chars());
        self.text = content.into_iter().collect();
    }

    /// Convert a Position to a character index in the document
    fn position_to_index(&self, position: Position) -> usize {
        let mut index = 0;
        let mut line = 0;
        let mut character = 0;
        
        for ch in self.text.chars() {
            if line == position.line && character == position.character {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }
            index += ch.len_utf8();
        }
        
        index.min(self.text.len())
    }

    /// Convert a byte offset to a Position
    pub fn byte_offset_to_position(&self, offset: usize) -> Position {
        let mut line = 0;
        let mut character = 0;
        let mut current_offset = 0;
        
        for ch in self.text.chars() {
            if current_offset >= offset {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }
            current_offset += ch.len_utf8();
        }
        
        Position::new(line, character)
    }

    /// Get a Range from byte offsets
    pub fn byte_range_to_lsp_range(&self, start: usize, end: usize) -> Range {
        Range::new(
            self.byte_offset_to_position(start),
            self.byte_offset_to_position(end),
        )
    }
}

/// Manages all documents in the LSP workspace
#[derive(Debug, Default)]
pub struct DocumentManager {
    documents: HashMap<Url, Document>,
}

impl DocumentManager {
    /// Create a new document manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Open a document
    pub fn open(&mut self, uri: Url, language_id: String, version: i32, text: String) {
        let document = Document::new(uri.clone(), language_id, version, text);
        self.documents.insert(uri, document);
    }

    /// Close a document
    pub fn close(&mut self, uri: &Url) {
        self.documents.remove(uri);
    }

    /// Update a document with changes
    pub fn update(&mut self, uri: &Url, version: i32, changes: Vec<TextDocumentContentChangeEvent>) {
        if let Some(document) = self.documents.get_mut(uri) {
            document.apply_changes(version, changes);
        }
    }

    /// Get a document by URI
    pub fn get(&self, uri: &Url) -> Option<&Document> {
        self.documents.get(uri)
    }

    /// Get all document URIs
    pub fn uris(&self) -> impl Iterator<Item = &Url> {
        self.documents.keys()
    }
}