//! Language Server Protocol implementation for KQL linter
//!
//! This module provides LSP functionality to enable real-time diagnostics
//! for KQL table naming conventions in code editors.

pub mod diagnostics;
pub mod document;
pub mod server;
pub mod tree_sitter_integration;

pub use server::KlintLanguageServer;