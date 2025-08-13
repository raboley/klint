//! Lexer for tokenizing KQL statements

use anyhow::Result;

/// Token types in KQL
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // KQL Management Commands
    Dot,                    // .
    Create,                 // create  
    Table,                  // table
    With,                   // with
    BasedOn,               // based-on
    IfNotExists,           // ifnotexists
    
    // Identifiers and literals
    Identifier(String),
    StringLiteral(String),
    
    // Operators and punctuation
    LeftParen,             // (
    RightParen,            // )
    Comma,                 // ,
    Colon,                 // :
    Equals,                // =
    
    // Special
    Whitespace,
    Newline,
    Comment(String),       // // comment
    Eof,
}

/// Token with position information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub lexeme: String,
}

/// Lexer for KQL
pub struct Lexer {
    input: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer with the given input
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the input
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            if let Some(token) = self.next_token()? {
                if !matches!(token.token_type, TokenType::Whitespace | TokenType::Comment(_)) {
                    tokens.push(token);
                }
            }
        }
        
        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            column: self.column,
            lexeme: String::new(),
        });
        
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        // TODO: Implement tokenization logic
        Ok(None)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}