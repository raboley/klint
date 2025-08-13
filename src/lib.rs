//! KQL Linter Library
//!
//! This library provides the core functionality for linting KQL files
//! and enforcing table naming conventions.

pub mod case_detector;
pub mod config_template;
pub mod configuration;
pub mod disable_comments;
pub mod error;
pub mod file_processor;
pub mod linter;
pub mod output;
pub mod parser;

pub use case_detector::{convert_case, detect_case, NamingCase};
pub use configuration::{Config, ConfigBuilder};
pub use error::{KlintError, Result};
pub use linter::fixer::{FixPreview, Fixer};
pub use linter::{LintResult, Linter, Violation};
pub use output::{OutputFormat, OutputFormatter};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
