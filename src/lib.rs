//! KQL Linter Library
//! 
//! This library provides the core functionality for linting KQL files
//! and enforcing table naming conventions.

pub mod case_detector;
pub mod config;
pub mod file_processor;
pub mod linter;
pub mod output;
pub mod parser;

pub use case_detector::{NamingCase, detect_case, convert_case};
pub use config::Config;
pub use linter::{Linter, LintResult, Violation};
pub use output::{OutputFormat, OutputFormatter};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for the library
pub type Result<T> = anyhow::Result<T>;