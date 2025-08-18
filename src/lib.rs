//! KQL Linter Library
//!
//! This library provides the core functionality for linting KQL files
//! and enforcing table naming conventions.
//!
//! ## Features
//!
//! - **CLI Tool**: Command-line linter with check and fix modes
//! - **LSP Server**: Language Server Protocol implementation for real-time diagnostics
//! - **Configurable**: YAML-based configuration for naming conventions and exclusions
//! - **Tree-sitter Integration**: Accurate KQL parsing using tree-sitter grammar
//!
//! ## Usage
//!
//! ### CLI Tool
//! ```bash
//! klint file.kql              # Check for violations
//! klint file.kql --fix        # Fix violations automatically
//! ```
//!
//! ### LSP Server
//! ```bash
//! klint-lsp                   # Start LSP server for editor integration
//! ```
//!
//! ### Library
//! ```rust
//! use klint::{Config, ConfigBuilder, Linter};
//!
//! let config = ConfigBuilder::new()
//!     .table_naming("PascalCase")
//!     .build();
//! let linter = Linter::new(config);
//! // Use linter to check KQL files...
//! ```

pub mod case_detector;
pub mod config_template;
pub mod configuration;
pub mod disable_comments;
pub mod error;
pub mod file_processor;
pub mod linter;
pub mod lsp;
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
