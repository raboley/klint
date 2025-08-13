//! Error handling for the KQL linter
//!
//! This module provides centralized error types using standard Rust error handling.

use std::error::Error;
use std::fmt;
use std::io;

/// The main error type for the KQL linter
#[derive(Debug)]
pub enum KlintError {
    /// I/O related errors
    Io(io::Error),

    /// Configuration errors
    Config { message: String },

    /// Parsing errors
    Parse {
        file: String,
        line: usize,
        message: String,
    },

    /// Case conversion errors
    CaseConversion { message: String },

    /// File processing errors
    FileProcessing {
        file: String,
        source: Box<KlintError>,
    },

    /// Invalid naming case
    InvalidNamingCase {
        case: String,
        valid_cases: String,
    },

    /// Yaml serialization/deserialization errors
    Yaml(serde_yaml::Error),

    /// JSON serialization/deserialization errors
    Json(serde_json::Error),

    /// Pattern matching errors (for glob patterns)
    Pattern(glob::PatternError),

    /// Walk directory errors
    WalkDir(walkdir::Error),
}

impl fmt::Display for KlintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KlintError::Io(err) => write!(f, "I/O error: {}", err),
            KlintError::Config { message } => write!(f, "Configuration error: {}", message),
            KlintError::Parse { file, line, message } => {
                write!(f, "Parse error in {}:{}: {}", file, line, message)
            }
            KlintError::CaseConversion { message } => {
                write!(f, "Case conversion error: {}", message)
            }
            KlintError::FileProcessing { file, source } => {
                write!(f, "File processing error for '{}': {}", file, source)
            }
            KlintError::InvalidNamingCase { case, valid_cases } => {
                write!(
                    f,
                    "Invalid naming case '{}'. Valid options are: {}",
                    case, valid_cases
                )
            }
            KlintError::Yaml(err) => write!(f, "YAML error: {}", err),
            KlintError::Json(err) => write!(f, "JSON error: {}", err),
            KlintError::Pattern(err) => write!(f, "Pattern error: {}", err),
            KlintError::WalkDir(err) => write!(f, "Directory traversal error: {}", err),
        }
    }
}

impl Error for KlintError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            KlintError::Io(err) => Some(err),
            KlintError::Yaml(err) => Some(err),
            KlintError::Json(err) => Some(err),
            KlintError::Pattern(err) => Some(err),
            KlintError::WalkDir(err) => Some(err),
            KlintError::FileProcessing { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl From<io::Error> for KlintError {
    fn from(err: io::Error) -> Self {
        KlintError::Io(err)
    }
}

impl From<serde_yaml::Error> for KlintError {
    fn from(err: serde_yaml::Error) -> Self {
        KlintError::Yaml(err)
    }
}

impl From<serde_json::Error> for KlintError {
    fn from(err: serde_json::Error) -> Self {
        KlintError::Json(err)
    }
}

impl From<glob::PatternError> for KlintError {
    fn from(err: glob::PatternError) -> Self {
        KlintError::Pattern(err)
    }
}

impl From<walkdir::Error> for KlintError {
    fn from(err: walkdir::Error) -> Self {
        KlintError::WalkDir(err)
    }
}

impl KlintError {
    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new parse error
    pub fn parse<S: Into<String>>(file: S, line: usize, message: S) -> Self {
        Self::Parse {
            file: file.into(),
            line,
            message: message.into(),
        }
    }

    /// Create a new case conversion error
    pub fn case_conversion<S: Into<String>>(message: S) -> Self {
        Self::CaseConversion {
            message: message.into(),
        }
    }

    /// Create a new invalid naming case error
    pub fn invalid_naming_case<S: Into<String>>(case: S, valid_cases: &[&str]) -> Self {
        Self::InvalidNamingCase {
            case: case.into(),
            valid_cases: valid_cases.join(", "),
        }
    }

    /// Wrap an error as a file processing error
    pub fn wrap_file_processing<S: Into<String>>(file: S, error: KlintError) -> Self {
        Self::FileProcessing {
            file: file.into(),
            source: Box::new(error),
        }
    }
}

/// Result type alias for the KQL linter
pub type Result<T> = std::result::Result<T, KlintError>;